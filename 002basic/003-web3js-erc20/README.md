# Web3js ERC20

This basic task is to show how to interact with ERC20 contract, so the developer can understand the basic interface of ERC20 contract.
本样例演示了 ERC20 合约的基本调用, 让开发者了解 ERC20 合约的基本接口

## Getting started 开始
### SimpleToken contract function description   SimpleToken合约功能描述

- IERC20
  totalSupply: Get the total amount of ERC20 token in the contract 获取该合约内总的 ERC20 Token 总量  
  balanceOf: Get the amount of ERC20 token of specific account in the contract 获取特定账户的 ERC20 Token 总量  
  transfer: Transfer ERC20 token to specific account 向目标账户转移特定数量的 ERC20 Token
  allowance: Get the amount of ERC20 tokens of the source account that the target account can use 获取目标账户能够使用的源账户的 ERC20 Token 数量  
  approve: Authorize the target account to transfer the specified amount of ERC20 Tokens 向目标账户授权, 可以转移指定额度的 ERC20 Token 数量  
  transferFrom: (Third party call) Transfer a specific amount of ERC20 token from the source account to target account ( 第三方调用 ) 从源账户向目标账户转移制定数量的 ERC20 Token

- IERC20Metadata
  name: Get the name of the Token (返回 Token 的名称)  
  symbol: Get the symbol of the Token (返回 Token 的符号  )
  decimals: Get the decimals of the Token (返回 Token 所支持的精度)

  ## How to run it 测试流程

1. Install dependencies 安装依赖

   ```sh
   npm install
   ```

2. Copy the configuration file 配置 .env

   ```sh
   cp .env.example .env

   ## 修改 .env 中的 INFURA_ID 和 PRIVATE_KEY 为实际的值
   PRIVATE_KEY=xxxxxxxxxxxxxxxx
   INFURA_ID=yyyyyyyy
   ```

3. Run the `index.js` file 执行测试

   ```sh
   node index.js
   ```

## Interpret Source Code 代码逻辑说明
### `compile.js`
You can't use `.sol` files directly, you need to compile it to binary file firstly.
我们无法直接使用 .sol 文件, 需要把它编译为 bin 文件 ( 二进制文件 ), 因此在代码中需要进行这一步的逻辑处理.

1. Load the smart contract file `SimpleToken.sol` into `source` variable.

```js
// Load contract
const source = fs.readFileSync('SimpleToken.sol', 'utf8');
```
2. Compile the smart contract file 编译智能合约

