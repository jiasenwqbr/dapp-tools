import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";

async function main(){
    const [owner] = await ethers.getSigners();
    // const receiver = owner;
    const receiver = '0xBB5EAccCEB5CBCfBD73d8Fb6bBd122eACa47ae37';
    const singer = '0xd4f0f0c79a35f217e5de4bff0752ba63cbc013e9';
    const args = [receiver,singer,owner.address,500];
    const CrossChainLock =  await ethers.getContractFactory('CrossChainLockTest');
    const crossChain_deploy =  await upgrades.deployProxy(CrossChainLock,args,{kind:'uups'});
    await crossChain_deploy.deployed();
    console.log("CrossChainLockTest address is : ",crossChain_deploy.address);
}


main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});


/***
 * 0x3D436e3503B40a2c73D0EA70ab407405aDaf13d5
 npx hardhat run ./scripts/pijs_testnet/bridge/deploy_CrossChainLock_uni.ts --network uac
 */