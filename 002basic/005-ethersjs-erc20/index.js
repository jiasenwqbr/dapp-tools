const { ethers } = require("ethers");
const fs = require("fs");
const contractFile = require("./compile");
require("dotenv").config();
const privatekey = process.env.PRIVATE_KEY;

const provider = new ethers.providers.JsonRpcProvider(
  "https://sepolia.infura.io/v3/" + process.env.INFURA_ID
);
// Variables
const account_from = {
  privateKey: privatekey,
};

const bytecode = contractFile.evm.bytecode.object;
const abi = contractFile.abi;
// Create Wallet
let wallet = new ethers.Wallet(account_from.privateKey, provider);
/*
  -- Deploy Contract --
*/
// Create Contract Instance with Signer
const deployContractIns = new ethers.ContractFactory(abi, bytecode, wallet);

const Trans = async () => {
  /*
  console.log("===============================1. Deploy Contract");
  console.log(`Attempting to deploy from account: ${wallet.address}`);

  Send Tx (Initial Value set to 5) and Wait for Receipt
  const deployedContract = await deployContractIns.deploy(
    "simple",
    "ss",
    1,
    100000000,
    { gasLimit: 8000000 }
  );
  await deployedContract.deployed();
  console.log(`Contract deployed at address: ${deployedContract.address}`);
   */
  /*
   -- Send Function --
   */
  // Create Contract Instance

  console.log();
  console.log(
    "===============================2. Call Transaction Interface Of Contract"
  );
  const transactionContract = new ethers.Contract(
    "0xB9E5B8F15a91E0f271a27C796B76B1D22427C271",
    abi,
    wallet
  );

  console.log(
    `Transfer 100000 to address: 0xB9E5B8F15a91E0f271a27C796B76B1D22427C271`
  );

  // Call Contract
  const transferReceipt = await transactionContract.transfer(
    "0xB9E5B8F15a91E0f271a27C796B76B1D22427C271",
    100000
  );
  await transferReceipt.wait();

  console.log(`Tx successful with hash: ${transferReceipt.hash}`);

  /*
  -- Call Function --
   */
  // Create Contract Instance

  console.log();
  console.log(
    "===============================3. Call Read Interface Of Contract"
  );
  const providerContract = new ethers.Contract(
    "0xB9E5B8F15a91E0f271a27C796B76B1D22427C271",
    abi,
    provider
  );

  // Call Contract
  const balanceVal = await providerContract.balanceOf(
    "0xB9E5B8F15a91E0f271a27C796B76B1D22427C271"
  );

  console.log(
    `balance of 0xB9E5B8F15a91E0f271a27C796B76B1D22427C271 is : ${balanceVal}`
  );

  /*
   -- Listen to Events --
   */
  console.log();
  console.log("===============================4. Listen To Events");
  // Listen to event once
  providerContract.once("Transfer", (from, to, value) => {
    console.log(
      `I am a once Event Listener, I have got an event Transfer, from: ${from}   to: ${to}   value: ${value}`
    );
  });
  // Listen to events continuously
  providerContract.on("Transfer", (from, to, value) => {
    console.log(
      `I am a longstanding Event Listener, I have got an event Transfer, from: ${from}   to: ${to}   value: ${value}`
    );
  });
  for (let step = 0; step < 3; step++) {
    let transferTransaction = await transactionContract.transfer(
      "0xB9E5B8F15a91E0f271a27C796B76B1D22427C271",
      10
    );
    await transferTransaction.wait();

    if (step == 2) {
      console.log("Going to remove all Listeners");
      providerContract.removeAllListeners();
    }
  }
};
Trans()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
