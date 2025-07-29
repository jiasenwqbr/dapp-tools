import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
async function main() {
    const [owner,singer] = await ethers.getSigners();
    const UnionBridgeSource = await ethers.getContractFactory('UnionBridgeSource');
    const args = [owner.address,owner.address,owner.address,owner.address,500];
    // const UnionBridgeSource_C = await upgrades.deployProxy(UnionBridgeSource,args,{ kind:'uups'});
    const UnionBridgeSource_C = await upgrades.upgradeProxy('0xDFB5d9c356Bd9F388559e665672757AB82C78E4c', UnionBridgeSource, { kind: 'uups' });
    await UnionBridgeSource_C.deployed();
    console.log("UnionBridgeSource  contract address:",UnionBridgeSource_C.address);

}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});


/***
 * 
 npx hardhat run ./scripts/pijs_testnet/bridge/deploy_UnionBridgeSource_testnet.ts --network pijstestnet
 */