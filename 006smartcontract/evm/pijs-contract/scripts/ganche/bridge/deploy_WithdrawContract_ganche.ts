import {id} from "ethers/lib/utils";
import {ethers,upgrades } from "hardhat";

async function mian(){
    const [owner,receiver] = await ethers.getSigners();
    const args = ['0xb08b8EAF87C21959a19985Ae127500cA4A556D9D',owner.address,owner.address,owner.address];
    const WithdrawContract =   await ethers.getContractFactory('WithdrawContract');
    const WithdrawContract_deploy = await upgrades.deployProxy(WithdrawContract,args,{ kind:'uups'});
    //const PIJSOrder_C = await upgrades.upgradeProxy('0xA0ACe7397B58fCF5E56B1f3d6Ed14d8Fd9008A8e', WithdrawContract, { kind: 'uups' });

    await WithdrawContract_deploy.deployed();
    console.log("WithdrawContract address is : ",WithdrawContract_deploy.address);



}


mian().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});


/***
 * 
 npx hardhat run ./scripts/ganche/bridge/deploy_WithdrawContract_ganche.ts --network ganache
 */