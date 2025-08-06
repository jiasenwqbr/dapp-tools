import { expect } from "chai";
import { ethers, upgrades } from "hardhat";
import { network } from "hardhat";
import {CrossChainLock} from "../../typechain-types";

describe("CrossChainLock",()=> {
    let crossChainLock:CrossChainLock;
    let owner:any;
    let feeReceiver:any;
    let user1: any;
    let user2: any;
    let user3: any;
    let token:any;
    let feeDenominator = 10000;
    let feePercent = 500;
    beforeEach(async () => {
        [owner, feeReceiver,user1 ,user2, user3] = await ethers.getSigners();
        const crossChainLockFactory = await ethers.getContractFactory("CrossChainLock");
        crossChainLock = (
            await upgrades.deployProxy(crossChainLockFactory,[feeReceiver.address,owner.address,owner.address,500],{initializer:"initialize"})
        ) as CrossChainLock;
        await crossChainLock.deployed();

        const TestERC20 = await ethers.getContractFactory("TestERC20");
        token = await TestERC20.deploy(owner.address); // 初始全部转给 deployer
        await token.deployed();
        const deployerBalance = await token.balanceOf(owner.address);

        // send 100 eth to user1
        const amount = ethers.utils.parseEther("100");
        const tx = await token.transfer(user1.address,amount);
        await tx.wait();
        const deployerBalance1 = await token.balanceOf(owner.address);
        //console.log("Deployer token balance:", ethers.utils.formatEther(deployerBalance1));

        const user1Balance =  await token.balanceOf(user1.address);
        //console.log("user1 balance:", ethers.utils.formatEther(user1Balance));
    });

    describe("Initialization",() => {
        it("shold owner has the role of manage",async () => {
            const  hasManageRole =  await crossChainLock.hasRole(
                await crossChainLock.MANAGE_ROLE(),
                owner.address
            );
            expect(hasManageRole).to.be.true;
        });
    });

    describe("depositeUNI",() => {
        it("deposite UNI value should be correct", async () => {
            var params = {
                userAddr:owner.address,
                receiver:crossChainLock.address,
                amount:ethers.utils.parseEther("1"),
                orderId:1,
                chainId:(await ethers.provider.getNetwork()).chainId
            }
            const types = {
                Permit: [
                    { name: "userAddr", type: "address" },
                    { name: "receiver", type: "address" },
                    { name: "amount", type: "uint256" },
                    { name: "orderId", type: "uint256" },
                    { name: "chainId", type: "uint256" },
                ],
            };
            const domain = {
                name: "UnionBridgeSource",
                version: "1",
                chainId: (await ethers.provider.getNetwork()).chainId,
                verifyingContract: crossChainLock.address,
            };
            const signature = await owner._signTypedData(domain, types, params);

            const data = ethers.utils.defaultAbiCoder.encode(
                [
                   "address",
                   "address",
                   "uint256",
                   "uint256", 
                   "uint256", 
                   "uint256",
                   "bytes"
                ],[
                    params.userAddr,
                    params.receiver,
                    params.amount,
                    1,
                    params.orderId,
                    params.chainId,
                    signature
                ]
            );

            const ownerBeforeDepositeUNIBalance = await ethers.provider.getBalance(owner.address);
            const balanceOfTheContractBeforeDeposite = await ethers.provider.getBalance(crossChainLock.address);
            console.log("balanceOfTheContractBeforeDeposite:",balanceOfTheContractBeforeDeposite);
            const tx = await crossChainLock.connect(owner).depositeUNI(data,{
                value: ethers.utils.parseEther("1"), 
                gasLimit: 1_000_000,
            });
            const receipt = await tx.wait();
           

            expect(await crossChainLock.balance(ethers.constants.AddressZero)).to.equal(ethers.utils.parseEther("1"));
            const balance = await ethers.provider.getBalance(crossChainLock.address);
            expect(balance).to.equal(ethers.utils.parseEther("1"));
            const ownerAfterDepositeUNIBalance = await ethers.provider.getBalance(owner.address);

            console.log("before desposite:",ownerBeforeDepositeUNIBalance.toString());
            console.log("aftter desposite:",ownerAfterDepositeUNIBalance.toString());
            expect(ownerBeforeDepositeUNIBalance.gt(ownerAfterDepositeUNIBalance.add(ethers.utils.parseEther("1")))).to.be.true;
            const gasUsed = ownerBeforeDepositeUNIBalance.sub(ownerAfterDepositeUNIBalance).sub(ethers.utils.parseEther("1"));
            console.log("the gas used:",gasUsed.toString());

            const balanceOfTheContractAfterDeposite = await ethers.provider.getBalance(crossChainLock.address);
            console.log("balanceOfTheContractAfterDeposite:",balanceOfTheContractAfterDeposite);

            expect(balanceOfTheContractAfterDeposite).equal(ethers.utils.parseEther("1"));


            const event = receipt.events?.find((e) => e.event === "DepositeUNI");
            expect(event).to.not.be.undefined;
            if (event) {
                expect(event?.args?.caller).to.equal(owner.address);
                expect(event?.args?.amount).to.equal(ethers.utils.parseEther("1"));
                expect(event?.args?.receiver).to.equal(crossChainLock.address);
            }
        });
    });

    describe("withdrawUNI",()=>{
        beforeEach(async () => {
            var params = {
                userAddr:owner.address,
                receiver:crossChainLock.address,
                amount:ethers.utils.parseEther("1"),
                orderId:1,
                chainId:(await ethers.provider.getNetwork()).chainId
            }
            const types = {
                Permit: [
                    { name: "userAddr", type: "address" },
                    { name: "receiver", type: "address" },
                    { name: "amount", type: "uint256" },
                    { name: "orderId", type: "uint256" },
                    { name: "chainId", type: "uint256" },
                ],
            };
            const domain = {
                name: "UnionBridgeSource",
                version: "1",
                chainId: (await ethers.provider.getNetwork()).chainId,
                verifyingContract: crossChainLock.address,
            };
            const signature = await owner._signTypedData(domain, types, params);

            const data = ethers.utils.defaultAbiCoder.encode(
                [
                   "address",
                   "address",
                   "uint256",
                   "uint256", 
                   "uint256", 
                   "uint256",
                   "bytes"
                ],[
                    params.userAddr,
                    params.receiver,
                    params.amount,
                    1,
                    params.orderId,
                    params.chainId,
                    signature
                ]
            );

            const ownerBeforeDepositeUNIBalance = await ethers.provider.getBalance(owner.address);
            const balanceOfTheContractBeforeDeposite = await ethers.provider.getBalance(crossChainLock.address);
            console.log("balanceOfTheContractBeforeDeposite:",balanceOfTheContractBeforeDeposite);
            const tx = await crossChainLock.connect(owner).depositeUNI(data,{
                value: ethers.utils.parseEther("1"), 
                gasLimit: 1_000_000,
            });
        });
        it("withdrawUNI the balance of user and receiver is correct",async () => {
            const params = {
                caller:owner.address,
                fee:ethers.utils.parseEther("0.1"),
                amount:ethers.utils.parseEther("1"),
                userAddr:owner.address,
                orderId:1,
                chainId:(await ethers.provider.getNetwork()).chainId
            };
            const types = {
                Permit: [
                    { name: "caller", type: "address" },
                    { name: "fee", type: "uint256" },
                    { name: "amount", type: "uint256" },
                    { name:"userAddr",type: "address"},
                    { name: "orderId", type: "uint256" },
                    { name: "chainId", type: "uint256" },
                ],
            };
            const domain = {
                name: "UnionBridgeSource",
                version: "1",
                chainId: (await ethers.provider.getNetwork()).chainId,
                verifyingContract: crossChainLock.address,
            };
            const signature = await owner._signTypedData(domain, types, params);
            const data = ethers.utils.defaultAbiCoder.encode(
                [
                   "address",
                   "uint256",
                   "address",
                   "uint256", 
                   "uint256", 
                   "uint256",
                   "uint256",
                   "bytes"
                ],[
                    params.caller,
                    params.amount,
                    params.userAddr,
                    params.fee,
                    1,
                    params.orderId,
                    params.chainId,
                    signature
                ]
            );

            const beforeOwnerBanlance = await  ethers.provider.getBalance(owner.address);
            const beforeReceiverBalance = await  ethers.provider.getBalance(feeReceiver.address);
            const beforeContractBanalce = await crossChainLock.balance(ethers.constants.AddressZero);

            console.log("beforeOwnerBanlance:",beforeOwnerBanlance);
            console.log("beforeReceiverBalance:",beforeReceiverBalance);
             console.log("beforeContractBanalce:",beforeContractBanalce);

            const tx = await crossChainLock.connect(owner).withdrawUNI(data,{
                gasLimit: 1_000_000
            });

            const afterOwnerBanlance = await  ethers.provider.getBalance(owner.address);
            const afterReceiverBalance = await  ethers.provider.getBalance(feeReceiver.address);
            const afterContractBanalce = await crossChainLock.balance(ethers.constants.AddressZero);

            console.log("afterOwnerBanlance:",afterOwnerBanlance);
            console.log("afterReceiverBalance:",afterReceiverBalance);
            console.log("afterContractBanalce:",afterContractBanalce);

            expect(afterReceiverBalance).to.equal(beforeReceiverBalance.add(ethers.utils.parseEther("0.1")));
            expect(afterContractBanalce).to.equal(beforeContractBanalce.sub(ethers.utils.parseEther("1")));
        });
    });

    


});

/**
 npx hardhat test ./test/bridge/CrossChainLock.test.ts --network ganache
 */