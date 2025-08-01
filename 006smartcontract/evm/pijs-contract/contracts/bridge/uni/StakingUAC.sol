// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts-upgradeable/token/ERC20/IERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/AccessControlEnumerableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";


contract StakingUAC is
    Initializable,
    AccessControlEnumerableUpgradeable,
    ReentrancyGuardUpgradeable,
    UUPSUpgradeable
{
    using SafeERC20Upgradeable for IERC20Upgradeable;

    bytes32 public constant MANAGE_ROLE = keccak256("MANAGE_ROLE");

    bytes32 public constant OPERATE_ROLE = keccak256("OPERATE_ROLE");
    bool private funcSwitch;

    IERC20Upgradeable private uacToken;
    // 签名地址
    address public signer;
    bytes32 public DOMAIN_SEPARATOR;

    // staking
    struct Order {
        uint256 orderId; // 订单号
        uint256 userId; // 用户ID
        uint256 amount;
        uint24 balanceSource; // 0 crosschain 1 direct
        uint256 endTimestamp; // 订单到期时间
        uint256 startTimestamp; // 订单开始时间(可以由区块决定)
        uint8 renewable; // 是否允许续期
        uint8 status; // 0 -staking; 1- unstaking  状态
        uint256 renewTime; // 续期时间
    }
    struct RenewOrder {
        uint256 orderId;
        uint256 renewTime;
        uint256 blockTime;
    }
    mapping(address => mapping(uint256 => Order)) public userOrders;
    mapping(address => uint256[]) userOrderIds;
    mapping(address => RenewOrder[]) public userRenewOrders;

    bytes32 private constant PERMIT_TYPEHASH = keccak256(
            abi.encodePacked(
                "Permit(uint256 productId,uint256 orderId,uint256 userId,uint256 phase,uint256 purchaseNum,uint256 payNum,uint256 anchorCoinNum,string anchorCoin)"
            )
    );
    

    function initialize(IERC20Upgradeable _uacToken,address _signer) public initializer {
        __AccessControlEnumerable_init();
        __ReentrancyGuard_init();
        __UUPSUpgradeable_init();

        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(MANAGE_ROLE, msg.sender);

        uacToken = _uacToken;
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
                    keccak256(bytes("StakingUAC")),
                    keccak256(bytes("1")),
                    chainId,
                    address(this)
                )
            );
    }

    function _authorizeUpgrade(
        address newImplementation
    ) internal override onlyRole(MANAGE_ROLE) {}

    function balance(address token) public view returns (uint256) {
        if (token == address(0)) {
            return address(this).balance;
        }
        return IERC20Upgradeable(token).balanceOf(address(this));
    }

    function setFuncSwith(bool _funcSwitch) public onlyRole(MANAGE_ROLE) {
        funcSwitch = _funcSwitch;
    }

    function stakeUAC(bytes memory data)  public payable nonReentrant{
        Order memory order = parseOrder(data);
        require(userOrders[msg.sender][order.orderId].orderId == 0, "PIJSOrder: ORDER_EXISTS");
        // 判断balanceSource : "crosschain"（使用跨链余额）或 "direct"（直接从钱包扣款）
        


    }

    function parseOrder(bytes memory data) internal view returns(Order memory) {
        (
            uint256 orderId,
            uint256 userId,
            uint256 amount,
            uint24 balanceSource,
            uint256 endTimestamp,
            uint256 startTimestamp,
            uint8 renewable,
            bytes memory signature
        ) = abi.decode(
            data,
            (
                uint256,
                uint256,
                uint256,
                uint24,
                uint256,
                uint256,
                uint8,
                bytes
            )
        );
        require(endTimestamp > block.timestamp, "StakingUAC: order invalid");
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);
         bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        PERMIT_TYPEHASH,
                        orderId,
                        userId,
                        amount,
                        balanceSource,
                        endTimestamp,
                        startTimestamp,
                        renewable,
                        signature
                    )
                )
            )
        );
        require(signer == ecrecover(signHash, v, r, s),"StakingUAC:INVALID_REQUEST");
        
        return Order({
            orderId:orderId,
            userId: userId,
            amount: amount,
            balanceSource: balanceSource,
            endTimestamp: endTimestamp,
            startTimestamp: startTimestamp,
            renewable: renewable,
            status: 0,
            renewTime:0
        });
        

    }

    function splitSignature(
        bytes memory sig
    ) internal pure returns (uint8 v, bytes32 r, bytes32 s) {
        require(sig.length == 65, "EIP712: invalid signature length");
        assembly {
            r := mload(add(sig, 32))
            s := mload(add(sig, 64))
            v := byte(0, mload(add(sig, 96)))
        }
    }

     


}