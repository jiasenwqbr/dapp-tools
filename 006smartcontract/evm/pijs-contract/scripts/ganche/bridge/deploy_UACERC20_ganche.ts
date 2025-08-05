import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";

async function main(){
    const [owner,singer,feeReceiver] = await ethers.getSigners();
    const UACERC20 = await ethers.getContractFactory('UACERC20');

    const args = ['UAC','UAC',owner.address,owner.address,feeReceiver.address,500,owner.address];
    // const UACERC20_deploy =  await upgrades.deployProxy(UACERC20,args,{kind:'uups'});
    const UACERC20_deploy = await upgrades.upgradeProxy('0xb08b8EAF87C21959a19985Ae127500cA4A556D9D',UACERC20,{ kind:'uups'});
    await UACERC20_deploy.deployed();
    console.log("UACERC20 contract address:",UACERC20_deploy.address);
}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});

/***
 * 
 npx hardhat run ./scripts/ganche/bridge/deploy_UACERC20_ganche.ts --network ganache
 */