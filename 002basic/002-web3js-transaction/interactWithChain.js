const { Web3 } = require("web3");
const fs = require("fs");
require("dotenv").config();
const privatekey = process.env.PRIVATE_KEY;
function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
/*
-- Define Provider --
*/
///Provider;
const providerRPC = {
  development: "https://sepolia.infura.io/v3/" + process.env.INFURA_ID,
};
const web3 = new Web3(providerRPC.development); //Change to correct network
// Create account with privatekey
const account = web3.eth.accounts.privateKeyToAccount(privatekey);
console.log("account:", account);
const account_from = {
  privateKey: privatekey,
  accountAddress: account.address,
};

// Get abi & bin
const abi = [
  {
    inputs: [
      { internalType: "uint256", name: "_initialNumber", type: "uint256" },
    ],
    stateMutability: "nonpayable",
    type: "constructor",
  },
  {
    inputs: [],
    name: "getNumber",
    outputs: [{ internalType: "uint256", name: "", type: "uint256" }],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [{ internalType: "uint256", name: "_value", type: "uint256" }],
    name: "increment",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [],
    name: "number",
    outputs: [{ internalType: "uint256", name: "", type: "uint256" }],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "reset",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
];
const bytecode =
  "0x608060405234801561001057600080fd5b506004361061004c5760003560e01c80637cf5dab0146100515780638381f58a1461006d578063d826f88f1461008b578063f2c9ecd814610095575b600080fd5b61006b600480360381019061006691906100f7565b6100b3565b005b6100756100ca565b604051610082919061012f565b60405180910390f35b6100936100d0565b005b61009d6100d9565b6040516100aa919061012f565b60405180910390f35b806000546100c1919061014a565b60008190555050565b60005481565b60008081905550565b60008054905090565b6000813590506100f1816101d9565b92915050565b60006020828403121561010957600080fd5b6000610117848285016100e2565b91505092915050565b610129816101a0565b82525050565b60006020820190506101446000830184610120565b92915050565b6000610155826101a0565b9150610160836101a0565b9250827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff03821115610195576101946101aa565b5b828201905092915050565b6000819050919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b6101e2816101a0565b81146101ed57600080fd5b5056fea26469706673582212200aaa26e0ee4d35c9b565420ea9e1d5a7c69e22ab92c626f7bb10039ab29d9f9464736f6c63430008000033";
const Trans = async () => {
  console.log(
    "============================ 2. Call Contract Interface getNumber"
  );
  let contractAddress = "0x3A969F69460D62873b0c7f63fb750063C7f4360d";
  let incrementer = new web3.eth.Contract(abi, contractAddress);
  //console.log(incrementer);
  // let number = await incrementer.methods.getNumber().call();
  // console.log("number:", number);
  // console.log(`The current number stored is: ${number}`);

  await incrementer.methods
    .getNumber()
    .call()
    .then((result) => {
      console.log("返回值:", result);
    })
    .catch((error) => {
      console.error("读取数据时出错:", error);
    });
  console.log(
    "============================ 3. Call Contract Interface increment"
  );
  const _value = 3;
  let incrementTx = incrementer.methods.increment(_value);

  console.log("incrementTx:", incrementTx);
  // Sign with Pk
  let incrementTransaction = await web3.eth.accounts.signTransaction(
    {
      to: contractAddress,
      data: incrementTx.encodeABI(),
      gas: 800000000,
    },
    account_from.privateKey
  );

  // Send Transactoin and Get TransactionHash
  const incrementReceipt = await web3.eth.sendSignedTransaction(
    incrementTransaction.rawTransaction
  );
  console.log(`Tx successful with hash: ${incrementReceipt.transactionHash}`);

  number = await incrementer.methods.getNumber().call();
  console.log(`After increment, the current number stored is: ${number}`);
};
Trans()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
