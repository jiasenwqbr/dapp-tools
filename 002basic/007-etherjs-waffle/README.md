## Preface 前言

Waffle is a smart contract test library that adapts to ehter.js. This example demonstrates the basic process and usage of Waffle test.
You can refer to the offical website of Waffle width the detailed usage(https://ethereum-waffle.readthedocs.io/en/latest/getting-started.html).If you are a developler not familiar with Waffle, you could read the sample code and do exercises, then refer to the official website for a more in-depth understanding.<br>
Waffle 是一款适配 ehter.js 的智能合约测试库。本样例演示了使用 Waffle 进行测试的基本流程及使用方法.  
Waffle 详细使用方法可以参考 [Waffle 官网](https://ethereum-waffle.readthedocs.io/en/latest/getting-started.html) , 对于不熟悉 Waffle 测试框架的开发者, 可以根据本样例进行基础的操作, 阅读样例代码, 形成初步的流程概念, 之后再参考官网进行更加深入的了解.
## Contracts Introduction

- contract/SimpleToken.sol
  A standard ERC20 contract that implements all interface of ERC20. Users could issue ERC20 tokens using this contract.<br>一个标准的 ERC20 合约, 实现了 ERC20 的所有接口, 用户可以使用这个合约进行 ERC20 代币的发放.
  

## Scripts Introduction

- test/simpleTokenTest.js
  The unit test code of SimpleToken.sol contract. There is only one test script here, you could write multiple scripts of unit test codes for different contracts under the "test" directory during the development.
  Each interface of SimpleToken.sol contract will be simplely tested in the simpleTokenTest.js script.You can refer to the sample to write unit test codes of other contracts.
<br>
SimpleToken.sol 合约的单元测试代码. 这里只写了一个测试脚本, 实际开发中, 可以在 test 目录下, 针对不同的合约, 编写多个单元测试脚本, 之后使用 yarn test 命令即可执行 test 目录下所有的单元测试脚本.  
  在 simpleTokenTest.js 脚本中, 对 SimpleToken.sol 合约的各个接口进行简单的测试, 可以参考此样例编写其他合约的单元测试代码.
- index.js
  External contracts needs to be invoked separately. When unit test is passed, the script could be called to generate some actions in the production environment.<br>
  外部合约, 需要单独进行调用. 对应实际生产环境中, 当单元测试通过后, 就可以调用此脚本进行实际的生成操作.  
  此脚本名字自行进行定义, 这里是使用 index.js 进行指定

## steps

- 1 install dependencies 安装依赖

```bash
npm install
```

- 2 compile contracts 编译合约

```bash
npm run  build
```

- 3 config environment variables 配置环境变量

```bash
cp .env.example .env

## modify PRIVATE_KEY and INFURA_ID in .env
```

- 4 Test Execution 执行测试

```bash
npm run  test
```
```bash
$ npm run test 

> 007-ethersjs-waffle@1.0.0 test
> export NODE_ENV=test && mocha --timeout 10000



  SimpleToken
*****2
    ✓ Assigns initial balance
    ✓ Transfer adds amount to destination account (100ms)
    ✓ Transfer emits event (84ms)
    ✓ Can not transfer above the amount (41ms)
    ✓ Calls totalSupply on SimpleToken contract
    ✓ Calls balanceOf with sender address on SimpleToken contract


  6 passing (2s)
```
- 5 test index.js

```bash
node index.js
```


## Reference document 参考文档


- waffle offical document: <https://ethereum-waffle.readthedocs.io/en/latest/getting-started.html>

- etherjs offical document: <https://docs.ethers.io/v4/api-providers.html>  
  <https://docs.ethers.io/v5/getting-started/#getting-started--contracts>

- Chinese document: <https://learnblockchain.cn/docs/ethers.js/api-providers.html>
