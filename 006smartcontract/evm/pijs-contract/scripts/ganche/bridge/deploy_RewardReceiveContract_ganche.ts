import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
async function mian(){
    const [owner,singer] = await ethers.getSigners();
    const RewardReceiveContract = await ethers.getContractFactory('RewardReceiveContract');
    const args = [owner.address];

    // const RewardReceiveContract_deploy = await upgrades.deployProxy(RewardReceiveContract,args,{kind:'uups'});
    const RewardReceiveContract_deploy = await upgrades.upgradeProxy('0xA1d7e049a86e094e00215448C8Dd086191F98438', RewardReceiveContract, { kind: 'uups' });
    
    await RewardReceiveContract_deploy.deployed();

    console.log("StakingUAC address is:",RewardReceiveContract_deploy.address);


}

mian().catch(error => {
    console.error(error);
    process.exitCode = 1;
});

/***
 * 
 npx hardhat run ./scripts/ganche/bridge/deploy_RewardReceiveContract_ganche.ts --network ganache
 */