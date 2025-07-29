import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
async function main() {
    const [owner, user ,receiver,operator] = await ethers.getSigners();
    const pIJSBridgeTarget = await ethers.getContractFactory('PIJSBridgeTarget');
    let feePercent = 500;
    const args = [
                    "UAC",
                    "UAC",
                    owner.address,
                    operator.address,
                    receiver.address,
                    feePercent,
                    owner.address,
                ];
    const pIJSBridgeTarget_C = await upgrades.deployProxy(pIJSBridgeTarget,args,{ kind:'uups'});
    // const pIJSBridgeTarget_C = await upgrades.upgradeProxy('0xDFB5d9c356Bd9F388559e665672757AB82C78E4c', pIJSBridgeTarget, { kind: 'uups' });
    await pIJSBridgeTarget_C.deployed();
    console.log("UnionBridgeSource  contract address:",pIJSBridgeTarget_C.address);

}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});


/***
 * 
 npx hardhat run ./scripts/ganche/bridge/deploy_PIJSBridgeTarget_ganche.ts --network ganache
 */