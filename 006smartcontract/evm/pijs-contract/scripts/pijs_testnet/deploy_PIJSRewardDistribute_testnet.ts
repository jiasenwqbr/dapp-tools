import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
async function main() {
    const [owner,singer] = await ethers.getSigners();
    const PIJSRewardDistribute = await ethers.getContractFactory('PIJSRewardDistribute');
    // const args = [singer.address];
    const args = ["0xd4f0f0c79a35f217e5de4bff0752ba63cbc013e9"];
    // const PIJSRewardDistribute_C = await upgrades.deployProxy(PIJSRewardDistribute,args,{ kind:'uups'});
    const PIJSRewardDistribute_C = await upgrades.upgradeProxy('0xE203657fA5596fB264ACCFb38981Ae83b98BC1a9', PIJSRewardDistribute, { kind: 'uups' });
    await PIJSRewardDistribute_C.deployed();
    console.log("PIJSRewardDistribute contract address:",PIJSRewardDistribute_C.address);

    




     

}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

/***
 * 
 npx hardhat run ./scripts/pijs_testnet/deploy_PIJSRewardDistribute_testnet.ts --network pijstestnet
 0xd4f0f0c79a35f217e5de4bff0752ba63cbc013e9
 0xd4f0f0c79a35f217e5de4bff0752ba63cbc013e9

 */