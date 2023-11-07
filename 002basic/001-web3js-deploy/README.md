
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

# References

- Web3js Official Documents: https://web3js.readthedocs.io/en/v1.2.11/getting-started.html
- Code and Examples: https://docs.moonbeam.network/getting-started/local-node/deploy-contract/
- How to use web3js: https://www.dappuniversity.com/articles/web3-js-intro
- Nodejs APIs Documents: http://nodejs.cn/api/fs.html
