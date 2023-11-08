
# Abstract 前言

Through this basic task, you can learn the processes of compiling and deploying a smart contract, as well as learn how to use the basic APIs of `web3js`.
通过本样例代码，使开发者了解合约编译，部署的基本流程，并掌握基本的 web3js 接口使用方法

# Preparation 准备工作

- You need to create a project on [Infura](https://infura.io), and get the `PROJECT ID`, change your `ENDPOINTS` to `Sepolia`;
  你需要在[Infura](https://infura.io)创建一个项目，之后获取`PROJECT ID`，然后更换你的`Sepolia`的`ENDPOINTS` ；

- Create an account on `MetaMask`, which is a browser extension; 安装`MetaMask`插件；

  1. Get a wallet `address`, and the private key; 获取钱包的地址，就是私钥；
  2. Go `Settings` - `advanced` and open `Show test networks`;到`设置`-`高级`，打开`显示测试网络`
     - Select `Sepolia`, and record this address 选择`Sepolia`
  3. Top up your account ;给账号充值；
   测试代币：
  - 每日领取0.5：https://sepoliafaucet.com/
  - 每日领取1：https://www.infura.io/faucet/sepolia
  - 每日领取0.5：https://access.rockx.com/faucet-sepolia
  - 无上限挖取测试币单次最高2.5：https://sepolia-faucet.pk910.de/
  4. Wait for minutes, and see the balance on `MetaMask` 等待一会儿，可以看到钱包的余额

- Create a `.env` file, and add the following lines:  创建`.env`文件

  ```text
  PRIVATE_KEY=YOUR_PRIVATE_KEY
  INFURA_ID=YOUR_PROJECT_ID
  ```

  | Note: You can check the `.env.example` file. 样例文件可参考 .env.example 



# Getting Started 开始

## Understanding The Functions of the [Smart Contract](Incrementer.sol)合约功能说明

- `Constructor`: The constructor function of the smart contract, which is called when the contract is deployed, at the same time it will initialize the `number` to `_initialNumber`;构造函数, 用于部署合约时调用, 同时在其中初始化了公共变量 number 的值；
- `increment`: The function of incrementing the `number` by given `_value`;增值函数, 根据传入的数值 ( _value ), 对公共变量 number 进行增值 ( number + _value )；
- `rest`: The function of resetting the `number` to 0;重置函数, 用于重置公共变量 number 的值为 0；
- `getNumber`: The function of getting the `number`.查询函数, 用于查询公共变量 number 当前的数值。

## How to run it 如何运行

1. Install dependencies: `npm install` 安装依赖
   ```bash
   npm install
   ```
2. Copy the configuration file: `cp .env.example .env`
   ```bash
   cp .env.example .env

    ## 修改 .env 中的 INFURA_ID 和 PRIVATE_KEY 为实际的值  
    PRIVATE_KEY=xxxxxxxxxxxxxxxx
    INFURA_ID=yyyyyyyy
   ```
3. Edit the configuration file: `vim .env`, copy your project ID and private key to the `.env` file
   ```text
   PRIVATE_KEY=YOUR_PRIVATE_KEY
   INFURA_ID=YOUR_PROJECT_ID
   ```
4. Run the `index.js` file: `node index.js`

# Interpret the Code in `index.js` 执行index.js
```bash
node index.js
```

`index.js` contains the most important part of this task, which includes the following functions: index.js功能说明：

## 1. Load the configuration file  读取配置文件

For security sake, the private key is not hard-coded, but it can be read as environment variables. When run this task, the `dotenv` plugin will automatically read the configurations in the `.env` file and load them as environment variables, and then you can use the private key and other environment variables via `process.env` .  
出于安全考虑, 私钥没有进行硬编码, 而是通过环境变量的方式进行获取. 启动测试时, dotenv 插件自动读取 .env 配置文件中的配置项, 然后加载为环境变量, 之后在代码中可以通过 process.env 读取私钥 ( 也包括其他环境变量 )
Here is the code:

```js
require('dotenv').config();
const privatekey = process.env.PRIVATE_KEY;
```

## 2. Compile the smart contract file 编译合约

You can not use `.sol` files directly, you need to compile it to binary file 
我们无法直接使用 .sol 文件, 需要把它编译为 bin 文件 ( 二进制文件 ), 因此在代码中需要进行这一步的逻辑处理.firstly.

### Load the smart contract file `Incrementer.sol` into `source` variable.读取文件
第一步, 我们先进行文件的读取, 把 sol 文件加载为 source 变量

```js
// Load contract
const source = fs.readFileSync('Incrementer.sol', 'utf8');
```

#### Compile the smart contract file 进行合约编译
这里进行编译动作. 把 sol 源码编译为 solidity 对象. 这里需要注意的是不同的 sol 源码版本, 
```js
const input = {
  language: 'Solidity',
  sources: {
    'Incrementer.sol': {
      content: source,
    },
  },
  settings: {
    outputSelection: {
      '*': {
        '*': ['*'],
      },
    },
  },
};

const tempFile = JSON.parse(solc.compile(JSON.stringify(input)));
```

| Note: The version of solidity in this task is `0.8.0`, different versions may have different compile ways.编译的方式可能稍有不同, 这里因为 "Incrementer.sol" 对应的是 sol 是 0.8.0 版本, 所以我们可以使用如下的方式进行编译

## 3. Get the `bytecode` and `abi` 获取二进制对象

```js
const contractFile = tempFile.contracts['Incrementer.sol']['Incrementer'];

// Get bin & abi
const bytecode = contractFile.evm.bytecode.object;
const abi = contractFile.abi;
```

## 4. Create the `web3` instance 构造 web3 对象

`web3` is the main API of the `web3js` library. It is used to interact with the blockchain.
通过 web3 对象可以很方便的发送相应的交易到区块链网络, 同时获取区块链的处理结果. 构造 web3 对象时, 主要需要传入一个参数, 就是对应的区块链网络, 包括 sepolia 等测试网络, 或是 mainnet 主网. 这里我们使用 sepolia 测试网络. 如果没有 sepolia 网络的测试币, 可以切换到其他的测试网络. 同时需要注意的是, 这里我们通过 infura 向对应的区块链网络发送交易, 而 INFURA_ID 这个变量值也需要配置在 .env 文件中, 具体如何获取 infura_id, 可自行搜索查找相关文档

```js
// Create web3 with sepolia provider，you can change sepolia to other testnet
const web3 = new Web3('https://sepolia.infura.io/v3/' + process.env.INFURA_ID);
```

| Note: The `INFURA_ID` is the `PROJECT ID` of the `Infura` project you created in **Preparation** part.

## 5. Get the `account` address 获取账户地址

On blockchain, each user has a `address`, which is unique for others, and you can get the `address` by the private key. In this task, you can use to `web3.eth.accounts.privateKeyToAccount` API to get your `account` address by passing the private key as a parameter.
在区块链上, 每个用户都有一个对应的账户地址, 而这个账户地址可以通过私钥进行获取. 这里, 我们调用 web3.eth.accounts.privateKeyToAccount 接口, 传入对应的私钥, 就可以获取对应的账户地址

```js
// Create account from privatekey
const accounts = web3.eth.accounts.wallet.add(privatekey);
```

## 6. Get contract instance 构造合约实例

In the 3rd step, you got the `bytecode` and `abi`, so you can create the contract instance by the `abi`
 在步骤 3 中, 我们获取了 sol 源文件编译后的二进制 和 abi, 这里就可以使用对应的 abi 构造相应的合约实例, 以便在后续中通过合约实例进行交易的发送

```js
// Create contract instance
const deployContract = new web3.eth.Contract(abi);
```

## 7. Create the `deploy` transaction 创建合约交易

```js
// Create Tx
const deployTx = deployContract.deploy({
  data: '0x' + bytecode,
  arguments: [0], // Pass arguments to the contract constructor on deployment(_initialNumber in Incremental.sol) 调用 deployContract.deploy 接口, 我们创建了部署合约的二进制交易. 这里, 此交易还没有发送到区块链网络, 即合约还没有被创建
});
```

## 8. Deploy your smart contract 部署合约

Use your private key to sign the `deploy` transaction.
这里使用发送签名后的交易到区块链网络, 同时会去返回的交易回执. 从返回的交易回执中可以得到此次部署的合约的地址

```js
const tx = await deployTx.send({
  from: accounts[0].address,
  gas,
  // gasPrice: 10000000000,
});
```
### deployed successful 部署成功
```bash
node index.js
estimated gas: 176272n
Contract deployed at address: ..................
```
Your deployed contrac can be viewed at: https://sepolia.etherscan.io/address/${tx.options.address}
你部署的合约可以在这里看到: https://sepolia.etherscan.io/address/${tx.options.address}

## FAQ
### TransactionBlockTimeoutError
```bash
TransactionBlockTimeoutError: Transaction started at 4651045 but was not mined within 50 blocks. Please make sure your transaction was properly sent and there are no previous pending transaction for the same account. However, be aware that it might still be mined!
        Transaction Hash: 0xaee6a685bb6086993f9cfce65698035e1e2a3526c63e0a3065d8b00dd32e9d60
    at /Users/a1234/Desktop/Markdown/dapp-tools/002basic/001-web3js-deploy/node_modules/web3-eth/lib/commonjs/utils/reject_if_block_timeout.js:31:20
    at Generator.next (<anonymous>)
    at fulfilled (/Users/a1234/Desktop/Markdown/dapp-tools/002basic/001-web3js-deploy/node_modules/web3-eth/lib/commonjs/utils/reject_if_block_timeout.js:5:58)
    at process.processTicksAndRejections (node:internal/process/task_queues:95:5) {
  innerError: undefined,
  code: 432
}
````
The error message you provided is related to Ethereum blockchain transactions and it indicates that a transaction you initiated was not mined within a certain number of blocks. This can happen for several reasons, and it's important to understand the possible causes and how to address them:
你提供的错误消息与以太坊区块链交易相关，它表明你发起的交易在一定数量的区块内未被打包。这可能由多种原因引起，重要的是要了解可能的原因以及如何解决：

- Transaction Queuing: Ethereum transactions are processed in a queue, and if there are many transactions pending ahead of yours, it may take some time for your transaction to be included in a block. This is particularly common during periods of high network congestion.
交易排队：以太坊交易在队列中处理，如果有许多在你之前等待处理的交易，你的交易可能需要一些时间才能被打包到区块中。这在网络拥堵时特别常见。

- Gas Price: Transactions require a fee called gas, and the gas price you set can affect how quickly your transaction is processed. If you set a low gas price, miners may prioritize other transactions with higher fees. You can increase the gas price to potentially speed up confirmation.
燃气价格：交易需要支付燃气费，你设置的燃气价格会影响交易被处理的速度。如果你设置了较低的燃气价格，矿工可能会优先处理具有更高手续费的其他交易。你可以增加燃气价格以加快确认速度。
- Pending Transactions: If there are other pending transactions from the same account, they can potentially block subsequent transactions. Make sure there are no previous pending transactions associated with the same account, as the error message suggests.
待处理交易：如果同一帐户存在其他待处理的交易，它们可能会阻止后续交易的处理。确保同一帐户没有先前未处理的待处理交易，正如错误消息所建议的那样。
- Network Congestion: High network congestion can lead to delays in transaction processing. This is especially common during periods of high demand, such as ICO launches or decentralized application (DApp) usage spikes.
网络拥堵：高度拥堵的网络可能导致交易处理延迟。这在需求高峰期，如ICO发布或去中心化应用（DApp）使用高峰时尤其常见。
- Network Issues: Sometimes, the issue might not be with your transaction but with the Ethereum network itself. Network issues, outages, or software bugs can cause delays in transaction confirmation.
网络问题：有时问题可能不是由于你的交易，而是由于以太坊网络本身。网络问题、故障或软件错误可能导致交易确认延迟。
To address this issue:
要解决此问题：
- Check the status of your transaction using the transaction hash provided. You can use an Ethereum block explorer like Etherscan to monitor your transaction's status.
使用提供的交易哈希检查交易的状态。你可以使用以太坊区块浏览器（如Etherscan）来监视交易的状态。
- If you see your transaction as pending on a block explorer, you can wait for it to be mined. Be patient, as it might take some time, especially during network congestion.
如果在区块浏览器上看到交易处于挂起状态，可以等待它被打包。要有耐心，特别是在网络拥堵时可能需要一些时间。
- If you believe your transaction is stuck due to a low gas price or other issues, you can attempt to send a new transaction with a higher gas price and the same nonce (if applicable) to replace the stuck transaction. This new transaction may get mined more quickly.
如果你认为交易由于低燃气价格或其他原因而卡住，可以尝试发送一个具有更高燃气价格的新交易，并具有相同的nonce（如果适用），以替代卡住的交易。这个新交易可能会更快地被打包。
- Ensure there are no pending transactions from the same account that might be blocking the new transaction.
确保没有来自同一帐户的待处理交易可能会阻止新交易。
- If the Ethereum network is experiencing widespread issues, you might have to wait until the network stabilizes.
如果以太坊网络存在广泛问题，你可能需要等待网络稳定。
It's important to note that Ethereum transaction processing times can vary, and you might not always have control over the timing. Be cautious when increasing the gas price, as setting it too high can result in unnecessary fees. Make sure to use a reliable library or tool to handle your transactions and handle any potential errors gracefully in your application.
重要的是要注意，以太坊交易处理时间可能有所不同，你可能无法始终控制时机。在增加燃气价格时要小心，因为设置得太高可能会导致不必要的费用。确保使用可靠的库或工具来处理你的交易，并在你的应用程序中优雅地处理潜在的错误。


# References

- Web3js Official Documents: https://web3js.readthedocs.io/en/v1.2.11/getting-started.html
- Code and Examples: https://docs.moonbeam.network/getting-started/local-node/deploy-contract/
- How to use web3js: https://www.dappuniversity.com/articles/web3-js-intro
- Nodejs APIs Documents: http://nodejs.cn/api/fs.html
