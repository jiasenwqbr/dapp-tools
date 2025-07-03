import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
async function main() {
    const [owner,singer] = await ethers.getSigners();
    const PIJSOrder = await ethers.getContractFactory('PIJSOrderV1');
    const args = [singer.address];
    //const PIJSOrder_C = await upgrades.deployProxy(PIJSOrder,args,{ kind:'uups'});
    const PIJSOrder_C = await upgrades.upgradeProxy('0xA0ACe7397B58fCF5E56B1f3d6Ed14d8Fd9008A8e', PIJSOrder, { kind: 'uups' });
    await PIJSOrder_C.deployed();
    console.log("pijs oder contract address:",PIJSOrder_C.address);

}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});


/***
 * 
 npx hardhat run ./scripts/deploy_PIJSOrderV1_ganche.ts --network ganache
 */