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
// Create account from privatekey
const accounts = web3.eth.accounts.wallet.add(privatekey);
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
const Trans = async () => {
  console.log(
    "============================ 1. Call Contract Interface getNumber"
  );
  let contractAddress = "0x3A969F69460D62873b0c7f63fb750063C7f4360d";
  let incrementer = new web3.eth.Contract(abi, contractAddress);
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
    "============================ 2. Call Contract Interface increment"
  );
  const _value = 3;
  let incrementTx = incrementer.methods.increment(_value);

  console.log("incrementTx.encodeABI():", incrementTx.encodeABI());
  const gas = await incrementTx.estimateGas({
    from: accounts,
  });
  console.log("gas:", gas);
  // Sign with Pk
  let incrementTransaction = await web3.eth.accounts.signTransaction(
    {
      from: account.address,
      to: contractAddress,
      data: incrementTx.encodeABI(),
      gas: 43896 * 2,
      gasPrice: 10000000000,
    },
    account_from.privateKey
  );
  console.log("incrementTransaction:", incrementTransaction);
  // Send Transactoin and Get TransactionHash
  const incrementReceipt = await web3.eth.sendSignedTransaction(
    incrementTransaction.rawTransaction
  );
  console.log(`Tx successful with hash: ${incrementReceipt.transactionHash}`);

  number = await incrementer.methods.getNumber().call();
  console.log(`After increment, the current number stored is: ${number}`);

  console.log();
  console.log("============================ 3. Call Contract Interface reset");
  const resetTx = incrementer.methods.reset();

  const resetTransaction = await web3.eth.accounts.signTransaction(
    {
      from: account.address,
      to: contractAddress,
      data: resetTx.encodeABI(),
      gas: 8000000,
      gasPrice: 10000000000,
    },
    account_from.privateKey
  );

  const resetcReceipt = await web3.eth.sendSignedTransaction(
    resetTransaction.rawTransaction
  );
  console.log(`Tx successful with hash: ${resetcReceipt.transactionHash}`);
  number = await incrementer.methods.getNumber().call();
  console.log(`After reset, the current number stored is: ${number}`);
};
Trans()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
