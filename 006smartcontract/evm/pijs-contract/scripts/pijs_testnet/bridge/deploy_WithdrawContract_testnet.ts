import {id} from "ethers/lib/utils";
import {ethers,upgrades } from "hardhat";

async function mian(){
    const [owner,receiver] = await ethers.getSigners();
    const args = ['0xb08b8EAF87C21959a19985Ae127500cA4A556D9D',owner.address,'0xd4f0f0c79a35f217e5de4bff0752ba63cbc013e9',owner.address];
    const WithdrawContract =   await ethers.getContractFactory('WithdrawContract');
    const WithdrawContract_deploy = await upgrades.deployProxy(WithdrawContract,args,{ kind:'uups'});
    //const PIJSOrder_C = await upgrades.upgradeProxy('0x0036c3465A118B8429dD796AA18201e9D1EEE0C9', WithdrawContract, { kind: 'uups' });

    await WithdrawContract_deploy.deployed();
    console.log("WithdrawContract address is : ",WithdrawContract_deploy.address);



}


mian().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});


/***
 * 0x0036c3465A118B8429dD796AA18201e9D1EEE0C9
 npx hardhat run ./scripts/pijs_testnet/bridge/deploy_WithdrawContract_testnet.ts --network pijstestnet
 */