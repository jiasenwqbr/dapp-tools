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
    UUPSUpgradeable,
    EIP712Verifier
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
                "Permit(uint256 productId,uint256 orderId,uint256 userId,uint256 phase,uint256 purchaseNum,uint256 payNum,uint256 anchorCoinNum,string anchorCoin)"
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
        uint256 anchorCoinNum
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
        DOMAIN_SEPARATOR = getDomainSeparator(
            "PIJSOrder",
            "1",
            block.chainid,
            address(this)
        );
    }

    receive() external payable {}

    function balance() public view returns (uint256) {
        return address(this).balance;
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
            uint256 userProductPurchaseLimit,
            uint256 isAddPurchaseSum,
            uint256 phase,
            uint256 renewable,
            uint256 anchorCoinNum,
            string memory anchorCoin,
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
                    uint256,
                    uint256,
                    string,
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

        // userProductPurchaseLimit
        (uint256 productCount) = getUserProductPurchaseNum(msg.sender,productId);
        require(
            productCount.add(purchaseNum)  <=
                userProductPurchaseLimit,
            "PIJSOrder: The order quantity exceeds the user product purchase limit"
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
            anchorCoin: anchorCoin,
            status: 0,
            renewTime: 0
        });
       
        userOrderIds[msg.sender].push(orderId);
        if (isAddPurchaseSum==1){
            // 更新 orderSumByProductByPhase
            orderSumByProductByPhase[phase][productId] = orderSumByProductByPhase[
                phase
            ][productId].add(purchaseNum);
            // 更新 orderSumByUserByPhase
            orderSumByUserByPhase[phase][msg.sender] = orderSumByUserByPhase[phase][
                msg.sender
            ].add(purchaseNum);
        }
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
            anchorCoinNum);
    }


    function getUserProductPurchaseNum(address userAddress,uint256 productId) internal view returns(uint256 productCount){
        
        uint256[] storage userOrderIds_  = userOrderIds[userAddress];
            for (uint256 i = 0; i < userOrderIds_.length; i++) {
                Order storage order = userOrders[userAddress][userOrderIds_[i]];
                if (order.productId == productId) {
                    productCount += order.purchaseNum;
                }
            }
        
    }

    function reNewOrder(
        bytes memory data
    ) public nonReentrant {
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
        require(
            verifyRenew(
                signature,
                orderId,
                newEndTimestamp
            ),
            "ERROR:INVALID_REQUEST"
        );

        // has order
        bool _hasOrder = hasOrder(msg.sender, orderId);
        require(_hasOrder, "PIJSOrder: no order exist");
        Order memory order = userOrders[msg.sender][orderId];
        require(order.status == 0, "PIJSOrder: order is invalid");
        require(order.renewable == 1, "PIJSOrder: order is not renewable");
        // if (order.renewTime > 0) {
        //     require(
        //         newEndTimestamp > block.timestamp &&
        //             newEndTimestamp > order.renewTime,
        //         "PIJSOrder: newEndTimestamp is invalid"
        //     );
        // } else {
        //     require(
        //         newEndTimestamp > block.timestamp &&
        //             newEndTimestamp > order.endTimestamp,
        //         "PIJSOrder: newEndTimestamp is invalid"
        //     );
        // }
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
         require(
            verifyBetBack(
                signature,
                orderId
            ),
            "ERROR:INVALID_REQUEST"
        );

        // has order
        bool _hasOrder = hasOrder(msg.sender, orderId);
        require(_hasOrder, "PIJSOrder: no order exist");
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
        require(order.payNum>0);

        // payable(msg.sender).transfer(order.payNum);
        (bool success, ) = payable(msg.sender).call{value: order.payNum}("");
        require(success, "Transfer failed");

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
        bytes memory signature,
        uint256 productId,
        uint256 orderId,
        uint256 userId,
        uint256 phase,
        uint256 purchaseNum,
        uint256 payNum,
        uint256 anchorCoinNum,
        string memory anchorCoin
    ) internal view returns (bool) {
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);
        bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        PERMIT_TYPEHASH,
                        productId,
                        orderId,
                        userId,
                        phase,
                        purchaseNum,
                        payNum,
                        anchorCoinNum,
                        anchorCoin
                    )
                )
            )
        );
        
        return   signer == ecrecover(signHash, v, r, s);

    }

    function verifyRenew( bytes memory signature,uint256 orderId,uint256 newEndTimestamp) internal view returns (bool) {
        // bytes32[] memory values = new bytes32[](2);
        // values[0] = bytes32(orderId);
        // values[1] = bytes32(newEndTimestamp);
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
        
        return   signer == ecrecover(signHash, v, r, s);
    }

    function verifyBetBack( bytes memory signature,uint256 orderId) internal view returns (bool) {
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
        
        return   signer == ecrecover(signHash, v, r, s);
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



    ///////////////////////////////////////////////////////////////////////////////////////// V2
    // make order v2
    struct OrderV2 {
        uint256 productId; // 购买的产品ID
        uint256 orderId; // 订单号
        uint256 userId; // 用户ID
        uint256 purchaseNum; //购买的份数
        uint256 payNum; // 支付的PIJS
        uint256 startTimestamp; // 订单开始时间(可以由区块决定)
        uint256 endTimestamp; // 订单到期时间
        uint256 v2RoundId; // 轮次ID
        uint256 roundProductLimit; // 单个轮次单个用户的限购
        uint256 renewable; // 是否可续期
        uint256 payValue; // 支付的PIJS的价值
        uint256 status; // 0 -staking; 1- unstaking  状态
        uint256 renewTime; // 续期时间
    }
    mapping(address => mapping(uint256 => OrderV2)) public userOrdersV2;
      // v2RoundId -> user -> orderCount
    mapping(uint256 => mapping(address => uint256)) public orderSumByUserByv2Round;
    bytes32 private constant PERMIT_MAKEORDER_V2_TYPEHASH =
        keccak256(
            abi.encodePacked(
                "Permit(uint256 productId,uint256 orderId,uint256 userId,uint256 purchaseNum,uint256 payNum,uint256 startTimestamp,uint256 endTimestamp,uint256 v2RoundId,uint256 roundProductLimit,uint256 renewable,uint256 payValue)"
            )
        );
    event MakeOrderV2(
        address caller,
        uint256 productId,
        uint256 orderId,
        uint256 userId,
        uint256 purchaseNum,
        uint256 payNum,
        uint256 startTimestamp,
        uint256 endTimestamp,
        uint256 v2RoundId,
        uint256 roundProductLimit,
        uint256 renewable,
        uint256 payValue,
        uint256 relationId
    );
    struct MakeOrderDataV2 {
        uint256 productId; 
        uint256 orderId; 
        uint256 userId; 
        uint256 purchaseNum; 
        uint256 payNum; 
        uint256 startTimestamp; 
        uint256 endTimestamp; 
        uint256 v2RoundId; 
        uint256 roundProductLimit; 
        uint256 renewable; 
        uint256 payValue; 
        uint256 relationId;
        bytes  signature;

    }
    function makeOrderV2(bytes memory data) public payable nonReentrant {
        MakeOrderDataV2 memory mkV2 = parseMakeOrderDataV2(data);
        // user v2RoundId limit
        require(orderSumByUserByv2Round[mkV2.v2RoundId][msg.sender].add(mkV2.purchaseNum) <= mkV2.roundProductLimit,"PIJSOrder: The order quantity exceeds the limit of the round");
        require(userOrdersV2[msg.sender][mkV2.orderId].orderId == 0, "PIJSOrder: ORDER_EXISTS");
        // update userOrdersV2
        userOrdersV2[msg.sender][mkV2.orderId] = OrderV2({
            productId: mkV2.productId,
            orderId: mkV2.orderId,
            userId: mkV2.userId,
            purchaseNum: mkV2.purchaseNum,
            payNum: mkV2.payNum,
            startTimestamp: mkV2.startTimestamp,
            endTimestamp: mkV2.endTimestamp,
            v2RoundId: mkV2.v2RoundId,
            roundProductLimit: mkV2.roundProductLimit,
            renewable: mkV2.renewable,
            payValue: mkV2.payValue,
            status:0,
            renewTime:0
        });
        // update orderSumByUserByv2Round
        orderSumByUserByv2Round[mkV2.v2RoundId][msg.sender] = orderSumByUserByv2Round[mkV2.v2RoundId][msg.sender].add(mkV2.purchaseNum);

        // event
        emit MakeOrderV2(
            msg.sender,
            mkV2.productId,
            mkV2.orderId,
            mkV2.userId,
            mkV2.purchaseNum,
            mkV2.payNum,
            mkV2.startTimestamp,
            mkV2.endTimestamp,
            mkV2.v2RoundId,
            mkV2.roundProductLimit,
            mkV2.renewable,
            mkV2.payValue,
            mkV2.relationId
        );
    }

    function parseMakeOrderDataV2(bytes memory data) internal returns(MakeOrderDataV2 memory){
        (
            uint256 productId,
            uint256 orderId,
            uint256 userId,
            uint256 purchaseNum, 
            uint256 payNum,
            uint256 startTimestamp, 
            uint256 endTimestamp,
            uint256 v2RoundId,
            uint256 roundProductLimit,
            uint256 renewable,
            uint256 payValue,
            uint256 relationId,
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
                bytes
            )
        );
         // validite para
        require(msg.value >= payNum, "PIJSOrder: invalid payNum");
        require(endTimestamp > block.timestamp, "PIJSOrder: order invalid");
        MakeOrderDataV2 memory order = MakeOrderDataV2({
            productId: productId,
            orderId: orderId,
            userId: userId,
            purchaseNum: purchaseNum, 
            payNum: payNum,
            startTimestamp: startTimestamp, 
            endTimestamp: endTimestamp,
            v2RoundId: v2RoundId,
            roundProductLimit: roundProductLimit,
            renewable: renewable,
            payValue: payValue,
            relationId:relationId,
            signature:signature
        });
        require(verfyMakeOrderV2(order),"ERROR:INVALID_REQUEST");


        return MakeOrderDataV2({
            productId: productId,
            orderId: orderId,
            userId: userId,
            purchaseNum: purchaseNum, 
            payNum: payNum,
            startTimestamp: startTimestamp, 
            endTimestamp: endTimestamp,
            v2RoundId: v2RoundId,
            roundProductLimit: roundProductLimit,
            renewable: renewable,
            payValue: payValue,
            relationId:relationId,
            signature:signature
        });

    }
 
    function verfyMakeOrderV2(MakeOrderDataV2 memory orderV2) internal view returns (bool) {
         (uint8 v, bytes32 r, bytes32 s) = splitSignature(orderV2.signature);
          bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        PERMIT_MAKEORDER_V2_TYPEHASH,
                        orderV2.productId,
                        orderV2.orderId,
                        orderV2.userId,
                        orderV2.purchaseNum, 
                        orderV2.payNum,
                        orderV2.startTimestamp, 
                        orderV2.endTimestamp,
                        orderV2.v2RoundId,
                        orderV2.roundProductLimit,
                        orderV2.renewable,
                        orderV2.payValue
                    )
                )
            )
        );
        return   signer == ecrecover(signHash, v, r, s);
    }
    // renew order
    struct RenewOrderV2Data {
        uint256 orderId;
        uint256 newEndTimestamp;
        bytes  signature;
    }
    event ReNewOrderV2(address caller, uint256 orderId, uint256 newEndTimestamp);
    mapping(address => RenewOrder[]) public userRenewOrdersV2;
    function reNewOrdeV2( bytes memory data ) public nonReentrant {
        RenewOrderV2Data memory reNewV2Data = parseRenewOrderV2Data(data);  
        OrderV2 memory order = userOrdersV2[msg.sender][reNewV2Data.orderId];
        require(order.orderId != 0,"PIJSOrder: no order exist");
        require(order.status == 0, "PIJSOrder: order is invalid");
        require(order.renewable == 1, "PIJSOrder: order is not renewable");
        userOrdersV2[msg.sender][reNewV2Data.orderId ].renewTime = reNewV2Data.newEndTimestamp;
        userRenewOrdersV2[msg.sender].push(
            RenewOrder({
                orderId: reNewV2Data.orderId,
                renewTime: reNewV2Data.newEndTimestamp,
                blockTime: block.timestamp
            })
        );

        emit ReNewOrderV2(msg.sender, order.orderId, reNewV2Data.newEndTimestamp);

    }

    function parseRenewOrderV2Data(bytes memory data) internal view returns(RenewOrderV2Data memory){
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
        require(
            verifyRenew(
                signature,
                orderId,
                newEndTimestamp
            ),
            "ERROR:INVALID_REQUEST"
        );
        return RenewOrderV2Data({
            orderId:orderId,
            newEndTimestamp:newEndTimestamp,
            signature:signature
        });
    }

    // betBackOrder
    event BetBackOrderV2(address caller, uint256 orderId, uint256 amount);
    function betBackOrderV2( bytes memory data) public nonReentrant {
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
         require(
            verifyBetBack(
                signature,
                orderId
            ),
            "ERROR:INVALID_REQUEST"
        );
        OrderV2 memory order = userOrdersV2[msg.sender][orderId];
        require(order.orderId != 0,"PIJSOrder: no order exist");
        require(order.status == 0, "PIJSOrder: order is invalid");
        userOrders[msg.sender][orderId].status = 1;
        require(order.payNum>0);
        // payable(msg.sender).transfer(order.payNum);
        (bool success, ) = payable(msg.sender).call{value: order.payNum}("");
        require(success, "Transfer failed");

        emit BetBackOrderV2(msg.sender, orderId, order.payNum);

    }






    
}
