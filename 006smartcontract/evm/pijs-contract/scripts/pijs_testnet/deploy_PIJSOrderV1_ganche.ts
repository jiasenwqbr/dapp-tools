import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
async function main() {
    const [owner,singer] = await ethers.getSigners();
    const PIJSOrder = await ethers.getContractFactory('PIJSOrderV1');
    const args = [singer.address];
    //const PIJSOrder_C = await upgrades.deployProxy(PIJSOrder,args,{ kind:'uups'});
    const PIJSOrder_C = await upgrades.upgradeProxy('0x60bb6983a38e12eeD68d4c0ded9a9E726f791a6d', PIJSOrder, { kind: 'uups' });
    await PIJSOrder_C.deployed();
    console.log("pijs oder contract address:",PIJSOrder_C.address);

}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});


/***
 * 
 npx hardhat run ./scripts/pijs_testnet/deploy_PIJSOrderV1_ganche.ts --network pijstestnet

 PIJSOrderV1 0x60bb6983a38e12eeD68d4c0ded9a9E726f791a6d
 PIJSRewardDistribute 0xd6111257AD67aB25Ce2949C8E59d1E0372EfcBD9
 */