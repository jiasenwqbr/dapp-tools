import { expect } from "chai";
import { ethers, upgrades } from "hardhat";
import { PIJSOrderV1 } from "../typechain-types";
describe("PIJSOrderV1", () => {
  let orderContract: PIJSOrderV1;
  let owner: any;
  let user1: any;
  let user2: any;
  let user3: any;
  beforeEach(async () => {
    [owner, user3 ,user1, user2] = await ethers.getSigners();

    const PIJSOrderFactory = await ethers.getContractFactory("PIJSOrderV1");
    orderContract = (await upgrades.deployProxy(PIJSOrderFactory, [owner.address], {
      initializer: "initialize",
    })) as PIJSOrderV1;

    await orderContract.deployed();
  });
  describe("Initialization", () => {
    it("should assign roles correctly", async () => {
      const hasManageRole = await orderContract.hasRole(
        await orderContract.MANAGE_ROLE(),
        owner.address
      );
      expect(hasManageRole).to.be.true;
    });
  });

  describe("Function Switch", () => {
    it("should allow MANAGE_ROLE to toggle funcSwitch", async () => {
      await orderContract.setFuncSwith(true);
      // 检查状态通过调用公开 getter（如果有），否则跳过
    });
  });

  describe("Order Placement", () => {
    // should allow placing an order with valid signature
    it("should allow placing an order with valid signature", async () => {
      const fakeOrder = {
        productId: 1,
        orderId: 123,
        userId: 1,
        purchaseNum: 1,
        payNum: ethers.utils.parseEther("0.5"),
        endTimestamp: Math.floor(Date.now() / 1000) + 3600,
        startTimestamp: Math.floor(Date.now() / 1000),
        userPurchaseLimit: 5,
        productPurchaseLimit: 10,
        phase: 1,
        renewable: 1,
        anchorCoinNum: 100,
        anchorCoin: ethers.utils.formatBytes32String("USDT"),
      };
      const types = {
        Permit: [
          { name: "productId", type: "uint256" },
          { name: "orderId", type: "uint256" },
          { name: "userId", type: "uint256" },
          { name: "phase", type: "uint256" },
          { name: "purchaseNum", type: "uint256" },
          { name: "payNum", type: "uint256" },
          { name: "anchorCoinNum", type: "uint256" },
          { name: "anchorCoin", type: "bytes32" },
        ],
      };
      const domain = {
        name: "PIJSOrder",
        version: "1",
        chainId: (await ethers.provider.getNetwork()).chainId,
        verifyingContract: orderContract.address,
      };
      const signature = await owner._signTypedData(domain, types, fakeOrder);
      const data = ethers.utils.defaultAbiCoder.encode(
        [
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "bytes32",
          "bytes",
        ],
        [
          fakeOrder.productId,
          fakeOrder.orderId,
          fakeOrder.userId,
          fakeOrder.purchaseNum,
          fakeOrder.payNum,
          fakeOrder.endTimestamp,
          fakeOrder.startTimestamp,
          fakeOrder.userPurchaseLimit,
          fakeOrder.productPurchaseLimit,
          fakeOrder.phase,
          fakeOrder.renewable,
          fakeOrder.anchorCoinNum,
          fakeOrder.anchorCoin,
          signature,
        ]
      );
      //console.log("data:",data);
      await expect(
        orderContract.connect(user1).makeOrder(data, {
          value: fakeOrder.payNum,
        })
      ).to.emit(orderContract, "MakeOrder");
      expect(await orderContract.balance()).to.equal(ethers.utils.parseEther("0.5"));
    });

    // should fail if payNum (msg.value) is too low
    it("should fail if payNum (msg.value) is too low", async () => {
      const params = {
        productId: 1,
        orderId: 100,
        userId: 123,
        purchaseNum: 1,
        payNum: ethers.utils.parseEther("1"),
        endTimestamp: Math.floor(Date.now() / 1000) + 86400,
        startTimestamp: Math.floor(Date.now() / 1000),
        userPurchaseLimit: 5,
        productPurchaseLimit: 10,
        phase: 1,
        renewable: 1,
        anchorCoinNum: 100,
        anchorCoin: ethers.utils.formatBytes32String("USDT"),
        signature: getFakeSignature(),
      };

      const data = encodeOrder(params);
      try {
        await orderContract.connect(user1).makeOrder(data, {
          value: ethers.utils.parseEther("0.5"), // 少于 payNum
          gasLimit: 1_000_000,
        });
        expect.fail("Expected makeOrder to revert but it succeeded");
      } catch (err: any) {
        // 捕获 Hardhat 的 revert 错误信息
        expect(err.message).to.include("PIJSOrder: invalid payNum");
      }
    });
    // should fail if endTimestamp <= block.timestamp
    it("should fail if endTimestamp <= block.timestamp",async ()=> {
      const params = {
        productId: 1,
        orderId: 100,
        userId: 123,
        purchaseNum: 1,
        payNum: ethers.utils.parseEther("1"),
        endTimestamp: Math.floor(Date.now() / 1000),
        startTimestamp: Math.floor(Date.now() / 1000),
        userPurchaseLimit: 5,
        productPurchaseLimit: 10,
        phase: 1,
        renewable: 1,
        anchorCoinNum: 100,
        anchorCoin: ethers.utils.formatBytes32String("USDT"),
        signature: getFakeSignature(),
      };
      const data = encodeOrder(params);
      try {
        await orderContract.connect(user1).makeOrder(data, {
          value: ethers.utils.parseEther("1"), // 少于 payNum
          gasLimit: 1_000_000,
        });
        expect.fail("Expected makeOrder to revert but it succeeded");
      } catch (err: any) {
        // 捕获 Hardhat 的 revert 错误信息
        expect(err.message).to.include("PIJSOrder: order invalid");
      }
    });
    // purchaseNum<= userPurchaseLimit
    it("should fail if purchaseNum > userPurchaseLimit",async () => {
      const fakeOrder = {
        productId: 1,
        orderId: 100,
        userId: 12,
        purchaseNum: 6,
        payNum: ethers.utils.parseEther("1"),
        endTimestamp: Math.floor(Date.now() / 1000)+8600,
        startTimestamp: Math.floor(Date.now() / 1000),
        userPurchaseLimit: 8,
        productPurchaseLimit: 4,
        phase: 1,
        renewable: 1,
        anchorCoinNum: 100,
        anchorCoin: ethers.utils.formatBytes32String("USDT"),
      };
      const types = {
        Permit: [
          { name: "productId", type: "uint256" },
          { name: "orderId", type: "uint256" },
          { name: "userId", type: "uint256" },
          { name: "phase", type: "uint256" },
          { name: "purchaseNum", type: "uint256" },
          { name: "payNum", type: "uint256" },
          { name: "anchorCoinNum", type: "uint256" },
          { name: "anchorCoin", type: "bytes32" },
        ],
      };
      const domain = {
        name: "PIJSOrder",
        version: "1",
        chainId: (await ethers.provider.getNetwork()).chainId,
        verifyingContract: orderContract.address,
      };
      const signature = await owner._signTypedData(domain, types, fakeOrder);
      const params = {
        ...fakeOrder,
        signature: signature,
      };
      const data = encodeOrder(params);
      try {
        await orderContract.connect(user1).makeOrder(data, {
          value: ethers.utils.parseEther("1"), // 少于 payNum
          gasLimit: 1_000_000,
        });
        expect.fail("Expected makeOrder to revert but it succeeded");
      } catch (err: any) {
       // console.log(err);
        // 捕获 Hardhat 的 revert 错误信息
        expect(err.message).to.include("PIJSOrder: The product quantity exceeds the limit");
      }
    });

    // purchaseNum<= userPurchaseLimit
    it("should fail if  purchaseNum > userPurchaseLimit", async () => {
      const fakeOrder = {
        productId: 1,
        orderId: 100,
        userId: 12,
        purchaseNum: 6,
        payNum: ethers.utils.parseEther("1"),
        endTimestamp: Math.floor(Date.now() / 1000)+8600,
        startTimestamp: Math.floor(Date.now() / 1000),
        userPurchaseLimit: 5,
        productPurchaseLimit: 10,
        phase: 1,
        renewable: 1,
        anchorCoinNum: 100,
        anchorCoin: ethers.utils.formatBytes32String("USDT"),
      };
      const types = {
        Permit: [
          { name: "productId", type: "uint256" },
          { name: "orderId", type: "uint256" },
          { name: "userId", type: "uint256" },
          { name: "phase", type: "uint256" },
          { name: "purchaseNum", type: "uint256" },
          { name: "payNum", type: "uint256" },
          { name: "anchorCoinNum", type: "uint256" },
          { name: "anchorCoin", type: "bytes32" },
        ],
      };
      const domain = {
        name: "PIJSOrder",
        version: "1",
        chainId: (await ethers.provider.getNetwork()).chainId,
        verifyingContract: orderContract.address,
      };
      const signature = await owner._signTypedData(domain, types, fakeOrder);
      const params = {
        ...fakeOrder,
        signature: signature,
      };
      const data = encodeOrder(params);
      try {
        await orderContract.connect(user1).makeOrder(data, {
          value: ethers.utils.parseEther("1"), // 少于 payNum
          gasLimit: 1_000_000,
        });
        expect.fail("Expected makeOrder to revert but it succeeded");
      } catch (err: any) {
        // console.log(err);
        // 捕获 Hardhat 的 revert 错误信息
        expect(err.message).to.include("The order quantity exceeds the limit");
      }

    });

    it("Test the balance of the contract",async ()=> {
      const fakeOrder = {
        productId: 1,
        orderId: 123,
        userId: 1,
        purchaseNum: 1,
        payNum: ethers.utils.parseEther("0.5"),
        endTimestamp: Math.floor(Date.now() / 1000) + 3600,
        startTimestamp: Math.floor(Date.now() / 1000),
        userPurchaseLimit: 5,
        productPurchaseLimit: 10,
        phase: 1,
        renewable: 1,
        anchorCoinNum: 100,
        anchorCoin: ethers.utils.formatBytes32String("USDT"),
      };
      const types = {
        Permit: [
          { name: "productId", type: "uint256" },
          { name: "orderId", type: "uint256" },
          { name: "userId", type: "uint256" },
          { name: "phase", type: "uint256" },
          { name: "purchaseNum", type: "uint256" },
          { name: "payNum", type: "uint256" },
          { name: "anchorCoinNum", type: "uint256" },
          { name: "anchorCoin", type: "bytes32" },
        ],
      };
      const domain = {
        name: "PIJSOrder",
        version: "1",
        chainId: (await ethers.provider.getNetwork()).chainId,
        verifyingContract: orderContract.address,
      };
      const signature = await owner._signTypedData(domain, types, fakeOrder);
      const data = ethers.utils.defaultAbiCoder.encode(
        [
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "bytes32",
          "bytes",
        ],
        [
          fakeOrder.productId,
          fakeOrder.orderId,
          fakeOrder.userId,
          fakeOrder.purchaseNum,
          fakeOrder.payNum,
          fakeOrder.endTimestamp,
          fakeOrder.startTimestamp,
          fakeOrder.userPurchaseLimit,
          fakeOrder.productPurchaseLimit,
          fakeOrder.phase,
          fakeOrder.renewable,
          fakeOrder.anchorCoinNum,
          fakeOrder.anchorCoin,
          signature,
        ]
      );
      //console.log("data:",data);
      await expect(
        orderContract.connect(user1).makeOrder(data, {
          value: fakeOrder.payNum,
        })
      ).to.emit(orderContract, "MakeOrder");
      expect(await orderContract.balance()).to.equal(ethers.utils.parseEther("0.5"));
    });



  });

  describe("Renew order",()=>{
    beforeEach(async ()=> {
       const fakeOrder = {
        productId: 1,
        orderId: 123,
        userId: 1,
        purchaseNum: 1,
        payNum: ethers.utils.parseEther("0.5"),
        endTimestamp: Math.floor(Date.now() / 1000) + 3600,
        startTimestamp: Math.floor(Date.now() / 1000),
        userPurchaseLimit: 5,
        productPurchaseLimit: 10,
        phase: 1,
        renewable: 1,
        anchorCoinNum: 100,
        anchorCoin: ethers.utils.formatBytes32String("USDT"),
      };
      const types = {
        Permit: [
          { name: "productId", type: "uint256" },
          { name: "orderId", type: "uint256" },
          { name: "userId", type: "uint256" },
          { name: "phase", type: "uint256" },
          { name: "purchaseNum", type: "uint256" },
          { name: "payNum", type: "uint256" },
          { name: "anchorCoinNum", type: "uint256" },
          { name: "anchorCoin", type: "bytes32" },
        ],
      };
      const domain = {
        name: "PIJSOrder",
        version: "1",
        chainId: (await ethers.provider.getNetwork()).chainId,
        verifyingContract: orderContract.address,
      };
      const signature = await owner._signTypedData(domain, types, fakeOrder);
      const data = ethers.utils.defaultAbiCoder.encode(
        [
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "bytes32",
          "bytes",
        ],
        [
          fakeOrder.productId,
          fakeOrder.orderId,
          fakeOrder.userId,
          fakeOrder.purchaseNum,
          fakeOrder.payNum,
          fakeOrder.endTimestamp,
          fakeOrder.startTimestamp,
          fakeOrder.userPurchaseLimit,
          fakeOrder.productPurchaseLimit,
          fakeOrder.phase,
          fakeOrder.renewable,
          fakeOrder.anchorCoinNum,
          fakeOrder.anchorCoin,
          signature,
        ]
      );
      await  orderContract.connect(user1).makeOrder(data, {
          value: fakeOrder.payNum,
      });
      
      console.log("before each,the balance of the contract is : ",await orderContract.balance());
    });

    it("balance of the contract", async ()=> {
       expect(await orderContract.balance()).to.equal(ethers.utils.parseEther("0.5"));
    });

    it("renew order", async ()=> {
      const fakeOrder = {
        orderId: 123,
        newEndTimestamp: Math.floor(Date.now() / 1000) + 9600,
      };
      const types = {
        Permit: [
          { name: "orderId", type: "uint256" },
          { name: "newEndTimestamp", type: "uint256" },
        ],
      };
    })


  });


});

function getFakeSignature(): string {
  return "0x" + "0".repeat(130); // 假签名
}

function encodeOrder(params: any): string {
  return ethers.utils.defaultAbiCoder.encode(
    [
      "uint256", // productId
      "uint256", // orderId
      "uint256", // userId
      "uint256", // purchaseNum
      "uint256", // payNum
      "uint256", // endTimestamp
      "uint256", // startTimestamp
      "uint256", // userPurchaseLimit
      "uint256", // productPurchaseLimit
      "uint256", // phase
      "uint256", // renewable
      "uint256", // anchorCoinNum
      "bytes32", // anchorCoin
      "bytes"    // signature
    ],
    [
      params.productId,
      params.orderId,
      params.userId,
      params.purchaseNum,
      params.payNum,
      params.endTimestamp,
      params.startTimestamp,
      params.userPurchaseLimit,
      params.productPurchaseLimit,
      params.phase,
      params.renewable,
      params.anchorCoinNum,
      params.anchorCoin,
      params.signature,
    ]
  );
}

/**
 npx hardhat test ./test/PIJSOrder.test.ts --network ganache
 */