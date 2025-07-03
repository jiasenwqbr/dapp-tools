import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
async function main() {
    const [owner,singer] = await ethers.getSigners();
    const PIJSRewardDistribute = await ethers.getContractFactory('PIJSRewardDistribute');
    const args = [singer.address];
    // const PIJSRewardDistribute_C = await upgrades.deployProxy(PIJSRewardDistribute,args,{ kind:'uups'});
    const PIJSRewardDistribute_C = await upgrades.upgradeProxy('0xd6111257AD67aB25Ce2949C8E59d1E0372EfcBD9', PIJSRewardDistribute, { kind: 'uups' });
    await PIJSRewardDistribute_C.deployed();
    console.log("pijs oder contract address:",PIJSRewardDistribute_C.address);

}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});


/***
 * 
 npx hardhat run ./scripts/deploy_PIJSRewardDistribute_ganche.ts --network ganache
 */