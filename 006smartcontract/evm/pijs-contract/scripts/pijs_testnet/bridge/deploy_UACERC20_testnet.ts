import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";

async function main(){
    const [owner,singer,feeReceiver] = await ethers.getSigners();
    const UACERC20 = await ethers.getContractFactory('UACERC20');
    const signer = '0xd4f0f0c79a35f217e5de4bff0752ba63cbc013e9';
    const receiver = '0xBB5EAccCEB5CBCfBD73d8Fb6bBd122eACa47ae37';
    const args = ['UAC','UAC',owner.address,owner.address,receiver,500,signer];
    const UACERC20_deploy =  await upgrades.deployProxy(UACERC20,args,{kind:'uups'});
    //const UACERC20_deploy = await upgrades.upgradeProxy('0x06236DE9f60c80B756817a79d1a8202829CD3281',UACERC20,{ kind:'uups'});
    await UACERC20_deploy.deployed();
    console.log("UACERC20 contract address:",UACERC20_deploy.address);
}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});

/***
 * 0x06236DE9f60c80B756817a79d1a8202829CD3281
 npx hardhat run ./scripts/pijs_testnet/bridge/deploy_UACERC20_testnet.ts --network pijstestnet
 */