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
    let user: any;
    beforeEach(async () => {
        [owner, user,user3, user1, user2] = await ethers.getSigners();

        const PIJSOrderFactory = await ethers.getContractFactory("PIJSOrderV1Debug2");
        orderContract = (await upgrades.deployProxy(PIJSOrderFactory, [owner.address], {
            initializer: "initialize",
        })) as PIJSOrderV1;

        await orderContract.deployed();
    });
    describe("make order", () => {
        it("PIJSOrder: The order quantity exceeds the user product purchase limit", async () => {
            const fakeOrder = {
                productId: 1,
                orderId: 100,
                userId: 12,
                purchaseNum: 7,
                payNum: ethers.utils.parseEther("0.5"),
                endTimestamp: Math.floor(Date.now() / 1000)+8600,
                startTimestamp: Math.floor(Date.now() / 1000),
                userPurchaseLimit: 10,
                productPurchaseLimit: 100,
                userProductPurchaseLimit:8,
                isAddPurchaseSum:1,
                phase: 1,
                renewable: 1,
                anchorCoinNum: 100,
               //  anchorCoin: ethers.utils.hexlify(ethers.utils.toUtf8Bytes("USDT"))
                // anchorCoin:ethers.utils.keccak256(ethers.utils.toUtf8Bytes("USDT"))
                anchorCoin: "USDT",
                // anchorCoin:ethers.utils.toUtf8String(ethers.utils.toUtf8Bytes("USDT"))
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
                    { name: "anchorCoin", type: "string" },
                ],
            };

            const domain = {
                name: "PIJSOrder",
                version: "1",
                chainId: (await ethers.provider.getNetwork()).chainId,
                verifyingContract: orderContract.address,
            };

            const signOrder = {
                productId: fakeOrder.productId,
                orderId: fakeOrder.orderId,
                userId: fakeOrder.userId,
                phase: fakeOrder.phase,
                purchaseNum: fakeOrder.purchaseNum,
                payNum: fakeOrder.payNum,
                anchorCoinNum: fakeOrder.anchorCoinNum,
                // anchorCoin :  ethers.utils.keccak256(ethers.utils.toUtf8Bytes(fakeOrder.anchorCoin)),
                anchorCoin: fakeOrder.anchorCoin,
                // anchorCoin: ethers.utils.hexlify(ethers.utils.toUtf8Bytes("USDT"))
                //  anchorCoin: ethers.utils.toUtf8Bytes("USDT")
            }

            const signature = await owner._signTypedData(domain, types, signOrder);

            console.log(signature);
             console.log("anchorCoin:",ethers.utils.toUtf8String(ethers.utils.toUtf8Bytes(fakeOrder.anchorCoin)));
            const params = {
                ...fakeOrder,
                signature: signature,
            };
            console.log(params);
            const data = encodeOrder(params);
            console.log(data);
            try {
                await orderContract.connect(user).makeOrder(data, {
                    value: fakeOrder.payNum, 
                    gasLimit: 2_000_000,
                });

                const orders = await orderContract.getUserOrder(user.address);
                expect(orders.length).to.equal(1);
                expect(orders[0].orderId).to.equal(fakeOrder.orderId);

                expect(await orderContract.balance()).to.equal(ethers.utils.parseEther("0.5"));
            } catch (err: any) {
                console.log(err);
                // 捕获 Hardhat 的 revert 错误信息
                // expect(err.message).to.include("PIJSOrder: The order quantity exceeds the user product purchase limit");
            }
            });
    });

}
);

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
            "string", // anchorCoin
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
                // ethers.utils.keccak256(ethers.utils.toUtf8Bytes(params.anchorCoin)),
                // ethers.utils.hexlify(ethers.utils.toUtf8Bytes(params.anchorCoin)),
                // params.anchorCoin,
                 ethers.utils.hexlify(ethers.utils.toUtf8Bytes("USDT")),
                // ethers.utils.keccak256(ethers.utils.toUtf8Bytes(params.anchorCoin)),
                // ethers.utils.keccak256(ethers.utils.toUtf8Bytes(params.anchorCoin)),
                params.signature,
            ]
        );
    }

/**
 npx hardhat test ./test/PIJSOrder2.test.ts --network ganache
 */