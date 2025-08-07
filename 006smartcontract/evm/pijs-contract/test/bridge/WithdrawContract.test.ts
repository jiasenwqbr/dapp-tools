import { expect } from "chai";
import { ethers,upgrades,network } from "hardhat";
import { WithdrawContract,UACERC20 } from "../../typechain-types";

describe("WithdrawContract test",() => {
    let uac:UACERC20;
    let withDrawContract:WithdrawContract;
    let operator:any;
    let signer:any;
    let admin:any;
    let owner:any;
    let feeReceiver:any;
    let user1: any;
    let user2: any;
    let user3: any;
    beforeEach(async () => {
        [owner,feeReceiver,user1,user2,user3] = await ethers.getSigners();
        operator = owner;
        signer = owner;
        admin = owner;
        // deploy UAC
        const UACERC20Factory = await ethers.getContractFactory("UACERC20");
        uac = ( 
            await upgrades.deployProxy(UACERC20Factory,['UAC','UAC',admin.address,operator.address,feeReceiver.address,500,signer.address],{initializer:"initialize"})
        ) as UACERC20;
        await uac.deployed();

        // deploy WithDrawContract
        const WithDrawContractFactory =  await ethers.getContractFactory("WithdrawContract");
        withDrawContract = (
            await upgrades.deployProxy(WithDrawContractFactory,[uac.address,operator.address,signer.address,admin.address],{initializer:"initialize"})
        ) as WithdrawContract;
        await withDrawContract.deployed();

    });
    describe("mint UAC Uint test",() => {
         it("mint UAC",async () => {
            const params = {
                caller : operator.address,
                withdrawContract:withDrawContract.address,
                amount:ethers.utils.parseEther("100"),
                fee:ethers.utils.parseEther("10"),
                orderId:1,
                chainId:(await ethers.provider.getNetwork()).chainId
            };
            const types = {
                Permit: [
                    { name: "caller", type: "address" },
                    { name: "withdrawContract", type: "address" },
                    { name: "amount", type: "uint256" },
                    { name: "fee", type: "uint256" },
                    { name: "orderId", type: "uint256" },
                    { name: "chainId", type: "uint256" },
                ],
            }
            const domain = {
                name: "PIJSBridgeTarget",
                version: "1",
                chainId: (await ethers.provider.getNetwork()).chainId,
                verifyingContract: uac.address,
            };
            const signature = await owner._signTypedData(domain,types,params);
            const data =  ethers.utils.defaultAbiCoder.encode(
                [
                    "address",
                    "address",
                    "uint256",
                    "uint256", 
                    "uint256",
                    "uint256",
                    "bytes"
                ],[
                    params.caller,
                    params.withdrawContract,
                    params.amount,
                    params.fee,
                    params.orderId,
                    params.chainId,
                    signature
                ]
            );


            // before mint
            const beforeFeeReceiverBalance = await uac.balanceOf(feeReceiver.address);
            const beforeWithDrawContracBalance =  await uac.balanceOf(withDrawContract.address);    
            console.log("beforeFeeReceiverBalance:",beforeFeeReceiverBalance);
            console.log("beforeWithDrawContracBalance:",beforeWithDrawContracBalance);

            await uac.connect(owner).mintToken(data,{
                    gasLimit: 1_000_000,
            });

            const afterFeeReceiverBalance = await uac.balanceOf(feeReceiver.address);
            const afterWithDrawContracBalance =  await uac.balanceOf(withDrawContract.address);    
            console.log("afterFeeReceiverBalance:",afterFeeReceiverBalance);
            console.log("afterWithDrawContracBalance:",afterWithDrawContracBalance);

            expect(afterFeeReceiverBalance.sub(beforeFeeReceiverBalance)).to.equal(ethers.utils.parseEther("10"));
            expect(afterWithDrawContracBalance.sub(beforeWithDrawContracBalance)).to.equal(ethers.utils.parseEther("90"));
        });
    });

    describe("WithdrawContract",() => {
        beforeEach(async () => {
             const params = {
                caller : operator.address,
                withdrawContract:withDrawContract.address,
                amount:ethers.utils.parseEther("100"),
                fee:ethers.utils.parseEther("10"),
                orderId:1,
                chainId:(await ethers.provider.getNetwork()).chainId
            };
            const types = {
                Permit: [
                    { name: "caller", type: "address" },
                    { name: "withdrawContract", type: "address" },
                    { name: "amount", type: "uint256" },
                    { name: "fee", type: "uint256" },
                    { name: "orderId", type: "uint256" },
                    { name: "chainId", type: "uint256" },
                ],
            }
            const domain = {
                name: "PIJSBridgeTarget",
                version: "1",
                chainId: (await ethers.provider.getNetwork()).chainId,
                verifyingContract: uac.address,
            };
            const signature = await owner._signTypedData(domain,types,params);
            const data =  ethers.utils.defaultAbiCoder.encode(
                [
                    "address",
                    "address",
                    "uint256",
                    "uint256", 
                    "uint256",
                    "uint256",
                    "bytes"
                ],[
                    params.caller,
                    params.withdrawContract,
                    params.amount,
                    params.fee,
                    params.orderId,
                    params.chainId,
                    signature
                ]
            );


            // before mint
            const beforeFeeReceiverBalance = await uac.balanceOf(feeReceiver.address);
            const beforeWithDrawContracBalance =  await uac.balanceOf(withDrawContract.address);    
            console.log("beforeFeeReceiverBalance:",beforeFeeReceiverBalance);
            console.log("beforeWithDrawContracBalance:",beforeWithDrawContracBalance);

            await uac.connect(owner).mintToken(data,{
                    gasLimit: 1_000_000,
            });

            const afterFeeReceiverBalance = await uac.balanceOf(feeReceiver.address);
            const afterWithDrawContracBalance =  await uac.balanceOf(withDrawContract.address);    
            console.log("afterFeeReceiverBalance:",afterFeeReceiverBalance);
            console.log("afterWithDrawContracBalance:",afterWithDrawContracBalance);
        });

        describe("withdrawUAC",() => {
            it("withdrawUAC",async () => {
                const params = {
                    caller:owner.address,
                    to:user1.address,
                    amount:ethers.utils.parseEther("90"),
                    orderId:1,
                    chainId: (await ethers.provider.getNetwork()).chainId
                };
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
                    name: "WithdrawContract",
                    version: "1",
                    chainId: (await ethers.provider.getNetwork()).chainId,
                    verifyingContract: withDrawContract.address,
                };

                const signature = await owner._signTypedData(domain,types,params);
                const data =  ethers.utils.defaultAbiCoder.encode(
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

                const beforeUser1Balance = await uac.balanceOf(user1.address);
                const beforeWithDrawUACBalance =  await uac.balanceOf(withDrawContract.address);    
                console.log("beforeUser1Balance:",beforeUser1Balance);
                console.log("beforeWithDrawUACBalance:",beforeWithDrawUACBalance);

                await withDrawContract.connect(owner).withdrawUAC(data,{
                     gasLimit: 1_000_000,
                });

                const afterUser1Balance = await uac.balanceOf(user1.address);
                const afterWithDrawUACBalance =  await uac.balanceOf(withDrawContract.address);    
                console.log("afterUser1Balance:",afterUser1Balance);
                console.log("afterWithDrawUACBalance:",afterWithDrawUACBalance);

                expect(afterUser1Balance.sub(beforeUser1Balance)).to.equal(ethers.utils.parseEther("90"));
                expect(beforeWithDrawUACBalance.sub(afterWithDrawUACBalance)).to.equal(ethers.utils.parseEther("90"));






                








            });
        });
    });
   

});


/**
 * 
 npx hardhat test ./test/bridge/WithdrawContract.test.ts --network ganache
 */