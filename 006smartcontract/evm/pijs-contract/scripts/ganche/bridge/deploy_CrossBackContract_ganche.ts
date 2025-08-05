import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";

async function main(){
    const [owner,singer] = await ethers.getSigners();
    const CrossBackContract = await ethers.getContractFactory('CrossBackContract');

    const args = ['0xb08b8EAF87C21959a19985Ae127500cA4A556D9D',owner.address,owner.address,owner.address];

    // const crossBackContract_deploy = await upgrades.deployProxy(CrossBackContract,args,{kind:'uups'});
    const crossBackContract_deploy = await upgrades.upgradeProxy('0x7f31519A22E9f747bdFB455053eA064685AcCb1A', CrossBackContract, { kind: 'uups' });
    await crossBackContract_deploy.deployed();
    console.log("CrossBackContract contract address:",crossBackContract_deploy.address);

}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});



/***
 *  
 npx hardhat run ./scripts/ganche/bridge/deploy_CrossBackContract_ganche.ts --network ganache
 */