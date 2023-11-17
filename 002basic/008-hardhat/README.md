# Hardhat

Hardhat is a development environment for ethereum application to compile, deploy, test and debug.<br>Hardhat是一个编译、部署、测试和调试以太坊应用的开发环境。

It could help developer to manage , automate build smart contracts and some inherently repetitive tasks in dApps.The core of hardhat is compiling, running and testing smart contracts. It has a build-in Hardhat test network, which is designed for local ethereum network.Its main functions includes Solidity debugging, tracking invoked stack, console.log() and explicit error tips in failed transaction.<br>它可以帮助开发人员管理和自动化构建智能合约和dApps过程中固有的重复性任务，并围绕这一工作流程轻松引入更多功能。这意味着hardhat在最核心的地方是编译、运行和测试智能合约。

Hardhat Runner is a CLI interacted with Hardhat and a extensible task runner. It was designed by the concept of tasks and plugins. When you run Hardhat by CLI, you will run a task, such as `npx hardhat compile`. It will run the build-in compiling task. The task will trigger other tasks, which can give the complicated work flow. Users and plugins could cover all tasks now to customize and extend the work flow.<br>

Hardhat Runner是与Hardhat交互的CLI命令，是一个可扩展的任务运行器。它是围绕**任务**和**插件**的概念设计的。每次你从CLI运行Hardhat时，你都在运行一个任务。例如，`npx hardhat compile`运行的是内置的`compile`任务。任务可以调用其他任务，允许定义复杂的工作流程。用户和插件可以覆盖现有的任务，从而定制和扩展工作流程。

Hardhat的很多功能都来自于插件，而作为开发者，你可以自由选择想使用的插件。Hardhat不限制使用什么工具的，但它确实有一些内置的默认值。所有这些都可以覆盖。

## [Installation](https://hardhat.org/hardhat-runner/docs/getting-started#installation) 安装

Hardhat is used through a local installation in your project. This way your environment will be reproducible, and you will avoid future version conflicts.<br>

Hardhat是通过本地安装在项目中使用的。这样你的环境就可以重现，也可以避免未来的版本冲突。

To install it, you need to create an npm project by going to an empty folder, running npm init, and following its instructions. You can use another package manager, like yarn, but we recommend you use npm 7 or later, as it makes installing Hardhat plugins simpler.

Once your project is ready, you should run

要安装它，你需要创建一个npm项目，进入一个空文件夹，运行`npm init`。 并遵循其指示操作。项目初始化之后，运行：

```bash
npm install --save-dev hardhat
```

To use your local installation of Hardhat, you need to use npx to run it (i.e. npx hardhat init)<br>要使用本地安装的Hardhat，需要使用npx来运行它（如：npx hardhat）。

## Preparation准备工作

Before learning hardhat, you need to understand some Knowledge points as follows:<br>在开始学习 hardhat 之前，你需要提前了解以下知识点：

