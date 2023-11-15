const Web3 = require("web3");
const fs = require("fs");
const contractFile = require("./compile");

require("dotenv").config();
const privatekey = process.env.PRIVATE_KEY;
/*
   -- Define Provider & Variables --
*/
const receiver = "0x840bAEb9979233405c5626DFe99C288c3173c45F";
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
/*
   -- Deploy Contract --
*/
const Trans = async () => {
  console.log(
    `Attempting to deploy from account ${account_from.accountaddress}`
  );
  web3.eth.getBlockNumber(function (error, result) {
    console.log(result);
  });
  // Create deploy Contract Instance  创建部署合约的实例
  const deployContract = new web3.eth.Contract(abi);
  // method 1
  // Create Constructor Tx
  const deployTx = deployContract.deploy({
    data: bytecode,
    arguments: ["DAPP", "DAPP", 0, 10000000],
  });
  // Sign Transacation and Send
  const deployTransaction = await web3.eth.accounts.signTransaction(
    {
      data: deployTx.encodeABI(),
      gas: "8000000",
    },
    account_from.privateKey
  );

  // Send Tx and Wait for Receipt
  const deployReceipt = await web3.eth.sendSignedTransaction(
    deployTransaction.rawTransaction
  );
  console.log(`Contract deployed at address: ${deployReceipt.contractAddress}`);
  const erc20Contract = new web3.eth.Contract(
    abi,
    deployReceipt.contractAddress
  );

  //build the Tx
  const transferTx = erc20Contract.methods
    .transfer(receiver, 100000)
    .encodeABI();

  // Sign Tx with PK
  const transferTransaction = await web3.eth.accounts.signTransaction(
    {
      to: deployReceipt.contractAddress,
      data: transferTx,
      gas: 8000000,
    },
    account_from.privateKey
  );

  // Send Tx and Wait for Receipt
  await web3.eth.sendSignedTransaction(transferTransaction.rawTransaction);

  await erc20Contract.methods
    .balanceOf(receiver)
    .call()
    .then((result) => {
      console.log(`The balance of receiver is ${result}`);
    });
};

Trans()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