```js
// compile solidity
const input = {
    language: 'Solidity',
    sources: {
    'SimpleToken.sol': {
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

| Note: The version of solidity in this task is `0.8.0`, different versions may have different compile ways.

3. Get the Contract Binary Object 获取二进制对象

The solidity object that was successfully compiled in the previous step contains many properties/values, and what we only need is the contract object, so we can get the `SimpleToken` contract object by accessing the object properties.
 在上一步编译成功的 solidity 对象里面包含很多的属性/值, 而我们需要的是其中合约对象, 通过访问对象属性的方式提示 SimpleToken 合约对象

```js
const contractFile = tempFile.contracts['SimpleToken.sol']['SimpleToken'];
```

4. Export `contractFile` Object  导出对象
If you want to use the `contractFile` object in other `js` files, you can export it.
为了能使其他 js 文件使用 SimpleToken 合约对象 , 我们需要对合约对象进行导出

```js
module.exports = contractFile;
```

---

## index.js 
1. Load the `SimpleToken` smart contract from `compile` file
    
```js
const contractFile = require('./compile');
```

2. Load private key 读取私钥

For security’s sake, the private key is not hard-coded, but it can be read as environment variables. When run this task, the `dotenv` plugin will automatically read the configurations in the `.env` file and load them as environment variables, and then you can use the private key and other environment variables via `process.env`.  
处于安全考虑, 私钥没有进行硬编码, 而是通过环境变量的方式进行获取. 启动测试时, dotenv 插件自动读取 .env 配置文件中的配置项, 然后加载为环境变量, 之后在代码中可以通过 process.env 读取私钥 ( 也包括其他环境变量 )

```js
require('dotenv').config();
const privatekey = process.env.PRIVATE_KEY;
```

3. Create a `receiver` account for testing  设置收款账户

```js
const receiver = '0x840bAEb9979233405c5626DFe99C288c3173c45F';
```

4. Build the `web3` object  构建web3对象

```js
const web3 = new Web3(new Web3.providers.HttpProvider('https://sepolia.infura.io/v3/' + process.env.INFURA_ID));
```
| Note: The `INFURA_ID` is the `PROJECT ID` of the `Infura` project you created in last [task](../01-web3js-deploy/README.md)

5. Get the `account` address  获取账户地址

On blockchain, each user has a `address`, which is unique for others, and you can get the `address` by the private key. In this task, you can use the `we3.eth.accounts.privateKeyToAccount` API to get your `account` address by passing the private key as a parameter.
在区块链上, 每个用户都有一个对应的账户地址, 而这个账户地址可以通过私钥进行获取. 这里, 我们调用 web3.eth.accounts.privateKeyToAccount 接口, 传入对应的私钥, 就可以获取对应的账户地址

```js
const account = web3.eth.accounts.privateKeyToAccount(privatekey);
const account_from = {
    privateKey: account.privateKey,
    accountaddress: account.address,
};
```

6. Get the `abi` and `bin` 获取 abi 和 bin  
When deploying the smart contract, we need two important parameters which are the `bytecode` and `abi` of the smart contract. In previous step 1, we loaded the compiled `SimpleToken` object, so we can get the `bytecode` and `abi` from it.
在部署合约的过程中, 我们会用到两个重要的参数, 合约对应的 bytecode 和 abi. 在步骤 1 的时候, 我们导入了编译后的 SimpleToken 合约对象, 通过这个对象, 我们可以获取的合约对应的 bytecode 和 abi


```js
const bytecode = contractFile.evm.bytecode.object;
const abi = contractFile.abi;
```

7. Get contract instance 创建合约交易
In the last step, you got the `bin` and `abi`, so we can create the contract instance by the `abi`.
 调用 deployContract.deploy 接口, 我们创建了部署合约的二进制交易. 这里, 此交易还没有发送到区块链网络, 即合约还没有被创建
   
```js
const deployContract = new web3.eth.Contract(abi);
```

8. Create the transaction of the `deployContract`  创建合约交易

```js
const deployTx = deployContract.deploy({
    data: bytecode,
    arguments: ['DAPPLEARNING', 'DAPP', 0, 10000000],
});
```
| So far, this transaction has not been deployed into the blockchain.

调用 deployContract.deploy 接口, 我们创建了部署合约的二进制交易. 这里, 此交易还没有发送到区块链网络, 即合约还没有被创建

9. Sign the transaction 交易签名
Use your private key to sign the transaction.
如下使用私钥对交易进行签名,
```js
const deployTransaction = await web3.eth.accounts.signTransaction(
    {
    data: deployTx.encodeABI(),
    gas: '8000000',
    },
    account_from.privateKey
);
```

10. Deploy the contract 部署合约
Send your signed `deployTransaction` transaction to the blockchain. You will receive a receipt, and get this contract address from it.
 这里发送签名后的交易到区块链网络, 同时得到返回的交易回执. 从返回的交易回执中可以得到此次部署的合约的地址

```js
const deployReceipt = await web3.eth.sendSignedTransaction(deployTransaction.rawTransaction);
console.log(`Contract deployed at address: ${deployReceipt.contractAddress}`);
```

11. Create a transfer transaction 构造转账交易

We created a transfer transaction for `ERC20` token, the receiver is `receiver` account, and the amount is `100000` token.
如下构造一个 ERC20 Token 的转账交易, 收款账户为 receiver, 转账金额为 100000

```js
const transferTx = erc20Contract.methods.transfer(receiver, 100000).encodeABI();
```

12. Sign and send the transaction 签名并发送交易

```js
const transferReceipt = await web3.eth.sendSignedTransaction(transferTransaction.rawTransaction);
```

13. Check the balance of the `receiver` account 验证转账后余额

After the transaction is sent, you can log the balance of the `receiver` and make sure the balance is correct.
转账成功后, 输出验证下收款账户的余额, 检查余额是否正确
```js
erc20Contract.methods
    .balanceOf(receiver)
    .call()
    .then((result) => {
    console.log(`The balance of receiver is ${result}`);
    });
```

## Notes

- `infura` doesn't support `sendTransaction`, only support `sendRawTransaction`
- `infura` doesn't invoke `eth_sendTransaction`, so you need to an unlocked account on the `ethereum` node. More details, please refer to [this](https://ethereum.stackexchange.com/questions/70853/the-method-eth-sendtransaction-does-not-exist-is-not-available-on-infura)

## References

- Mocha tutorial: http://www.ruanyifeng.com/blog/2015/12/a-mocha-tutorial-of-examples.html
- Mocha blog: https://pcaaron.github.io/pages/fe/block/improve4.html#%E8%B7%91%E6%B5%8B%E8%AF%95
- ERC20 doc: https://docs.openzeppelin.com/contracts/2.x/api/token/erc20
