import { expect } from "chai";
import { ethers, upgrades } from "hardhat";
import { network } from "hardhat";
import { PIJSBridgeTarget } from  "../../typechain-types";

describe("PIJSBridgeTarget", () => {
    let owner: any;
    let user: any;
    let receiver: any;
    let operator:any;
    let feeDenominator = 10000;
    let feePercent = 500;
    let pIJSBridgeTargetContract:PIJSBridgeTarget;

      beforeEach(async () => {
        [owner, user ,receiver,operator] = await ethers.getSigners();
        const pIJSBridgeTargetFactory =  await ethers.getContractFactory("PIJSBridgeTarget");
        pIJSBridgeTargetContract = (await upgrades.deployProxy(pIJSBridgeTargetFactory,[
                    "UAC",
                    "UAC",
                    owner.address,
                    operator.address,
                    receiver.address,
                    feePercent,
                    owner.address,
                ],{
                    initializer: "initialize",
                })) as PIJSBridgeTarget;
         await pIJSBridgeTargetContract.deployed();
      });

      describe("mint token",()=> {
        it("the balance of user and receiver is correct",async () => {
            const params = {
                caller:operator.address,
                to:user.address,
                amount:ethers.utils.parseEther("10"),
                orderId:1,
                chainId:(await ethers.provider.getNetwork()).chainId
            }

            const types = {
                Permit: [
                    { name: "caller", type: "address" },
                    { name: "to", type: "address" },
                    { name: "amount", type: "uint256" },
                    { name: "orderId", type: "uint256" },
                    { name: "chainId", type: "uint256" },
                ],
            };
            const domain = {
                name: "PIJSBridgeTarget",
                version: "1",
                chainId: (await ethers.provider.getNetwork()).chainId,
                verifyingContract: pIJSBridgeTargetContract.address,
            };
            const signature = await owner._signTypedData(domain, types, params);

            const data = ethers.utils.defaultAbiCoder.encode(
                [
                   "address",
                   "address",
                   "uint256",
                   "uint256", 
                   "uint256",
                   "bytes"
                ],[
                    params.caller,
                    params.to,
                    params.amount,
                    params.orderId,
                    params.chainId,
                    signature
                ]
            );

            const userBalanceBeforeMint = await pIJSBridgeTargetContract.balanceOf(user.address);
            const receiverBalanceBeforeMint = await pIJSBridgeTargetContract.balanceOf(receiver.address);

            console.log("userBalanceBeforeMint:",userBalanceBeforeMint);
            console.log("receiverBalanceBeforeMint:",receiverBalanceBeforeMint);

            await pIJSBridgeTargetContract.connect(operator).mintToken(data,{
                 gasLimit: 1_000_000,
            });

            const userBalanceAfterMint = await pIJSBridgeTargetContract.balanceOf(user.address);
            const receiverBalanceAfterMint = await pIJSBridgeTargetContract.balanceOf(receiver.address);

            console.log("userBalanceAfterMint:",userBalanceAfterMint);
            console.log("receiverBalanceAfterMint:",receiverBalanceAfterMint);

            const fee =  params.amount.mul(feePercent).div(feeDenominator);
            const userIncome = params.amount.sub(params.amount.mul(feePercent).div(feeDenominator));
            console.log("fee:",fee);
            console.log("userIncome:",userIncome);

            expect(userBalanceAfterMint).to.equal(userBalanceBeforeMint.add(userIncome));
            expect(receiverBalanceAfterMint).to.equal(receiverBalanceBeforeMint.add(fee));
        });



      });

      describe("burn token",() => {
        beforeEach(async () => {
             const params = {
                caller:operator.address,
                to:user.address,
                amount:ethers.utils.parseEther("10"),
                orderId:1,
                chainId:(await ethers.provider.getNetwork()).chainId
            }

            const types = {
                Permit: [
                    { name: "caller", type: "address" },
                    { name: "to", type: "address" },
                    { name: "amount", type: "uint256" },
                    { name: "orderId", type: "uint256" },
                    { name: "chainId", type: "uint256" },
                ],
            };
            const domain = {
                name: "PIJSBridgeTarget",
                version: "1",
                chainId: (await ethers.provider.getNetwork()).chainId,
                verifyingContract: pIJSBridgeTargetContract.address,
            };
            const signature = await owner._signTypedData(domain, types, params);

            const data = ethers.utils.defaultAbiCoder.encode(
                [
                   "address",
                   "address",
                   "uint256",
                   "uint256", 
                   "uint256",
                   "bytes"
                ],[
                    params.caller,
                    params.to,
                    params.amount,
                    params.orderId,
                    params.chainId,
                    signature
                ]
            );

            const userBalanceBeforeMint = await pIJSBridgeTargetContract.balanceOf(user.address);
            const receiverBalanceBeforeMint = await pIJSBridgeTargetContract.balanceOf(receiver.address);

            console.log("userBalanceBeforeMint:",userBalanceBeforeMint);
            console.log("receiverBalanceBeforeMint:",receiverBalanceBeforeMint);

            await pIJSBridgeTargetContract.connect(operator).mintToken(data,{
                 gasLimit: 1_000_000,
            });

            const userBalanceAfterMint = await pIJSBridgeTargetContract.balanceOf(user.address);
            const receiverBalanceAfterMint = await pIJSBridgeTargetContract.balanceOf(receiver.address);

            console.log("userBalanceAfterMint:",userBalanceAfterMint);
            console.log("receiverBalanceAfterMint:",receiverBalanceAfterMint);

            const fee =  params.amount.mul(feePercent).div(feeDenominator);
            const userIncome = params.amount.sub(params.amount.mul(feePercent).div(feeDenominator));
            console.log("fee:",fee);
            console.log("userIncome:",userIncome);

            expect(userBalanceAfterMint).to.equal(userBalanceBeforeMint.add(userIncome));
            expect(receiverBalanceAfterMint).to.equal(receiverBalanceBeforeMint.add(fee));
        });
        it("burn token user balance shoud be plus the burned value",async () => {
            const params = {
                caller : user.address,
                amount : ethers.utils.parseEther("9"),
                orderId:1,
                chainId:(await ethers.provider.getNetwork()).chainId
            };

            const types = {
                Permit:[
                        { name: "caller", type: "address" },
                        { name: "amount", type: "uint256" },
                        { name: "orderId", type: "uint256" },
                        { name: "chainId", type: "uint256" },
                    ],
            };
            const domain = {
                name: "PIJSBridgeTarget",
                version: "1",
                chainId: (await ethers.provider.getNetwork()).chainId,
                verifyingContract: pIJSBridgeTargetContract.address,
            };

            const signature = await owner._signTypedData(domain, types, params);
            const data = ethers.utils.defaultAbiCoder.encode(
                [
                    "address",
                    "uint256",
                    "uint256", 
                    "uint256",
                    "bytes"
                ],[
                    params.caller,
                    params.amount,
                    params.orderId,
                    params.chainId,
                    signature
                ]
            );

            const userBalanceBeforeBurn = await pIJSBridgeTargetContract.balanceOf(user.address);
            console.log("userBalanceBeforeBurn:",userBalanceBeforeBurn);
            await pIJSBridgeTargetContract.connect(user).tokenBurned(data,{
                 gasLimit: 1_000_000,
            });

            const userBalanceAfterBurn = await pIJSBridgeTargetContract.balanceOf(user.address);
            console.log("userBalanceAfterBurn:",userBalanceAfterBurn);

            expect(userBalanceAfterBurn).to.equal(userBalanceBeforeBurn.sub(params.amount));

        });

        it("burn token if the burned amount is more than the balance of the user,it shoud be error.",async ()=> {
             const params = {
                caller : user.address,
                amount : ethers.utils.parseEther("10"),
                orderId:1,
                chainId:(await ethers.provider.getNetwork()).chainId
            };

            const types = {
                Permit:[
                        { name: "caller", type: "address" },
                        { name: "amount", type: "uint256" },
                        { name: "orderId", type: "uint256" },
                        { name: "chainId", type: "uint256" },
                    ],
            };
            const domain = {
                name: "PIJSBridgeTarget",
                version: "1",
                chainId: (await ethers.provider.getNetwork()).chainId,
                verifyingContract: pIJSBridgeTargetContract.address,
            };

            const signature = await owner._signTypedData(domain, types, params);
            const data = ethers.utils.defaultAbiCoder.encode(
                [
                    "address",
                    "uint256",
                    "uint256", 
                    "uint256",
                    "bytes"
                ],[
                    params.caller,
                    params.amount,
                    params.orderId,
                    params.chainId,
                    signature
                ]
            );

            const userBalanceBeforeBurn = await pIJSBridgeTargetContract.balanceOf(user.address);
            console.log("userBalanceBeforeBurn:",userBalanceBeforeBurn);
            try {
                await pIJSBridgeTargetContract.connect(user).tokenBurned(data,{
                 gasLimit: 1_000_000,
            });
            } catch (error) {
                console.log(error)
            }
            const userBalanceAfterBurn = await pIJSBridgeTargetContract.balanceOf(user.address);
            console.log("userBalanceAfterBurn:",userBalanceAfterBurn);
         });

      });

      it("burn token if the burned amount is zero ,it shoud be error.",async ()=> {
             const params = {
                caller : user.address,
                amount : ethers.utils.parseEther("0"),
                orderId:1,
                chainId:(await ethers.provider.getNetwork()).chainId
            };

            const types = {
                Permit:[
                        { name: "caller", type: "address" },
                        { name: "amount", type: "uint256" },
                        { name: "orderId", type: "uint256" },
                        { name: "chainId", type: "uint256" },
                    ],
            };
            const domain = {
                name: "PIJSBridgeTarget",
                version: "1",
                chainId: (await ethers.provider.getNetwork()).chainId,
                verifyingContract: pIJSBridgeTargetContract.address,
            };

            const signature = await owner._signTypedData(domain, types, params);
            const data = ethers.utils.defaultAbiCoder.encode(
                [
                    "address",
                    "uint256",
                    "uint256", 
                    "uint256",
                    "bytes"
                ],[
                    params.caller,
                    params.amount,
                    params.orderId,
                    params.chainId,
                    signature
                ]
            );

            const userBalanceBeforeBurn = await pIJSBridgeTargetContract.balanceOf(user.address);
            console.log("userBalanceBeforeBurn:",userBalanceBeforeBurn);
            try {
                await pIJSBridgeTargetContract.connect(user).tokenBurned(data,{
                 gasLimit: 1_000_000,
            });
            } catch (error) {
                console.log(error)
            }
            const userBalanceAfterBurn = await pIJSBridgeTargetContract.balanceOf(user.address);
            console.log("userBalanceAfterBurn:",userBalanceAfterBurn);
         });

      });






/**
 * 
 
npx hardhat test ./test/bridge/PIJSBridgeTarget.test.ts --network ganache
 
* 
 */