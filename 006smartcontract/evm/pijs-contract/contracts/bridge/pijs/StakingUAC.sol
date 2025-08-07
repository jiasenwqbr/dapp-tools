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
    IERC20Upgradeable private lpToken;
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
        uint8 isActive;//是否激活 0 未激活 1 已激活
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
                "Permit(uint256 orderId,uint256 userId,uint256 amount,uint24 balanceSource,uint256 endTimestamp,uint256 startTimestamp,uint8 renewable)"
            )
    );
    bytes32 private constant RENEW_TYPEHASH =
        keccak256(
            abi.encodePacked(
                "Permit(uint256 orderId,uint256 newEndTimestamp)"
            )
        );
    bytes32 private constant BEBACK_TYPEHASH =
        keccak256(
            abi.encodePacked(
                "Permit(uint256 orderId)"
            )
        );
    bytes32 private constant STAKELP_TYPEHASH =
        keccak256(
            abi.encodePacked(
                "Permit(address caller,uint256 amount,uint256 stakingId)"
            )
    );

    

    event StakeUAC(address caller,address contractAddress,uint256 orderId,uint256 userId,uint256 amount,uint24 balanceSource,uint256 startTimestamp,uint256 endTimestamp,uint8 renewable,uint8 status);
    event ReNewOrder(address caller, uint256 orderId, uint256 renewTime);
    event BetBackOrder(address caller, uint256 orderId, uint256 amount);
    event StakeLP(address caller,uint256 amount,uint256 stakingId);

    function initialize(address _uacToken,address _LPToken,address _signer) public initializer {
        __AccessControlEnumerable_init();
        __ReentrancyGuard_init();
        __UUPSUpgradeable_init();

        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(MANAGE_ROLE, msg.sender);

        uacToken = IERC20Upgradeable(_uacToken);
        signer = _signer;
        lpToken = IERC20Upgradeable(_LPToken);

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
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
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
        require(userOrders[msg.sender][order.orderId].orderId == 0, "StakingUAC: ORDER_EXISTS");
        // 判断balanceSource : "crosschain"（使用跨链余额）或 "direct"（直接从钱包扣款）
        require(order.balanceSource == 0 || order.balanceSource == 1,"StakingUAC:balanceSource is 0 or 1");
        require(order.amount>0,"StakingUAC:staking amount > 0");
        if (order.balanceSource == 1) {
            IERC20Upgradeable(uacToken).safeTransferFrom(
                msg.sender,
                address(this),
                order.amount
            );
            userOrders[msg.sender][order.orderId] = order;
            userOrderIds[msg.sender].push(order.orderId);
            emit StakeUAC(msg.sender,address(this),order.orderId,order.userId,order.amount,order.balanceSource,order.startTimestamp,order.endTimestamp,order.renewable,order.status);
        }
        if (order.balanceSource == 0) {
            userOrders[msg.sender][order.orderId] = order;
            userOrderIds[msg.sender].push(order.orderId);
            emit StakeUAC(msg.sender,address(this),order.orderId,order.userId,order.amount,order.balanceSource,order.startTimestamp,order.endTimestamp,order.renewable,order.status);
        }
    }

     function stakeLP(bytes memory data) public payable nonReentrant {
         (
            address caller,
            uint256 amount,
            uint256 stakingId,
            bytes memory signature
        ) = abi.decode(
            data,(address,uint256,uint256,bytes)
        );

        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);
        bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        STAKELP_TYPEHASH,
                        caller,
                        amount,
                        stakingId
                    )
                )
            )
        );
         require(signer == ecrecover(signHash, v, r, s),"INVALID_REQUEST");



        require(amount > 0, "LPStaking: amount error");
        require(caller == msg.sender,"LPStaking: caller error");
        require(userOrders[msg.sender][stakingId].orderId != 0, "LPStaking: UAC order is not exist");
        lpToken.safeTransferFrom(msg.sender, address(this), amount);
        // 激活订单
        userOrders[msg.sender][stakingId].isActive = 1;

        emit StakeLP(caller,amount,stakingId);


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
                        renewable
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
            renewTime:0,
            isActive:0
        });
        

    }

    function splitSignature(
        bytes memory sig
    ) internal pure returns (uint8, bytes32, bytes32) {
        require(sig.length == 65, "PIJSBridgeTarget:Not Invalid Signature Data");
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


    // renew  oder
    function reNewOrder( bytes memory data) public nonReentrant {
        (
            uint256 orderId,
            uint256 newEndTimestamp,
            bytes memory signature
        ) = abi.decode(
                data,
                (
                    uint256,
                    uint256,
                     bytes
                )
            );
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);
        bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        RENEW_TYPEHASH,
                        orderId,
                        newEndTimestamp
                    )
                )
            )
        );

        require(signer == ecrecover(signHash, v, r, s),"StakingUAC:INVALID_REQUEST");
        require( userOrders[msg.sender][orderId].orderId!=0, "StakingUAC: no order exist");
        Order memory order = userOrders[msg.sender][orderId];
        require(order.status == 0, "PIJSOrder: order is invalid");
        require(order.renewable == 1, "PIJSOrder: order is not renewable");
        userOrders[msg.sender][orderId].renewTime = newEndTimestamp;
        userRenewOrders[msg.sender].push(
            RenewOrder({
                orderId: order.orderId,
                renewTime: newEndTimestamp,
                blockTime: block.timestamp
            })
        );

        emit ReNewOrder(msg.sender, orderId, newEndTimestamp);
    }

    function betBackOrder( bytes memory data) public nonReentrant {
        (
            uint256 orderId,
            bytes memory signature
        ) = abi.decode(
                data,
                (
                    uint256,
                     bytes
                )
            );
        
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);
        bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        BEBACK_TYPEHASH,
                        orderId
                    )
                )
            )
        );
        require(signer == ecrecover(signHash, v, r, s),"StakingUAC:INVALID_REQUEST");

        // has order
        require( userOrders[msg.sender][orderId].orderId!=0, "StakingUAC: no order exist");
        Order memory order = userOrders[msg.sender][orderId];
        require(order.status == 0, "PIJSOrder: order is invalid");


        // if (order.renewTime > 0) {
        //     require(
        //         order.renewTime <= block.timestamp,
        //         "PIJSOrder: newEndTimestamp is invalid"
        //     );
        // } else {
        //     require(
        //         order.endTimestamp <= block.timestamp,
        //         "PIJSOrder: newEndTimestamp is invalid"
        //     );
        // }


        userOrders[msg.sender][orderId].status = 1;
        require(order.amount>0);

        // payable(msg.sender).transfer(order.payNum);
        (bool success, ) = payable(msg.sender).call{value: order.amount}("");
        require(success, "Transfer failed");

        emit BetBackOrder(msg.sender, orderId, order.amount);
    }

    

    // search user renewOrder
    function getUserReNewOrder(
        address user
    ) public view returns (RenewOrder[] memory) {
        return userRenewOrders[user];
    }

    // search orders by user address
    function getUserOrder(address user) public view returns (Order[] memory) {
        uint256[] storage orderIds = userOrderIds[user];
        uint256 len = orderIds.length;
        Order[] memory orders = new Order[](len);
        for (uint256 i = 0; i < len; i++) {
            orders[i] = userOrders[user][orderIds[i]];
        }
        return orders;
    }

     // search orderCount by user address
    function getUserOrderCount(address user) public view returns (uint256) {
        return userOrderIds[user].length;
    }

    
}