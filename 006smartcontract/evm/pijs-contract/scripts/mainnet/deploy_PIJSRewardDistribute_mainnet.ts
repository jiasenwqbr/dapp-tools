import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
async function main() {
    const [owner,singer] = await ethers.getSigners();
    const PIJSRewardDistribute = await ethers.getContractFactory('PIJSRewardDistribute');
    // const args = [singer.address];
    const args = ["0xd4f0f0c79a35f217e5de4bff0752ba63cbc013e9"];
    // const PIJSRewardDistribute_C = await upgrades.deployProxy(PIJSRewardDistribute,args,{ kind:'uups'});
    const PIJSRewardDistribute_C = await upgrades.upgradeProxy('0xcaEA3388aA91AE0a6EA6a1A0BE7B9F797E41F3F0', PIJSRewardDistribute, { kind: 'uups' });
    await PIJSRewardDistribute_C.deployed();
    console.log("pijs oder contract address:",PIJSRewardDistribute_C.address);

    




     

}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

/***
 * 
 npx hardhat run ./scripts/mainnet/deploy_PIJSRewardDistribute_mainnet.ts --network mainnet
 */