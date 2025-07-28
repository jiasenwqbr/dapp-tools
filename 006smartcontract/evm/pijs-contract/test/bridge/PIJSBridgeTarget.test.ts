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
                caller:operator,
                to:user,
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
            







        });
      });





});