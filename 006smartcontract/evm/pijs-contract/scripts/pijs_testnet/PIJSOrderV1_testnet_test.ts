import { id } from "ethers/lib/utils";
import { ethers, upgrades } from "hardhat";
import { Contract, ethers as ethers_1, Signer } from "ethers";
import {abi} from "./PIJSOrderV1.json" 
async function main() {
    const [owner] = await ethers.getSigners();
    const contract_address =  "0xF80B1819B4AD1F64b36058bF6dBDC7CD9459aD24";
    const contract = await getContractInstance(contract_address,owner);
    const singer = "0xd4f0f0c79a35f217e5de4bff0752ba63cbc013e9";
    console.log(contract.address);
       const fakeOrder = {
        productId: 1,
        orderId: 1234,
        userId: 1,
        purchaseNum: 1,
        payNum: ethers.utils.parseEther("0.1"),
        endTimestamp: Math.floor(Date.now() / 1000) + 10,
        startTimestamp: Math.floor(Date.now() / 1000),
        userPurchaseLimit: 5,
        productPurchaseLimit: 10,
        phase: 1,
        renewable: 1,
        anchorCoinNum: 100,
        anchorCoin: ethers.utils.formatBytes32String("USDT"),
      };
       console.log(fakeOrder);
      const types = {
        Permit: [
          { name: "productId", type: "uint256" },
          { name: "orderId", type: "uint256" },
          { name: "userId", type: "uint256" },
          { name: "phase", type: "uint256" },
          { name: "purchaseNum", type: "uint256" },
          { name: "payNum", type: "uint256" },
          { name: "anchorCoinNum", type: "uint256"},
          { name: "anchorCoin", type: "bytes32" },
        ],
      };
      const domain = {
        name: "PIJSOrder",
        version: "1",
        chainId: (await ethers.provider.getNetwork()).chainId,
        verifyingContract: contract.address,
      };
      console.log(domain);
      const signature = await owner._signTypedData(domain, types, fakeOrder);
      const data = ethers.utils.defaultAbiCoder.encode(
        [
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "uint256",
          "bytes32",
          "bytes",
        ],
        [
          fakeOrder.productId,
          fakeOrder.orderId,
          fakeOrder.userId,
          fakeOrder.purchaseNum,
          fakeOrder.payNum,
          fakeOrder.endTimestamp,
          fakeOrder.startTimestamp,
          fakeOrder.userPurchaseLimit,
          fakeOrder.productPurchaseLimit,
          fakeOrder.phase,
          fakeOrder.renewable,
          fakeOrder.anchorCoinNum,
          fakeOrder.anchorCoin,
          signature,
        ]
      );
        try {
            const tx = await  contract.connect(owner).makeOrder(data, {
              value: fakeOrder.payNum,
              gasLimit: 2_000_000,
            });
            console.log(tx);
            const receipt = await tx.wait();
            console.log(receipt);
         } catch (err: any) {
            console.log(err)
        }
          
}



main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

/// npx hardhat run ./scripts/pijs_testnet/PIJSOrderV1_testnet_test.ts --network pijstestnet

function encodeOrder(params: any): string {
  return ethers.utils.defaultAbiCoder.encode(
    [
      "uint256", // productId
      "uint256", // orderId
      "uint256", // userId
      "uint256", // purchaseNum
      "uint256", // payNum
      "uint256", // endTimestamp
      "uint256", // startTimestamp
      "uint256", // userPurchaseLimit
      "uint256", // productPurchaseLimit
      "uint256", // phase
      "uint256", // renewable
      "uint256", // anchorCoinNum
      "bytes32", // anchorCoin
      "bytes"    // signature
    ],
    [
      params.productId,
      params.orderId,
      params.userId,
      params.purchaseNum,
      params.payNum,
      params.endTimestamp,
      params.startTimestamp,
      params.userPurchaseLimit,
      params.productPurchaseLimit,
      params.phase,
      params.renewable,
      params.anchorCoinNum,
      params.anchorCoin,
      params.signature,
    ]
  );
}
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