- dotenv place private key in `.env` files, which could prevent exposure on cloud server, formatted with "PRIVATE_KEY=xxxx". It will be read by code automaticily. Refer to [dotenv](https://www.npmjs.com/package/dotenv)<br>dotenv 将私钥存放在 `.env` 文件中可以避免将私钥暴露在服务器上，格式为 "PRIVATE_KEY=xxxx", 然后代码自动从中读取，详情参考 [dotenv](https://www.npmjs.com/package/dotenv)
- The main problem npx want to resolve is to invoke modules installed internally in the project. Refer to [npx Tutorials](https://www.ruanyifeng.com/blog/2019/02/npx.html)<br>npx 想要解决的主要问题，就是调用项目内部安装的模块。详情参考 [npx 使用教程](https://www.ruanyifeng.com/blog/2019/02/npx.html)
- Compared to web3.js, the interfaces of ethers.js and ethereum network library is easily used(note the interface difference between v5 and v4) [ethers.js v5 document](https://docs.ethers.io/v5/)<br>ethers.js 与以太坊网络交互的工具库，相比 web3.js 接口设计更加易于使用（注意 v5 和 v4 接口差别较大） [ethers.js v5 文档](https://docs.ethers.io/v5/)
- mocha.js test framework is used to write the solution for contracts Interaction. [mochajs document](https://mochajs.org/#getting-started)<br>mocha.js 测试框架，用于编写合约交互的测试案例 [mochajs 文档](https://mochajs.org/#getting-started)
- chai.js assert framework is used to help to write testing scripts, refer to [ethereum-waffle chai document](https://ethereum-waffle.readthedocs.io/en/latest/matchers.html)<br>chai.js 断言库，辅助测试脚本编写，使用方法参考 [ethereum-waffle chai 使用文档](https://ethereum-waffle.readthedocs.io/en/latest/matchers.html)
- infura is a node internet service provider to connect to block chain, which allow some free use amounts. It is enough to develop and debug. [infura offical site](https://infura.io/)<br>infura 连接区块链的节点服务商，有免费的使用额度，足够开发调试使用 [infura 官网](https://infura.io/)

## Project structure and configuration hardhat<br>项目结构和配置 hardhat

```sh
mkdir 07-hardhat                // create folder
cd    07-hardhat                // move to folder
npm install --save-dev hardhat  // install hardhat
npx hardhat                     // initialize hardhat
```

Finished in inputing `npx hardhat`, it will show in the terminal:<br>输入`npx hardhat`后，命令行中会出现如下的界面:

```sh
 npx hardhat
888    888                      888 888               888
888    888                      888 888               888
888    888                      888 888               888
8888888888  8888b.  888d888 .d88888 88888b.   8888b.  888888
888    888     "88b 888P"  d88" 888 888 "88b     "88b 888
888    888 .d888888 888    888  888 888  888 .d888888 888
888    888 888  888 888    Y88b 888 888  888 888  888 Y88b.
888    888 "Y888888 888     "Y88888 888  888 "Y888888  "Y888

👷 Welcome to Hardhat v2.19.1 👷‍

✔ What do you want to do? · Create a JavaScript project
✔ Hardhat project root: · /Users/a1234/Desktop/Markdown/dapp-tools/002basic/008-hardhat
✔ Do you want to add a .gitignore? (Y/n) · y
✔ Do you want to install this sample project's dependencies with npm (@nomicfoundation/hardhat-toolbox)? (Y/n) · y


npm install --save-dev @nomicfoundation/hardhat-toolbox@^4.0.0

added 257 packages, and changed 2 packages in 3m

✨ Project created ✨

See the README.md file for some example tasks you can run

Give Hardhat a star on Github if you're enjoying it! ⭐️✨

     https://github.com/NomicFoundation/hardhat


DEPRECATION WARNING

 Initializing a project with npx hardhat is deprecated and will be removed in the future.
 Please use npx hardhat init instead.

```

We select 'Create a JavaScript project' options to initialize a basic project, click enter directly in the next 2 steps.<br>我们使用'Create a JavaScript project'选项，创建一个基础项目，后面的两个选项直接敲回车选择默认值。

### Project stucture 项目结构

A standard project build in hardhat is as follow:<br>一个标准的使用 hardhat 构建的项目通常是这样的：

```sh
contracts/
scripts/
test/
hardhat.config.js
```

-  palce fileds write in solidity in contracts<br>用于存放 solidity 合约文件
-  palce scripts files such as deploying contracts in scripts<br>用于存放脚本文件，如部署合约的脚本
-  palce testing scripts named with `contractName.test.js` in test<br>用于存放测试脚本，通常以 `contractName.test.js` 的形式命名
- `hardhat.config.js` is config file of hardhat<br>是 hardhat 的配置文件

### Configuration of hardhat 配置 hardhat

`hardhat.config.js` config file example

```js
require('@nomiclabs/hardhat-waffle');
require('dotenv').config();

module.exports = {
  networks: {
    // hardhat build-in testing network (optional)
    // hardhat 内置测试网络（选填）
    hardhat: {
      // a fixed gasPrice could be set, it will be useful when testing gas consumption
      // 可以设置一个固定的gasPrice，在测试gas消耗的时候会很有用
      gasPrice: 1000000000,
    },
    // you could config arbitrary network
    // goerli testing network
    // 你可以在这里配置任意网络
    // goerli 测试网络
    goerli: {
      // place INFURA_ID to yours
      // url: 'https://goerli.infura.io/v3/{INFURA_ID}',
      // 请将 INFURA_ID 替换成你自己的
      // url: 'https://goerli.infura.io/v3/{INFURA_ID}',
      url: 'https://goerli.infura.io/v3/' + process.env.INFURA_ID, 
      //<---- 在.env文件中配置自己的INFURA_ID

      //  place multiple privateKeyX to yours
      // 填写测试账户的私钥，可填写多个
      accounts: [process.env.PRIVATE_KEY, ...]
    }
  },
  solidity: {
    version: "0.8.0", // version of compiling contract, required // 合约编译的版本，必填
    settings: { // setting of compile, optional// 编译设置，选填
      optimizer: {  // setting of optimizing // 优化设置
        enabled: true,
        runs: 200
      }
    }
  },

  // config project paths, any path could be specified, The following is a common template
  // files in sources, test, scripts will be executed one by one
  // 项目路径配置，可指定任意路径，但下列是常用的一种结构
  // sources, tests, scripts 下的目录文件会被自动逐一执行
  paths: {
    sources: "./contracts", // directory of contracts // 合约目录
    tests: "./test",  // directory of test files // 测试文件目录
    cache: "./cache", // cache directory, generated by hardhat  // 缓存目录，由hardhat自动生成
    artifacts: "./artifacts" // directory of compiling result, generated by hardhat // 编译结果目录，由hardhat自动生成
  },
  // setting of testing framework  // 测试框架设置
  mocha: {
    timeout: 20000  // max waiting time of running unit test // 运行单元测试的最大等待时间
  }
}
```

### Build-in hardhat network 内置 hardhat 网络

hardhat has a special, secure and build-in testing network, named `hardhat`, you don't need a special configuration for it. The network will follow the mechanism in real block chain network, and it will generate 10 test accounts for you (just like truffle).<br>hardhat 内置了一个特殊的安全测试网络，其名称也叫 `hardhat`, 通常你不需要对他进行特殊配置。该网络会模拟真实区块链网络的运行机制，并为你生成好 10 个测试账户（和 truffle 类似）。

### Using plugins 使用插件

Plugins have many functions in Hardhat, you could choose arbitrary plugins as a developer<br>Hardhat 的很多功能都来自于插件，而作为开发者，你可以自由选择想使用的插件。

Waffle plugins could make hardhat Integrated with waffle framework<br>例如常用的 waffle 插件，使得 hardhat 可以集成 waffle 框架，进行开发，测试，部署。

```js
// hardhat.config.js
require('@nomiclabs/hardhat-waffle'); // hardhat waffle plugin
...
```

### Install dependencies 安装依赖

1. install nodejs (ignore)  安装 nodejs （略）

2. install project dependencies:安装项目依赖：

   ```sh
   npm install --save-dev @nomiclabs/hardhat-waffle ethereum-waffle chai @nomiclabs/hardhat-ethers ethers dotenv
   ```

   or use yarn to intall (yarn installed firstly)

   ```sh
   yarn add -D hardhat-deploy-ethers ethers chai chai-ethers mocha @types/chai @types/mocha dotenv
   ```

3. config private key and network:

   create `.env` file in the directory of project, add private key and infura node to the file

   ```js
   PRIVATE_KEY = xxxxxxxxxxxxxxxx; // place your private key
   INFURA_ID = yyyyyyyy; // place infura node
   ```

## Usage 用法

useage of hardhat <br>hardhat 的用法

### Compile 编译

Run the command, hardhat will compile all contracts file in directory of `sources`, the default path is `./contracts`

运行如下命令，hardhat 会自动编译配置中 `sources` 路径下的所有合约文件，默认是 `./contracts` 路径。

```sh
npx hardhat compile
```

### Test

Run the command, hardhat will compile all test files in directory of `tests`, the default path is `./test`<br>

运行如下命令，hardhat 会自动运行配置中 `tests` 路径下的所有测试文件，默认是 `./test` 路径。

```sh
npx hardhat test
```

you could also specify some test files to run it<br>

也可以指定运行某个特定测试文件

```sh
npx hardhat test ./test/Greeter.test.js
```

### Run

Run the specified script. If you are not, it will run on hardhat's build-in network by default(Hardhat Network).<br>运行指定脚本。如果不指定运行网络，会默认在 hardhat 内置网络内运行 (Hardhat Network)。

```sh
npx hardhat run ./scripts/deploy.js
```

Run the specified network, such as the contract deployed on goerli test network(make sure that the wallet could pay the gas)<br>指定运行的网络，例如在 goerli 测试网部署合约(请确保钱包地址在 goerli 测试网有足够的 gas 才能成功部署)

```sh
npx hardhat run ./scripts/deploy.js --network goerli
```

### Verify

Verify the smart contract, here is an example of `goerli`.<br>验证智能合约，这里以`goerli`举例。

Add the following configuration to `hardhat.config.js`:

```js
 etherscan: {
   apiKey: "<etherscan的api key>",
 }
```

Run script:

```shell
npx hardhat verify --network goerli <your contract address>
```

### Task

hardhat preset some task itself, such as compiling contract, running testing scripts. Those are build-in hardhat tasks.hardhat

 本身预设了一些程序任务，例如编译合约，运行测试文件，这些其实在 hardhat 中是预先配置好的任务。

Actually you could also customize some tasks, such as printing status of the current network's account

实际上你也可以自定义一些 task，比如打印一下当前网络中的账户状态：

```js
// hardhat.config.js
...

task('accounts', 'Prints the list of accounts', async () => {
  const accounts = await ethers.getSigners();

  for (const account of accounts) {
    console.log(account.address);
  }
});

...
```

run task

```sh
npx hardhat accounts
```

terminal will print 10 addresses of testing account

命令行会打印出 10 个测试账户地址

```sh
0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
...
```

### Console

The console mode of hardhat could interact with chain in real-time, where the hardhat build-in network is started by default.

hardhat 的控制台模式，实时与链上交互。默认会启动 hardhat 内置网络。

```sh
npx hardhat console
```

we can directly use build-in ethers and web3 library, no need to import.

控制内置 ethers 和 web3 库，可以直接使用，无须引入。

```js
// hardhat console mode:
// directly use async/await
> await ethers.provider.getBlockNumber()  // 0
```

### console.log debug

hardhat provide the `console.log()` method to print logs, debug and test when running the contract . **Only valid in hardhat build-int network**

hardhat 提供了一个 `console.log()` 方法，可以在合约运行时打印日志，方便调试和测试。**此方法仅在 hardhat 内置网络中运行有效。**

you can use is by importing `hardhat/console.sol` in your contract

在合约中引入 `hardhat/console.sol` 即可使用：

```solidity
import "hardhat/console.sol";

contract Greeter {
  ...

  function setGreeting(string memory _greeting) public {
    console.log("Changing greeting from '%s' to '%s'", greeting, _greeting);
    greeting = _greeting;
  }

}
```

When running test scripts, you can check logs:

```sh
Changing greeting from 'Hello, world!' to 'hello Dapp-Learning!'
```

## Steps 实操流程

### Compile and test 编译和测试

1. compile the contract 编译合约

   ```bash
   npx hardhat compile
   ```

2. batch run test scripts 批量运行测试脚本

   ```bash
   npx hardhat test
   ```

3. deploy to test network 部署到测试网：

   ```bash
   npx hardhat run scripts/deploy.js --network <network-name>
   ```

   `network-name` should be replaced with your networks, `goerli` is a choice which exists in the config file.

4. Verify smart contract 验证智能合约

   ```bash
   npx hardhat verify --network goerli <network-name> <contract-address>
   ```

    `network-name` : the name of the network you specify, here you can replace it with `goerli`, which corresponds to the network name in the configuration file.

    `network-name` ：你指定的网络名称，这里可以换成 `goerli`，对应配置文件中的网络名称。
   
   `contract-address` : The address of the contract deployed in the previous step.
   
   `contract-address` ：上一步部署的合约地址。

## Reference

- hardhat offical document: <https://hardhat.org/guides/project-setup.html>
- hardhat chinese document: <https://learnblockchain.cn/docs/hardhat/getting-started/>
- the usage of ethers.js and hardhat : <https://www.bilibili.com/video/BV1Pv411s7Nb>
- <https://rahulsethuram.medium.com/the-new-solidity-dev-stack-buidler-ethers-waffle-typescript-tutorial-f07917de48ae>
- erc20 openzepplin introduction: <https://segmentfault.com/a/1190000015400380>
