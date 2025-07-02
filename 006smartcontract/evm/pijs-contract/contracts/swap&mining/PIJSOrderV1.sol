// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts-upgradeable/token/ERC20/IERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";

import "@openzeppelin/contracts-upgradeable/access/AccessControlEnumerableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import "../utils/EIP712Verifier.sol";
import "../utils/SafeMath.sol";

struct Order {
    uint256 productId; // 购买的产品ID
    uint256 orderId; // 订单号
    uint256 userId; // 用户ID
    uint256 purchaseNum; //购买的份数
    uint256 payNum; // 支付的PIJS
    uint256 endTimestamp; // 订单到期时间
    uint256 startTimestamp; // 订单开始时间(可以由区块决定)
    uint256 userPurchaseLimit; // 指定phase 指定productId 指定用户的购买限制
    uint256 productPurchaseLimit; // 指定phase 指定productId 指定用户的购买限制
    uint256 phase; // 活动期
    uint256 renewable; // 是否允许续期
    uint256 anchorCoinNum; // 锚定货币数量
    string anchorCoin; // 锚定货币
    uint256 status; // 0 -staking; 1- unstaking  状态
    uint256 renewTime; // 续期时间
}

struct RenewOrder {
    uint256 orderId;
    uint256 renewTime;
    uint256 blockTime;
}

