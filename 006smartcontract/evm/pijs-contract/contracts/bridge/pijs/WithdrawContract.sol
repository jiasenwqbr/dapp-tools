// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";


contract WithdrawContract is AccessControlUpgradeable, OwnableUpgradeable, ReentrancyGuardUpgradeable, UUPSUpgradeable {
    using SafeERC20Upgradeable for IERC20Upgradeable;

    bytes32 public constant OPERATOR_ROLE = keccak256("OPERATOR_ROLE");
    // 拥有合约升级、参数配置权限。
    bytes32 public constant MANAGE_ROLE = keccak256("MANAGE_ROLE");

    IERC20Upgradeable public uacToken;
    bytes32 public DOMAIN_SEPARATOR;
    // 签名者地址
    address public signer;

    /// 提币事件
    event  WithdrawUAC(address caller,address to, uint256 amount,uint256 orderId);

    function _authorizeUpgrade(
        address newImplementation
    ) internal virtual override onlyRole(MANAGE_ROLE) {}

    function initialize(address uacToken_, address operator_,address _signer,address admin) public initializer {
        require(uacToken_ != address(0), "UAC token address is zero");
        require(operator_ != address(0), "Operator is zero");

        __AccessControl_init();
        __Ownable_init();
        __ReentrancyGuard_init();
        __UUPSUpgradeable_init();
        _grantRole(MANAGE_ROLE, admin);

        uacToken = IERC20Upgradeable(uacToken_);

        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(OPERATOR_ROLE, operator_);
        signer = _signer;

        uint256 chainId;
        assembly {
            chainId := chainid()
        }
        DOMAIN_SEPARATOR = keccak256(
            abi.encode(
                keccak256(
                    "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
                ),
                keccak256(bytes("WithdrawContract")),
                keccak256(bytes("1")),
                chainId,
                address(this)
            )
        );
    }

    struct WithDrawUACData {
        address caller;
        address to;
        uint256 amount;
        uint256 orderId;
        uint256 chainId;
    }

    bytes32 private constant  PERMIT_WITHDRAWUAC_TYPEHASH = keccak256(
        abi.encodePacked(
            "Permit(address caller,address to,uint256 amount,uint256 orderId,uint256 chainId)"
        )
    );

    /// @notice 管理员发起提币，提币代币发送给用户
    function withdrawUAC(bytes calldata data) external nonReentrant onlyRole(OPERATOR_ROLE) {
        
        WithDrawUACData memory withDrawData = parseWithDrawUACData(data);
        require(withDrawData.to != address(0), "Invalid to address");
        require(withDrawData.amount > 0, "Amount must be > 0");

        uacToken.safeTransfer(withDrawData.to, withDrawData.amount);

        emit WithdrawUAC(withDrawData.caller,withDrawData.to, withDrawData.amount,withDrawData.orderId);
    }

    function parseWithDrawUACData(bytes calldata data) internal view returns (WithDrawUACData memory) {
        (
            address caller,
            address to,
            uint256 amount,
            uint256 orderId,
            uint256 chainId,
            bytes memory signature
        ) =  abi.decode(
            data,
            (
                address,
                address,
                uint256,
                uint256,
                uint256,
                bytes
            )
        );

        require(caller == msg.sender, "WithdrawContract: INVALID_USER");
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);

         bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        PERMIT_WITHDRAWUAC_TYPEHASH,
                        caller,
                        to,
                        amount,
                        orderId,
                        chainId
                    )
                )
            )
        );
        require(
            signer == ecrecover(signHash, v, r, s),
            "WithdrawContract: INVALID_REQUEST"
        );

        return WithDrawUACData({
            caller:caller,
            to:to,
            amount:amount,
            orderId:orderId,
            chainId:chainId
        });
    }
    function splitSignature(
        bytes memory sig
    ) internal pure returns (uint8, bytes32, bytes32) {
        require(sig.length == 65, "WithdrawContract:Not Invalid Signature Data");
        bytes32 r;
        bytes32 s;
        uint8 v;
        assembly {
            r := mload(add(sig, 32))
            s := mload(add(sig, 64))
            v := byte(0, mload(add(sig, 96)))
        }
        return (v, r, s);
    }

    function setUACToken(address newUAC) external onlyOwner {
        require(newUAC != address(0), "Invalid token address");
        uacToken = IERC20Upgradeable(newUAC);
    }

}
