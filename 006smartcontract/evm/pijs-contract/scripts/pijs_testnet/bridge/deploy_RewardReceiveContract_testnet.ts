import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
async function mian(){
    const [owner,singer] = await ethers.getSigners();
    const RewardReceiveContract = await ethers.getContractFactory('RewardReceiveContract');
    const args = [owner.address];

    const RewardReceiveContract_deploy = await upgrades.deployProxy(RewardReceiveContract,args,{kind:'uups'});
    // const RewardReceiveContract_deploy = await upgrades.upgradeProxy('0x0A48C9D77C72994FfbFBFbC8dA2d17c2788318C0', RewardReceiveContract, { kind: 'uups' });
    
    await RewardReceiveContract_deploy.deployed();

    console.log("StakingUAC address is:",RewardReceiveContract_deploy.address);


}

mian().catch(error => {
    console.error(error);
    process.exitCode = 1;
});

/***
 * 0x0A48C9D77C72994FfbFBFbC8dA2d17c2788318C0
 npx hardhat run ./scripts/pijs_testnet/bridge/deploy_RewardReceiveContract_testnet.ts --network pijstestnet
 */