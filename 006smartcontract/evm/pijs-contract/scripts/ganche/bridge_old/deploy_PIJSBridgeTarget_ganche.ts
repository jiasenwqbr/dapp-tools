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
    // const pIJSBridgeTarget_C = await upgrades.deployProxy(pIJSBridgeTarget,args,{ kind:'uups'});
    const pIJSBridgeTarget_C = await upgrades.upgradeProxy('0xe0C354d9ED7e34C8cC78843D70451F7016901Bf8', pIJSBridgeTarget, { kind: 'uups' });
    await pIJSBridgeTarget_C.deployed();
    console.log("UnionBridgeSource  contract address:",pIJSBridgeTarget_C.address);

}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});


/***
 * 
 npx hardhat run ./scripts/ganche/bridge_old/deploy_PIJSBridgeTarget_ganche.ts --network ganache

 
 */