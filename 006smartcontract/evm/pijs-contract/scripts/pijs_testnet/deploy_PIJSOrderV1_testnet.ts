import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
async function main() {
    const [owner,singer] = await ethers.getSigners();
    const PIJSOrder = await ethers.getContractFactory('PIJSOrderV1');
    // const args = [singer.address];
     const args = ["0xd4f0f0c79a35f217e5de4bff0752ba63cbc013e9"];
    // const PIJSOrder_C = await upgrades.deployProxy(PIJSOrder,args,{ kind:'uups'});
    const PIJSOrder_C = await upgrades.upgradeProxy('0x46Fb52f671bde1235842AFA8776B847BFdb464B1', PIJSOrder, { kind: 'uups' });
    await PIJSOrder_C.deployed();
    console.log("pijs oder contract address:",PIJSOrder_C.address);






}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
/***
 * 
npx hardhat run ./scripts/pijs_testnet/deploy_PIJSOrderV1_testnet.ts --network pijstestnet

 PIJSOrderV1 0x46Fb52f671bde1235842AFA8776B847BFdb464B1
 PIJSRewardDistribute 0xcaEA3388aA91AE0a6EA6a1A0BE7B9F797E41F3F0

0xa65f7e356Ce6B1AbdCeB2BC2Cfa4956cfc56839E

zhengshi:
 0x5856C0f60CfEbD9BDBC403C6f72608630FF8E4D3
 */


