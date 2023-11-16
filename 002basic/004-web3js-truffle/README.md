# Web3js Truffle

## Introduction about Truffle
A world class development environment, testing framework and asset pipeline for blockchains using the Ethereum Virtual Machine (EVM), aiming to make life as a developer easier. With Truffle, you get:<br>
Truffle 是基于 Solidity 语言的一套开发框架，它简化了去中心化应用（Dapp）的构建和管理流程。本身是采用 Javascript 编写，支持智能合约的编译、部署和测试。truffle 开发框架提供了很多功能，简化了我们的开发、编译、部署与调试过程：

- Built-in smart contract compilation, linking, deployment and binary management.<br>
  内置了智能合约编译、链接、部署和二进制文件的管理
- Advanced debugging with breakpoints, variable analysis, and step functionality.<br>方便快速开发的合约自动化测试
- Use console.log in your smart contracts <br>在合约中使用console.log
- Deployments and transactions through MetaMask with Truffle Dashboard to protect your mnemonic.<br>通过 MetaMask 和 Truffle Dashboard 进行部署和交易，以保护您的助记词
- External script runner that executes scripts within a Truffle environment.<br>在 Truffle 环境中执行脚本的外部脚本运行程序。
- Interactive console for direct contract communication.<br>用于直接合约通信的交互式控制台。
- Automated contract testing for rapid development.<br>用于快速开发的自动化合同测试。
- Scriptable, extensible deployment & migrations framework.<br>可脚本化、可扩展的部署和迁移框架。
- Network management for deploying to any number of public & private networks.<br>用于部署到任意数量的公共和专用网络的网络管理。
- Package management with NPM, using the ERC190 standard.<br>使用ERC190标准的NPM包管理。
- Configurable build pipeline with support for tight integration.<br>支持紧密集成的可配置构建管道。

## Introduction about Project 项目说明
### Structure 项目结构
- `contracts/`: Directory for Solidity contracts  合约目录
- `migrations/`: Directory for scriptable deployment files 部署脚本文件目录
- `test/`: Directory for test files for testing your application and contracts 测试脚本目录
- `truffle-config.js`: Truffle configuration file 配置文件

### Files

1. `contracts/SimpleToken.sol`: It is an `erc20` smart contract written in `Solidity`.这是一个用 Solidity 编写的 erc20 代币 智能合约.
2. `migrations/1_initial_migration.js`: This file is the migration (deployment) script for the `Migrations` contract found in the `Migrations.sol` file.这是一个部署脚本，用来部署 Migrations 合约，对应 Migrations.sol 文件。

   > 1. Note that the filename is prefixed with a number and is suffixed by a description. The numbered prefix is required in order to record whether the migration ran successfully. The suffix is purely for human readability and comprehension.请注意，文件名以数字为前缀，并以描述为后缀。需要编号前缀才能记录迁移是否成功运行。后缀纯粹是为了便于阅读和理解。
   > 2. `Migrations.sol` is a separate Solidity file that manages and updates the status of your deployed smart contract. This file comes with every Truffle project, and is usually **not edited**.`Migrations.sol`是一个单独的Solidity文件，用于管理和更新已部署智能合约的状态。此文件随每个Truffle项目一起提供，通常不会进行编辑。

3. `truffle-config.js`: This is the Truffle configuration file, for setting network information and other project-related settings. The file is blank, but this is okay, as we'll be using a Truffle command that has some defaults built-in.Truffle 配置文件, 用来设置网络信息，和其他项目相关的设置。当我们使用内建的默认的 Truffle 命令时，这个文件留空也是可以的。

## How to run this project 运行工程
### 1. Install `Truffle`
```bash
   npm install -g truffle
```

### 2. Input your project ID and private key to the `.env` file. 配置env

   ```sh
   cp .env.example .env

   PRIVATE_KEY=xxxxxxxxxxxxxxxx
   INFURA_ID=yyyyyyyy
   ```
### 3. Test smart contracts 测试合约  

   ```bash
   truffle test
   ```

   > After running `truffle test` command, `truffle` will launch the built-in `test` network and run the test scripts in `test/` folder at the same time. If you want to run a specific test script, you can use `truffle test ./test/simpletoken.js` command.
   <br>这里，使用 "truffle test" 后，truffle 会启动内置的 test 网络，同时执行 测试 test 目录下的所有脚本，如果想单独测试某个脚本，可以
执行 "truffle test ./test/simpletoken.js"

### 4. Compile smart contracts 编译合约

   ```bash
   truffle compile
   ```

   > After running `truffle compile` command successfully, `truffle` will compile the smart contracts in `contracts/` folder and save the compiled bytecode in `build/contracts/` folder.
   > Here is the output:

   ```text
   Compiling .\contracts\SimpleToken.sol...

   Writing artifacts to .\build\contracts
   ```

### 5. Deploy smart contracts 部署合约
   In `truffle-config.js`, we can specify truffle to use the eth test network. However, after running `trffle migrate`, it reported there is no test network, so truffle didn't launch the built-in test network. We need to specify the test network as `sepolia` to deploy contracts manually.
<br>在 truffle-config.js 里面，可以配置 truffle 使用的以太网络，其中就包括 truffle test 使用的 "test" 网络。
这里，直接执行 truffle migrate 报没有找到 test 网络，因为 truffle 不会启动内置的 test 网络。所以这里我们使用 sepolia 进行 truffle 合约部署
   ```bash
   truffle migrate --network sepolia
   ```

   > If we run `truffle migrate` frequently, it may shows `Network update to date` and doesn't deploy the contracts. At that time, we need to run `truffle migrate --network goerli --reset` to reset the migration status.






