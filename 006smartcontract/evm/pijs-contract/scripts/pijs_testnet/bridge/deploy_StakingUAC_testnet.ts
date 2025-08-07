import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
async function mian(){
    const [owner,singer] = await ethers.getSigners();
    const StakingUAC = await ethers.getContractFactory('StakingUAC');
    const args = ['0xb08b8EAF87C21959a19985Ae127500cA4A556D9D',owner.address];

    // const StakingUAC_deploy = await upgrades.deployProxy(StakingUAC,args,{kind:'uups'});
    const StakingUAC_deploy = await upgrades.upgradeProxy('0x3fcCc2674945A3111F4eFCEFaC3508df91f36624', StakingUAC, { kind: 'uups' });
    
    await StakingUAC_deploy.deployed();

    console.log("StakingUAC address is:",StakingUAC_deploy.address);


}

mian().catch(error => {
    console.error(error);
    process.exitCode = 1;
});

/***
 * 
 npx hardhat run ./scripts/pijs_testnet/bridge/deploy_StakingUAC_testnet.ts --network pijstestnet
 */