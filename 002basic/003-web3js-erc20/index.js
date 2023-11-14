const Web3 = require("web3");
const fs = require("fs");
const contractFile = require("./compile");

require("dotenv").config();
const privatekey = process.env.PRIVATE_KEY;
/*
   -- Define Provider & Variables --
*/
const receiver = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
// Provider
const web3 = new Web3(
  new Web3.providers.HttpProvider(
    "https://sepolia.infura.io/v3/" + process.env.INFURA_ID
  )
);
//account
const account = web3.eth.accounts.privateKeyToAccount(privatekey);
const account_from = {
  privateKey: account.privateKey,
  accountaddress: account.address,
};
// sol ---> abi + bin
const bytecode = contractFile.evm.bytecode.object;
const abi = contractFile.abi;
console.log("bytecode:", bytecode);
console.log("abi:", abi);
