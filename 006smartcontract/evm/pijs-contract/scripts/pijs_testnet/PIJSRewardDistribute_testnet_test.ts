import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
import { Contract, ethers as ethers_1, Signer } from "ethers";
import {abi} from "./PIJSRewardDistribute.json" 


async function main() {
    const [owner] = await ethers.getSigners();
    const contract_address =  "0xE203657fA5596fB264ACCFb38981Ae83b98BC1a9";
    const contract = await getContractInstance(contract_address,owner);
     try {
            const tx = await  contract.connect(owner).setFunSwith(false);
            console.log(tx);
            
            const tx1= await  contract.connect(owner).funSwitch();
            console.log(tx1);

         } catch (err: any) {
            console.log(err)
        }
}
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

async function getContractInstance(contract_address:string,owner:Signer): Promise<Contract> {
  // 获取默认 signer（可改为 provider 只读访问）
  const [signer] = await ethers.getSigners();

  // 使用合约 ABI 和地址创建实例
  const contract = new ethers.Contract(
    contract_address,
    abi,
    owner
  );

  return contract;
}

/// npx hardhat run ./scripts/pijs_testnet/PIJSRewardDistribute_testnet_test.ts --network pijstestnet