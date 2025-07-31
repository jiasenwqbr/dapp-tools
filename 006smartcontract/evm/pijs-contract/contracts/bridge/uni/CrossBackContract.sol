// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/extensions/ERC20BurnableUpgradeable.sol";

contract CrossBackContract is AccessControlUpgradeable, OwnableUpgradeable, ReentrancyGuardUpgradeable, UUPSUpgradeable,PausableUpgradeable{
    using SafeERC20Upgradeable for ERC20BurnableUpgradeable;
    bytes32 public constant OPERATOR_ROLE = keccak256("OPERATOR_ROLE");
    // 拥有合约升级、参数配置权限。
    bytes32 public constant MANAGE_ROLE = keccak256("MANAGE_ROLE");
    ERC20BurnableUpgradeable public uacToken;
    bytes32 public DOMAIN_SEPARATOR;
    // 签名者地址
    address public signer;

    struct BurnTokenData {
        address caller;
        uint256 amount;
        uint256 orderId;
        uint256 chainId;
    }
    bytes32 private constant PERMIT_BURN_TYPEHASH = keccak256(
        abi.encodePacked("Permit(address caller,uint256 amount,uint256 orderId,uint256 chainId)")
    );

    event TokenBurned(address indexed from, uint256 amount, uint256 orderId);

    function _authorizeUpgrade(
        address newImplementation
    ) internal virtual override onlyRole(MANAGE_ROLE) {}

    function initialize(address uacToken_, address operator_,address _signer,address admin) public initializer {
        require(uacToken_ != address(0), "CrossBackContract:UAC token address is zero");
        require(operator_ != address(0), "CrossBackContract:Operator is zero");

        __AccessControl_init();
        __Ownable_init();
        __ReentrancyGuard_init();
        __UUPSUpgradeable_init();
        

        uacToken = ERC20BurnableUpgradeable(uacToken_);

        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(OPERATOR_ROLE, operator_);
        _grantRole(MANAGE_ROLE,admin);
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
                keccak256(bytes("CrossBackContract")),
                keccak256(bytes("1")),
                chainId,
                address(this)
            )
        );
    }

    /// @notice 用户 burn 表示要跨回原链
    function tokenBurned(bytes calldata data) external whenNotPaused nonReentrant {
        BurnTokenData memory burnTokenData = parseBurnTokenData(data);
        require(burnTokenData.amount>0,"CrossBackContract:amount should not be zero");

        require(burnTokenData.amount<=uacToken.balanceOf(burnTokenData.caller),"CrossBackContract:amount should less than the balance");
        uacToken.burn(burnTokenData.amount);

        emit TokenBurned(msg.sender, burnTokenData.amount, burnTokenData.orderId);
    }

    function parseBurnTokenData(bytes calldata data) internal view returns (BurnTokenData memory) {
        (
            address caller,
            uint256 amount,
            uint256 orderId,
            uint256 chainId,
            bytes memory signature
        ) =  abi.decode(
            data,
            (
                address,
                uint256,
                uint256,
                uint256,
                bytes
            )
        );
        require(caller == msg.sender, "CrossBackContract: INVALID_USER");
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);

         bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        PERMIT_BURN_TYPEHASH,
                        caller,
                        amount,
                        orderId,
                        chainId
                    )
                )
            )
        );
        require(
            signer == ecrecover(signHash, v, r, s),
            "CrossBackContract: INVALID_REQUEST"
        );

        return BurnTokenData({
            caller:caller,
            amount:amount,
            orderId:orderId,
            chainId:chainId
        });
    }

    function splitSignature(
        bytes memory sig
    ) internal pure returns (uint8, bytes32, bytes32) {
        require(sig.length == 65, "CrossBackContract:Not Invalid Signature Data");
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

    function pause() public onlyRole(DEFAULT_ADMIN_ROLE) {
        _pause();
    }

    function unpause() public onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }


}