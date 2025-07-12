import { expect } from "chai";
import { ethers as ethers_1, upgrades } from "hardhat";
import { PIJSOrderV1 } from "../typechain-types";
import { network } from "hardhat";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";

import { BigNumber,ethers } from "ethers";
describe("PIJSOrderV1", () => {
    let orderContract: PIJSOrderV1;
    let owner: any;
    let user1: any;
    let user2: any;
    let user3: any;
    let user: any;
    beforeEach(async () => {
        [owner, user,user3, user1, user2] = await ethers_1.getSigners();

        const PIJSOrderFactory = await ethers_1.getContractFactory("PIJSOrderV1Debug2");
        orderContract = (await upgrades.deployProxy(PIJSOrderFactory, [owner.address], {
            initializer: "initialize",
        })) as PIJSOrderV1;

        await orderContract.deployed();
    });
    describe("make order", () => {
        it("PIJSOrder: The order quantity exceeds the user product purchase limit", async () => {
            let coin:string = "USDT";
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
                // anchorCoin: ethers.utils.hexlify(ethers.utils.toUtf8Bytes("USDT"))
                // anchorCoin: coin,
                // anchorCoin:ethers.utils.keccak256(ethers.utils.toUtf8Bytes("USDT"))
                anchorCoin:"USDT"
            };

            // 1. 定义 EIP-712 的 Domain
            const domain: ethers.TypedDataDomain = {
                name: 'PIJSOrder',
                version: '1',
                chainId: (await ethers_1.provider.getNetwork()).chainId,
                verifyingContract: orderContract.address
            };
            // 2. 定义 Permit 结构体的类型字符串，必须和合约中的 PERMIT_TYPEHASH 完全对应
            const permitTypeString = "Permit(uint256 productId,uint256 orderId,uint256 userId,uint256 phase,uint256 purchaseNum,uint256 payNum,uint256 anchorCoinNum,string anchorCoin)";
            
            // 3. 计算 PERMIT_TYPEHASH (keccak256 of the type string)
            const PERMIT_TYPEHASH: string = ethers.utils.id(permitTypeString);

            // 4. 【核心步骤】手动构建需要哈希的结构体
            // 我们将严格按照合约 verify 函数的 abi.encode 逻辑来编码
            const structEncoded: string = ethers.utils.defaultAbiCoder.encode(
                [
                    'bytes32', 'uint256', 'uint256', 'uint256', 'uint256', 
                    'uint256', 'uint256', 'uint256', 'string'
                ],
                [
                    PERMIT_TYPEHASH,
                    fakeOrder.productId,
                    fakeOrder.orderId,
                    fakeOrder.userId,
                    fakeOrder.phase,
                    fakeOrder.purchaseNum,
                    fakeOrder.payNum,
                    fakeOrder.anchorCoinNum,
                    fakeOrder.anchorCoin // <- 注意：这里我们使用了原始的 string，而不是它的哈希，以匹配合约
                ]
            );
            // 5. 计算结构体的哈希 (keccak256 of the encoded struct)
            const structHash: string = ethers.utils.keccak256(structEncoded);

            // 6. 计算 Domain Separator
            const domainSeparator: string = ethers.utils._TypedDataEncoder.hashDomain(domain);
            // 7. 组合成最终需要签名的 digest
            // 这完全模拟了合约中的 keccak256(abi.encodePacked("\x19\x01", DOMAIN_SEPARATOR, structHash))
            const digest: string = ethers.utils.keccak256(
                ethers.utils.concat([
                    "0x1901",
                    domainSeparator,
                    structHash
                ])
            );
            console.log("digist:",digest);
             // 8. 对最终的 digest 进行签名
            // 我们需要使用 signer 的一个内部函数 _signDigest 来对一个已经哈希过的值进行签名。
            const flatSignature: string = await owner.signMessage(ethers.utils.arrayify(digest));
            //owner._signDigest
            console.log("flatSignature:",flatSignature);

            const params = {
                ...fakeOrder,
                signature: flatSignature,
            };
            console.log(params);
            const data = encodeOrder(params);
            console.log(data);

             try {
                await orderContract.connect(owner).makeOrder(data, {
                    value: fakeOrder.payNum, 
                    gasLimit: 2_000_000,
                });

                // const orders = await orderContract.getUserOrder(owner.address);
                // expect(orders.length).to.equal(1);
                // expect(orders[0].orderId).to.equal(fakeOrder.orderId);

                // expect(await orderContract.balance()).to.equal(ethers.utils.parseEther("0.5"));
            } catch (err: any) {
                console.log(err);
                // 捕获 Hardhat 的 revert 错误信息
                // expect(err.message).to.include("PIJSOrder: The order quantity exceeds the user product purchase limit");
            }


        }


          
            
    );

}

);
})

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
                params.anchorCoin,
                // ethers.utils.keccak256(ethers.utils.toUtf8Bytes(params.anchorCoin)),
                params.signature,
            ]
        );
    }


/**
 npx hardhat test ./test/PIJSOrder2.test.ts --network ganache
 */


