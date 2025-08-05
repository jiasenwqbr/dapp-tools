import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";

async function main(){
    const [owner,receiver] = await ethers.getSigners();
    const args = [receiver.address,owner.address,owner.address,500];
    const CrossChainLock =  await ethers.getContractFactory('CrossChainLock');
    const crossChain_deploy =  await upgrades.deployProxy(CrossChainLock,args,{kind:'uups'});
    await crossChain_deploy.deployed();
    console.log("crosschainlock address is : ",crossChain_deploy.address);
}


main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});


/***
 * 
 npx hardhat run ./scripts/ganche/bridge/deploy_CrossChainLock_ganche.ts --network ganache
 */