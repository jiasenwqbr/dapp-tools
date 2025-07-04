import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
async function main() {
    const [owner,singer] = await ethers.getSigners();
    const PIJSRewardDistribute = await ethers.getContractFactory('PIJSRewardDistribute');
    const args = [singer.address];
    // const PIJSRewardDistribute_C = await upgrades.deployProxy(PIJSRewardDistribute,args,{ kind:'uups'});
    const PIJSRewardDistribute_C = await upgrades.upgradeProxy('0xB2Cc0C883Ebd8b310E2dc7d3Fabf419fd4c8C478', PIJSRewardDistribute, { kind: 'uups' });
    await PIJSRewardDistribute_C.deployed();
    console.log("pijs oder contract address:",PIJSRewardDistribute_C.address);

}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});


/***
 * 
 npx hardhat run ./scripts/pijs_testnet/deploy_PIJSRewardDistribute_testnet.ts --network pijstestnet
 */