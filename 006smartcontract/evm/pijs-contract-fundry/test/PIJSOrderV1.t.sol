// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/swap&mining/PIJSOrderV1Debug.sol";
import "forge-std/console.sol";

contract PIJSOrderV1Test is Test {
    PIJSOrderV1Debug public orderContract;

    address deployer = address(0xcdec065c7Eee6daAbd052AaA42D9aAc5838228aF);
    address user = address(0x79D72302236aBeF700C5E4b753BB4441d703F077);
    address signer = address(0xcdec065c7Eee6daAbd052AaA42D9aAc5838228aF);

    function setUp() public {
        vm.startPrank(deployer);
        orderContract = new PIJSOrderV1Debug();
        orderContract.initialize(deployer);
        orderContract.grantRole(orderContract.OPERATE_ROLE(), deployer);
        // vm.deal(deployer, 100 ether); // 给 user 100 ETH
        vm.stopPrank();
    }

    function testInitialSetup() public view {
        assertEq(orderContract.signer(), deployer);
        assertTrue(orderContract.hasRole(orderContract.MANAGE_ROLE(), deployer));
    }

    function testWithdraw() public {
        vm.prank(user);
        payable(address(orderContract)).transfer(1 ether);
        assertEq(orderContract.balance(), 1 ether);

        vm.prank(deployer);
        orderContract.withdraw(deployer, 1 ether);
        assertEq(orderContract.balance(), 0);
    }

    function testMakeOrder() public {
        
        // 构造 order 参数
        uint256 productId = 1;
        uint256 orderId = 10001;
        uint256 userId = 9999;
        uint256 purchaseNum = 1;
        uint256 payNum = 1 ether;
        uint256 endTimestamp = block.timestamp + 1 days;
        uint256 startTimestamp = block.timestamp;
        uint256 userPurchaseLimit = 10;
        uint256 productLimit = 10;
        uint256 userProductPurchaseLimit = 10;
        uint256 isAddPurchaseSum = 1;
        uint256 phase = 1;
        uint256 renewable = 1;
        uint256 anchorCoinNum = 100;
        string memory anchorCoin = "USDT";

        // 构造签名
        bytes32 structHash = keccak256(
            abi.encode(
                keccak256("Permit(uint256 productId,uint256 orderId,uint256 userId,uint256 phase,uint256 purchaseNum,uint256 payNum,uint256 anchorCoinNum,string anchorCoin)"),
                productId,
                orderId,
                userId,
                phase,
                purchaseNum,
                payNum,
                anchorCoinNum,
                anchorCoin
                // keccak256(bytes(anchorCoin))
            )
        );
        bytes32 domainSeparator = orderContract.DOMAIN_SEPARATOR();
       // console.logBytes32(domainSeparator);
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
        vm.prank(deployer);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(0x67e46235d56575ab55fdcaafd40c82833476f00bc5270dca0504193b30a53632, digest);
        bytes memory signature = abi.encodePacked(r, s, v);

        // ABI encode参数
        bytes memory data = abi.encode(
            productId,
            orderId,
            userId,
            purchaseNum,
            payNum,
            endTimestamp,
            startTimestamp,
            userPurchaseLimit,
            productLimit,
            userProductPurchaseLimit,
            isAddPurchaseSum,
            phase,
            renewable,
            anchorCoinNum,
            anchorCoin,
            signature
        );

        // 发起 makeOrder 调用
        // vm.prank(deployer);
        orderContract.makeOrder{value: payNum}(data);
        // 验证订单是否已创建
        (uint256 productId_,
            uint256 orderId_,
            uint256 userId_,
            uint256 purchaseNum_,
            uint256 payNum_,
            uint256 endTimestamp_,
            uint256 startTimestamp_,
            uint256 userPurchaseLimit_,
            uint256 productPurchaseLimit_,
            uint256 phase_,
            uint256 renewable_,
            uint256 anchorCoinNum_,
            string memory anchorCoin_,
            uint256  status_,
            uint256 renewTime_) = orderContract.userOrders(deployer, orderId);
        assertEq(userId_, userId);
        assertEq(endTimestamp, endTimestamp);
        assertEq(userPurchaseLimit_, userPurchaseLimit);
        assertEq(phase_, phase);


    }

    



}


// forge test --fork-url http://127.0.0.1:7545 --match-path test/PIJSOrderV1.t.sol -vvvv 

