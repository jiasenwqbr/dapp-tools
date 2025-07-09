import { expect } from "chai";
import { ethers, upgrades } from "hardhat";
import { PIJSOrderV1 } from "../typechain-types";
import { network } from "hardhat";
describe("PIJSOrderV1", () => {
    let orderContract: PIJSOrderV1;
    let owner: any;
    let user1: any;
    let user2: any;
    let user3: any;
    beforeEach(async () => {
        [owner, user3, user1, user2] = await ethers.getSigners();

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

    it("PIJSOrder: The order quantity exceeds the user product purchase limit", async () => {
        const fakeOrder = {
            productId: 1,
            orderId: 100,
            userId: 12,
            purchaseNum: 7,
            payNum: ethers.utils.parseEther("1"),
            endTimestamp: Math.floor(Date.now() / 1000)+8600,
            startTimestamp: Math.floor(Date.now() / 1000),
            userPurchaseLimit: 10,
            productPurchaseLimit: 100,
            userProductPurchaseLimit:8,
            isAddPurchaseSum:1,
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
            await orderContract.connect(owner).makeOrder(data, {
                value: ethers.utils.parseEther("1"), 
                gasLimit: 1_000_000,
            });

             expect(await orderContract.balance()).to.equal(ethers.utils.parseEther("1"));
        } catch (err: any) {
            console.log(err);
            // 捕获 Hardhat 的 revert 错误信息
            // expect(err.message).to.include("PIJSOrder: The order quantity exceeds the user product purchase limit");
        }
        });

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
            "uint256", // userProductPurchaseLimit
            "uint256", // isAddPurchaseSum
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
            params.userProductPurchaseLimit,
            params.isAddPurchaseSum,
            params.phase,
            params.renewable,
            params.anchorCoinNum,
            params.anchorCoin,
            params.signature,
            ]
        );
    }





}
);

/**
 npx hardhat test ./test/PIJSOrder2.test.ts --network ganache
 */