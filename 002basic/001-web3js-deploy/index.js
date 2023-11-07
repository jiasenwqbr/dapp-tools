let { Web3 } = require("web3");
let solc = require("solc");
let fs = require("fs");
const { Console } = require("console");

// Get privatekey from environment
// 从环境中获取私钥
require("dotenv").config();
let privatekey = process.env.PRIVATE_KEY;
if (privatekey.slice(0, 2) !== "0x") privatekey = "0x" + privatekey;
// console.log(privatekey);
// Load Contract
// 加载合约
const source = fs.readFileSync("Incrementer.sol", "utf8");
// console.log("source:", source);
// complie solidity
// 编译 solidity
const input = {
  language: "Solidity",
  sources: {
    "Incrementer.sol": {
      content: source,
    },
  },
  settings: {
    outputSelection: {
      "*": {
        "*": ["*"],
      },
    },
  },
};
const compliedCode = JSON.parse(solc.compile(JSON.stringify(input)));
// console.log("compliedCode:", compliedCode);
const contractFile = compliedCode.contracts["Incrementer.sol"]["Incrementer"];
// console.log("contractFile:", contractFile);
// Get bin & abi
const bytecode = contractFile.evm.bytecode.object;
const abi = contractFile.abi;
// Create web3 with sepolia provider，you can change sepolia to other testnet
// 使用sepolia提供商创建web3，您可以将sepolia更改为其他测试网
const web3 = new Web3("https://sepolia.infura.io/v3/" + process.env.INFURA_ID);
// Create account from privatekey
// 从私钥创建账户
const accounts = web3.eth.accounts.wallet.add(privatekey);
// console.log("accounts:", accounts);
/*
   -- Deploy Contract --
   -- 部署合约 --
*/
const Deploy = async () => {
  // Create contract instance
  // 创建合约实例
  /*
  The web3.eth.Contract object makes it easy to interact with smart contracts on the ethereum blockchain. 
  When you create a new contract object you give it the json interface of the respective smart contract and web3 will auto convert all calls into low level ABI calls over RPC for you.
  */
  const deployContract = new web3.eth.Contract(abi);
  console.log("1111111111111111111");
  // Create Tx
  // 创建交易
  const deployTx = deployContract.deploy({
    data: "0x" + bytecode,
    arguments: [0],
  });
  console.log("2222222222222222222");
  // optionally, estimate the gas that will be used for development and log it
  // 可选地，估计将用于开发的gas并记录
  const gas = await deployTx.estimateGas({
    from: accounts,
  });
  console.log("3333333333333333333333");
  console.log("estimated gas:", gas);
  try {
    // Deploy the contract to the INFURA network
    // 部署合约到INFURA网络
    // Your deployed contrac can be viewed at: https://sepolia.etherscan.io/address/${tx.options.address}
    // 你部署的合约可以在这里看到: https://sepolia.etherscan.io/address/${tx.options.address}
    // You can change sepolia in above url to your selected testnet.
    // 您可以将上面url中的sepolia更改为您选择的测试网。
    const tx = await deployTx.send({
      from: accounts[0].address,
      gas,
    });
    console.log("Contract deployed at address: " + tx.options.address);
  } catch (error) {
    console.error(error);
  }
};
// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
Deploy()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
