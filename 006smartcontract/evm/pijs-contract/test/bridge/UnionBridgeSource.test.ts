import { expect } from "chai";
import { ethers, upgrades } from "hardhat";
import { network } from "hardhat";
import { UnionBridgeSource } from  "../../typechain-types";

describe("UnionBridgeSource", () => {
    let unionBridgeSourceContract:UnionBridgeSource;
    let owner: any;
    let user1: any;
    let user2: any;
    let user3: any;

    beforeEach(async () => {
        [owner, user1 ,user2, user3] = await ethers.getSigners();
        const unionBridgeSourceFactory =  await ethers.getContractFactory("UnionBridgeSource");
        unionBridgeSourceContract = (await upgrades.deployProxy(unionBridgeSourceFactory,[owner.address,owner.address,owner.address,owner.address,500],{
            initializer: "initialize",
        })) as UnionBridgeSource;
        await unionBridgeSourceContract.deployed();


        const TestERC20 = await ethers.getContractFactory("TestERC20");
        const token = await TestERC20.deploy(owner.address); // 初始全部转给 deployer
        await token.deployed();
        console.log("TestERC20 deployed at:", token.address);

        const deployerBalance = await token.balanceOf(owner.address);
        console.log("Deployer token balance:", ethers.utils.formatEther(deployerBalance));

        // send 100 eth to user1
        const amount = ethers.utils.parseEther("100");
        const tx = await token.tranfer(user1.address,amount);
        await tx.wait();




    });

    describe("Initialization",() => {
        it("shold owner has the role of manage",async () => {
            const  hasManageRole =  await unionBridgeSourceContract.hasRole(
                await unionBridgeSourceContract.MANAGE_ROLE(),
                owner.address
            );
            expect(hasManageRole).to.be.true;
        });
    });

    describe("depositeUNI",() => {
        it("depost UNI value shoud be correct", async () => {
            const params = {
                userAddr:owner.address,
                receiver:unionBridgeSourceContract.address,
                amount:ethers.utils.parseEther("1"),
                orderId:1,
                chainId:(await ethers.provider.getNetwork()).chainId
            };

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
                verifyingContract: unionBridgeSourceContract.address,
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
                    params.userAddr,
                    params.receiver,
                    params.amount,
                    params.orderId,
                    params.chainId,
                    signature
                ]
            );
            const ownerBeforeDepositeUNIBalance = await ethers.provider.getBalance(owner.address);
            await unionBridgeSourceContract.connect(owner).depositeUNI(data,{
                value: ethers.utils.parseEther("1"), 
                gasLimit: 1_000_000,
            });
           

            expect(await unionBridgeSourceContract.balance(ethers.constants.AddressZero)).to.equal(ethers.utils.parseEther("1"));
            const balance = await ethers.provider.getBalance(unionBridgeSourceContract.address);
            expect(balance).to.equal(ethers.utils.parseEther("1"));
            const ownerAfterDepositeUNIBalance = await ethers.provider.getBalance(owner.address);

            console.log("before desposite:",ownerBeforeDepositeUNIBalance.toString());
            console.log("aftter desposite:",ownerAfterDepositeUNIBalance.toString());
            expect(ownerBeforeDepositeUNIBalance.gt(ownerAfterDepositeUNIBalance.add(ethers.utils.parseEther("1")))).to.be.true;
            const gasUsed = ownerBeforeDepositeUNIBalance.sub(ownerAfterDepositeUNIBalance).sub(ethers.utils.parseEther("1"));
            console.log("the gas used:",gasUsed.toString());
        });
    });

    describe("desposite erc20",() => {
        it("depost ERC20 value shoud be correct",async () => {
            


        });
    });
    





});


/**
 npx hardhat test ./test/bridge/UnionBridgeSource.test.ts --network ganache
 */