import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
async function main() {
    const [owner,singer] = await ethers.getSigners();
    const PIJSRewardDistribute = await ethers.getContractFactory('PIJSRewardDistribute');
    // const args = [singer.address];
    const args = ["0xa616ab85b262a5645fa2ca08d12fd7b0bb64efef"];
    const PIJSRewardDistribute_C = await upgrades.deployProxy(PIJSRewardDistribute,args,{ kind:'uups'});
    // const PIJSRewardDistribute_C = await upgrades.upgradeProxy('0x6a744ED46E6C4Dc67c258aB87Adc9711c57E2BDB', PIJSRewardDistribute, { kind: 'uups' });
    await PIJSRewardDistribute_C.deployed();
    console.log("pijs oder contract address:",PIJSRewardDistribute_C.address);

    




     

}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

/***
 * 0x6a744ED46E6C4Dc67c258aB87Adc9711c57E2BDB
 npx hardhat run ./scripts/mainnet/deploy_PIJSRewardDistribute_mainnet_2.ts --network pijs
 */