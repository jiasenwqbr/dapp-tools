import { expect } from "chai";
import { ethers, upgrades } from "hardhat";
import { network } from "hardhat";
import {StakingUAC,UACERC20,TestERC20} from "../../typechain-types";

describe("StakingUAC",()=>{
    let owner: any;
    let signer: any;
    let feeReceiver:any;
    let admin: any;
    let operator:any;
    let user1: any;
    let user2: any;
    let user3: any;
    let uac:any;
    let lp:any;
    let stakingUAC:any;

    beforeEach(async () => {
        [owner,feeReceiver,user1,user2,user3] = await ethers.getSigners();
        signer = owner;
        admin = owner;
        operator = owner;
        // lp
        const lpFactory = await ethers.getContractFactory("TestERC20");
        lp = await lpFactory.deploy(owner.address); // 初始全部转给 deployer
        await lp.deployed();
        console.log("TestERC20 deployed at:", lp.address);


        // deploy UAC
        const UACERC20Factory = await ethers.getContractFactory("UACERC20");
        uac = ( 
            await upgrades.deployProxy(UACERC20Factory,['UAC','UAC',admin.address,operator.address,feeReceiver.address,500,signer.address],{initializer:"initialize"})
        ) as UACERC20;
        await uac.deployed();
        // mint uac 至 owner
        uac.mint(owner.address,ethers.utils.parseEther("1000"));

        // deploy StakingUAC
        const stakingUACFactory = await ethers.getContractFactory("StakingUAC");
        stakingUAC = (
            await upgrades.deployProxy(stakingUACFactory,[uac.address,lp.address,owner.address],{initializer:"initialize"})
        ) as StakingUAC;
        await stakingUAC.deployed();


        
        

    });

    describe("stakeUAC",() => {
        it("stakeUAC,the balance of the contract is correct",async () => {
            const params = {
                orderId:1,
                userId:1,
                amount:ethers.utils.parseEther("100"),
                balanceSource:1,
                endTimestamp: Math.floor(Date.now() / 1000) + 3600,
                startTimestamp: Math.floor(Date.now() / 1000),
                renewable: 1,
            };
            const types = {
                Permit: [
                    { name: "orderId", type: "uint256" },
                    { name: "userId", type: "uint256" },
                    { name: "amount", type: "uint256" },
                    { name: "balanceSource", type: "uint24" },
                    { name: "endTimestamp", type: "uint256" },
                    { name: "startTimestamp", type: "uint256" },
                    { name: "renewable", type: "uint8" }
                ],
            };
            const domain = {
                name: "StakingUAC",
                version: "1",
                chainId: (await ethers.provider.getNetwork()).chainId,
                verifyingContract: stakingUAC.address,
            };
            // console.log("domain:",domain);
            const signature = await owner._signTypedData(domain, types, params);
            const data = ethers.utils.defaultAbiCoder.encode(
                [
                    "uint256",
                    "uint256",
                    "uint256",
                    "uint24",
                    "uint256",
                    "uint256",
                    "uint8",
                    "bytes",
                ],
                [
                    params.orderId,
                    params.userId,
                    params.amount,
                    params.balanceSource,
                    params.endTimestamp,
                    params.startTimestamp,
                    params.renewable,
                    signature,
                ]
            );

            const beforeStakeUACBalance = await uac.balanceOf(owner.address);
            const beforeContractUACBalance = await uac.balanceOf(uac.address);
            console.log("beforeStakeUACBalance:",beforeStakeUACBalance);
            console.log("beforeContractUACBalance:",beforeContractUACBalance);
            const allowance = await uac.allowance(owner.address, stakingUAC.address);
            console.log("Current allowance:", allowance.toString());
            await stakingUAC.connect(owner).stakeUAC(data,{
                gasLimit: 1_000_000,
            });

            const afterStakeUACBalance = await uac.balanceOf(owner.address);
            const afterContractUACBalance = await uac.balanceOf(uac.address);
            console.log("afterStakeUACBalance:",afterStakeUACBalance);
            console.log("afterContractUACBalance:",afterContractUACBalance);

            expect(afterContractUACBalance.sub(params.amount)).to.equal(beforeContractUACBalance);






        });
    });

});


/**
 * 
  npx hardhat test ./test/bridge/StakingUAC.test.ts --network ganache
 */