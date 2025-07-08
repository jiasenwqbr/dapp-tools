import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
async function main() {
    const [owner,singer] = await ethers.getSigners();
    const PIJSOrder = await ethers.getContractFactory('PIJSOrderV1');
    // const args = [singer.address];
     const args = ["0xa616ab85b262a5645fa2ca08d12fd7b0bb64efef"];
    // const PIJSOrder_C = await upgrades.deployProxy(PIJSOrder,args,{ kind:'uups'});
    const PIJSOrder_C = await upgrades.upgradeProxy('0x5F271D56Cb6Cdc8cfe45E0d65d69A27B9a928a8a', PIJSOrder, { kind: 'uups' });
    await PIJSOrder_C.deployed();
    console.log("pijs oder contract address:",PIJSOrder_C.address);






}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
/***
 * 
npx hardhat run ./scripts/mainnet/deploy_PIJSOrderV1_mainnet.ts --network pijs


PIJSOrderV1 0x5F271D56Cb6Cdc8cfe45E0d65d69A27B9a928a8a
PIJSRewardDistribute 0xE0521b37ebE80CC8983ada7B0DE278d96557FBAD
 
 */