contract PIJSOrderV1 is
    Initializable,
    AccessControlEnumerableUpgradeable,
    ReentrancyGuardUpgradeable,
    UUPSUpgradeable
{
    using SafeMath for uint;
    // MANAGE_ROLE：主要用于升级合约、切换开关等敏感操作
    bytes32 public constant MANAGE_ROLE = keccak256("MANAGE_ROLE");
    // OPERATE_ROLE：可扩展为前台调用权限、运营人员权限等
    bytes32 public constant OPERATE_ROLE = keccak256("OPERATE_ROLE");

    // data
    // user orders
    mapping(address => mapping(uint256 => Order)) public userOrders;
    mapping(address => uint256[]) userOrderIds;
    mapping(address => RenewOrder[]) public userRenewOrders;
    // Phase -> product -> orderCount
    mapping(uint256 => mapping(uint256 => uint256))
        public orderSumByProductByPhase;
    // Phase -> user -> orderCount
    mapping(uint256 => mapping(address => uint256))
        public orderSumByUserByPhase;

    // 签名地址
    address public signer;

    bool private funcSwitch;

    bytes32 public DOMAIN_SEPARATOR;
    bytes32 private constant PERMIT_TYPEHASH =
        keccak256(
            abi.encodePacked(
                "Permit(uint256 productId,uint256 orderId,uint256 userId,uint256 phase, uint256 purchaseNum, uint256 payNum,uint256 anchorCoinNum,bytes32 anchorCoin)"
            )
        );
    event MakeOrder(
        address caller,
        uint256 productId,
        uint256 orderId,
        uint256 userId,
        uint256 purchaseNum,
        uint256 payNum,
        uint256 endTimestamp,
        uint256 startTimestamp,
        uint256 userPurchaseLimit,
        uint256 productPurchaseLimit,
        uint256 phase,
        uint256 renewable,
        uint256 anchorCoinNum,
        string anchorCoin
    );
    event ReNewOrder(address caller, uint256 orderId, uint256 renewTime);

    event BetBackOrder(address caller, uint256 orderId, uint256 amount);

    function _authorizeUpgrade(
        address newImplementation
    ) internal override onlyRole(MANAGE_ROLE) {}

    function initialize(address _signer) public initializer {
        // 初始化 UUPS、权限模块、防重入模块
        __AccessControlEnumerable_init();
        __ReentrancyGuard_init();
        __UUPSUpgradeable_init();
        // 设置管理员权限
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(MANAGE_ROLE, msg.sender);

        signer = _signer;
        uint256 chainId;
        assembly {
            chainId := chainid()
        }
        DOMAIN_SEPARATOR = EIP712Verifier.getDomainSeparator(
            "PIJSOrder",
            "1",
            block.chainid,
            address(this)
        );
    }

    receive() external payable {}

    function balance(address token) public view returns (uint256) {
        if (token == address(0)) {
            return address(this).balance;
        }
        return IERC20Upgradeable(token).balanceOf(address(this));
    }

    function setFuncSwith(bool _funcSwitch) public onlyRole(MANAGE_ROLE) {
        funcSwitch = _funcSwitch;
    }

    // Make order
    function makeOrder(bytes memory data) public payable nonReentrant {
        (
            uint256 productId,
            uint256 orderId,
            uint256 userId,
            uint256 purchaseNum,
            uint256 payNum,
            uint256 endTimestamp,
            uint256 startTimestamp,
            uint256 userPurchaseLimit,
            uint256 productPurchaseLimit,
            uint256 phase,
            uint256 renewable,
            uint256 anchorCoinNum,
            bytes32 anchorCoin,
            bytes memory signature
        ) = abi.decode(
                data,
                (
                    uint256,
                    uint256,
                    uint256,
                    uint256,
                    uint256,
                    uint256,
                    uint256,
                    uint256,
                    uint256,
                    uint256,
                    uint256,
                    uint256,
                    bytes32,
                    bytes
                )
            );
        // validite para
        require(msg.value >= payNum, "PIJSOrder: invalid payNum");
        require(endTimestamp > block.timestamp, "PIJSOrder: order invalid");
        // validite singer
        // verify(bytes memory sig,uint256 productId,uint256 orderId,uint256 userId,uint256 phase, uint256 purchaseNum, uint256 payNum,uint256 anchorCoinNum,string memory anchorCoin)
        require(
            verify(
                signature,
                productId,
                orderId,
                userId,
                phase,
                purchaseNum,
                payNum,
                anchorCoinNum,
                anchorCoin
            ),
            "ERROR:INVALID_REQUEST"
        );
        // 指定phase 指定productId 已有productId订单数量<= productPurchaselimit
        require(
            orderSumByProductByPhase[phase][productId].add(purchaseNum) <=
                productPurchaseLimit,
            "PIJSOrder: The product quantity exceeds the limit"
        );
        // 当前用户已有productId订单数量+本订单 purchaseNum<= userPurchaseLimit
        require(
            orderSumByUserByPhase[phase][msg.sender].add(purchaseNum) <=
                userPurchaseLimit,
            "PIJSOrder: The order quantity exceeds the limit"
        );
        require(userOrders[msg.sender][orderId].orderId == 0, "PIJSOrder: ORDER_EXISTS");
        // 更新订单
        userOrders[msg.sender][orderId] = Order({
            productId: productId,
            orderId: orderId,
            userId: userId,
            purchaseNum: purchaseNum,
            payNum: payNum,
            endTimestamp: endTimestamp,
            startTimestamp: startTimestamp,
            userPurchaseLimit: userPurchaseLimit,
            productPurchaseLimit: productPurchaseLimit,
            phase: phase,
            renewable: renewable,
            anchorCoinNum: anchorCoinNum,
            anchorCoin: EIP712Verifier.bytes32ToString(anchorCoin),
            status: 0,
            renewTime: 0
        });
       
        userOrderIds[msg.sender].push(orderId);
        // 更新 orderSumByProductByPhase
        orderSumByProductByPhase[phase][productId] = orderSumByProductByPhase[
            phase
        ][productId].add(purchaseNum);
        // 更新 orderSumByUserByPhase
        orderSumByUserByPhase[phase][msg.sender] = orderSumByUserByPhase[phase][
            msg.sender
        ].add(purchaseNum);
        // event
        emit MakeOrder(
            msg.sender,
            productId,
            orderId,
            userId,
            purchaseNum,
            payNum,
            endTimestamp,
            startTimestamp,
            userPurchaseLimit,
            productPurchaseLimit,
            phase,
            renewable,
            anchorCoinNum,
            EIP712Verifier.bytes32ToString(anchorCoin)
        );
    }

    function reNewOrder(
        uint256 orderId,
        uint256 newEndTimestamp
    ) public nonReentrant {
        // has order
        bool _hasOrder = hasOrder(msg.sender, orderId);
        require(_hasOrder, "PIJSOrder: no order exist");
        Order memory order = userOrders[msg.sender][orderId];
        require(order.status == 0, "PIJSOrder: order is invalid");
        require(order.renewable == 1, "PIJSOrder: order is not renewable");
        if (order.renewTime > 0) {
            require(
                newEndTimestamp > block.timestamp &&
                    newEndTimestamp > order.renewTime,
                "PIJSOrder: newEndTimestamp is invalid"
            );
        } else {
            require(
                newEndTimestamp > block.timestamp &&
                    newEndTimestamp > order.endTimestamp,
                "PIJSOrder: newEndTimestamp is invalid"
            );
        }
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

    function betBackOrder(uint256 orderId) public nonReentrant {
        // has order
        bool _hasOrder = hasOrder(msg.sender, orderId);
        require(_hasOrder, "PIJSOrder: no order exist");
        Order memory order = userOrders[msg.sender][orderId];
        require(order.status == 0, "PIJSOrder: order is invalid");
        if (order.renewTime > 0) {
            require(
                order.renewTime <= block.timestamp,
                "PIJSOrder: newEndTimestamp is invalid"
            );
        } else {
            require(
                order.endTimestamp <= block.timestamp,
                "PIJSOrder: newEndTimestamp is invalid"
            );
        }
        userOrders[msg.sender][orderId].status = 1;
        payable(msg.sender).transfer(order.payNum);

        emit BetBackOrder(msg.sender, orderId, order.payNum);
    }

    function hasOrder(
        address user,
        uint256 orderId
    ) public view returns (bool found) {
        Order storage order = userOrders[user][orderId];
        return order.orderId != 0; // 注意：假设 0 是无效的订单号
    }

    function verify(
        bytes memory sig,
        uint256 productId,
        uint256 orderId,
        uint256 userId,
        uint256 phase,
        uint256 purchaseNum,
        uint256 payNum,
        uint256 anchorCoinNum,
        bytes32 anchorCoin
    ) internal view returns (bool) {
        bytes32[] memory values = new bytes32[](8);
        values[0] = bytes32(productId);
        values[1] = bytes32(orderId);
        values[2] = bytes32(userId);
        values[3] = bytes32(phase);
        values[4] = bytes32(purchaseNum);
        values[5] = bytes32(payNum);
        values[6] = bytes32(anchorCoinNum);
        values[7] = anchorCoin;
        return
            EIP712Verifier.verifySignature(
                DOMAIN_SEPARATOR,
                PERMIT_TYPEHASH,
                values,
                sig,
                signer
            );
    }

    // search
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

    // search user renewOrder
    function getUserReNewOrder(
        address user
    ) public view returns (RenewOrder[] memory) {
        return userRenewOrders[user];
    }

    // serch
    function getOrderSumsByProductAndPhase(
        uint256 phase,
        uint256[] calldata productIds
    ) public view returns (uint256[] memory sums) {
        uint256 len = productIds.length;
        sums = new uint256[](len);
        for (uint256 i = 0; i < len; i++) {
            sums[i] = orderSumByProductByPhase[phase][productIds[i]];
        }
    }

    function getOrderSumByUserByPhase(
        uint256 phase,
        address[] calldata userIds
    ) public view returns (uint256[] memory sums) {
        uint256 len = userIds.length;
        sums = new uint256[](len);
        for (uint256 i = 0; i < len; i++) {
            sums[i] = orderSumByUserByPhase[phase][userIds[i]];
        }
    }
    // 仅限拥有 MANAGE_ROLE 的管理员调用,处理溢出资金
    function withdraw(address to, uint256 amount) external onlyRole(MANAGE_ROLE) nonReentrant {
        require(to != address(0), "Withdraw: invalid recipient");
        require(amount > 0, "Withdraw: amount must be greater than zero");
        require(address(this).balance >= amount, "Withdraw: insufficient balance");
        (bool success, ) = to.call{value: amount}("");
        require(success, "Withdraw: ETH transfer failed");
    }
}
