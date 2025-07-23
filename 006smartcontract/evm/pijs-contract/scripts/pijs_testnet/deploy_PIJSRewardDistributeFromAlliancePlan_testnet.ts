import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
async function main() {
    const [owner,singer] = await ethers.getSigners();
    const PIJSRewardDistribute = await ethers.getContractFactory('PIJSRewardDistributeFromAlliancePlan');
    // const args = [singer.address];
    const args = ["0xd4f0f0c79a35f217e5de4bff0752ba63cbc013e9"];
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
 npx hardhat run ./scripts/pijs_testnet/deploy_PIJSRewardDistributeFromAlliancePlan_testnet.ts --network pijstestnet
网络：
PIJS测试网络
合约名称：
PIJSRewardDistributeFromAlliancePlan
地址：
0x41Fa8d95cAf846a0f5fdf758219B7B399968db38
签名地址：
0xd4f0f0c79a35f217e5de4bff0752ba63cbc013e9

 */