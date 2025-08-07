import {id} from "ethers/lib/utils";
import {ethers,upgrades} from "hardhat"

async function main(){
    const [owner,signer] = await ethers.getSigners(); 
    const args = [owner.address];
    const UAG = await ethers.getContractFactory("UAG");
    const UAG_deploy = await UAG.deploy(owner.address);
    await UAG_deploy.deployed();
    console.log("UAG address is : ",UAG_deploy.address);
}

main().catch(error => {
    console.error(error);
    process.exitCode = 1;
});

/***
 * 
 npx hardhat run ./scripts/pijs_testnet/bridge/deploy_UAGToken_testnet.ts --network pijstestnet
 */