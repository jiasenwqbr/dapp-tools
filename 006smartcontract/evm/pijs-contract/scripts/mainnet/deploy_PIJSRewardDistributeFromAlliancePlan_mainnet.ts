import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
async function main() {
    const [owner,singer] = await ethers.getSigners();
    const PIJSRewardDistribute = await ethers.getContractFactory('PIJSRewardDistributeFromAlliancePlan');
    // const args = [singer.address];
    const args = ["0xa616ab85b262a5645fa2ca08d12fd7b0bb64efef"];
    const PIJSRewardDistribute_C = await upgrades.deployProxy(PIJSRewardDistribute,args,{ kind:'uups'});
    // const PIJSRewardDistribute_C = await upgrades.upgradeProxy('0x41Fa8d95cAf846a0f5fdf758219B7B399968db38', PIJSRewardDistribute, { kind: 'uups' });
    await PIJSRewardDistribute_C.deployed();
    console.log("PIJSRewardDistributeFromAlliancePlan contract address:",PIJSRewardDistribute_C.address);

    




     

}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

/***
 * 
 npx hardhat run ./scripts/pijs_testnet/deploy_PIJSRewardDistributeFromAlliancePlan_mainnet.ts --network pijs
网络：
PIJS测试网络
合约名称：
PIJSRewardDistributeFromAlliancePlan
地址：

签名地址：
0xa616ab85b262a5645fa2ca08d12fd7b0bb64efef

 */