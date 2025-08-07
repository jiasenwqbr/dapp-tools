import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";

async function main(){
    const [owner,singer] = await ethers.getSigners();
    const CrossBackContract = await ethers.getContractFactory('CrossBackContract');

    const args = ['0xCB25B99ffe4f4B3d28DB5ABaC7D7d53005d4A5Dd',owner.address,'0xd4f0f0c79a35f217e5de4bff0752ba63cbc013e9',owner.address];

    const crossBackContract_deploy = await upgrades.deployProxy(CrossBackContract,args,{kind:'uups'});
    // const crossBackContract_deploy = await upgrades.upgradeProxy('0x2bFc439a9c9b98C1266836cC057Bef8D27a2A053', CrossBackContract, { kind: 'uups' });
    await crossBackContract_deploy.deployed();
    console.log("CrossBackContract contract address:",crossBackContract_deploy.address);

}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});



/***
 *  0x2bFc439a9c9b98C1266836cC057Bef8D27a2A053
 npx hardhat run ./scripts/pijs_testnet/bridge/deploy_CrossBackContract_testnet.ts --network pijstestnet
 */