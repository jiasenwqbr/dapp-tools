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
    "0x3A969F69460D62873b0c7f63fb750063C7f4360d"
  );
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
};
Trans()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
