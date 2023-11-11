const Web3 = require("web3");
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
  development: "https://goerli.infura.io/v3/" + process.env.INFURA_ID,
  moonbase: "https://rpc.testnet.moonbeam.network",
};
const web3 = new Web3(providerRPC.development); //Change to correct network

// // Create account with privatekey
// const account = web3.eth.accounts.privateKeyToAccount(privatekey);
// const account_from = {
//   privateKey: privatekey,
//   accountAddress: account.address,
// };

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
    "============================ 2. Call Contract Interface getNumber"
  );
  let incrementer = new web3.eth.Contract(
    abi,
    "0xCfe7c76D3cfe6b29e4607E6Aff9Ef5d2293694d5"
  );

  console.log(incrementer);

  let number = await incrementer.methods.getNumber().call();
  console.log(`The current number stored is: ${number}`);
};
Trans()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
