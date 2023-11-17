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

```bash
truffle migrate --network sepolia --reset

Compiling your contracts...
===========================
> Everything is up to date, there is nothing to compile.


Migrations dry-run (simulation)
===============================
> Network name:    'sepolia-fork'
> Network id:      11155111
> Block gas limit: 30000000 (0x1c9c380)


1_initial_migration.js
======================

   Deploying 'SimpleToken'
   -----------------------
   > block number:        4708615
   > block timestamp:     1700185196
   > account:             0x840bAEb9979233405c5626DFe99C288c3173c45F
   > balance:             3.962513634803138664
   > gas used:            3213663 (0x31095f)
   > gas price:           10 gwei
   > value sent:          0 ETH
   > total cost:          0.03213663 ETH

   -------------------------------------
   > Total cost:          0.03213663 ETH

Summary
=======
> Total deployments:   1
> Final cost:          0.03213663 ETH




Starting migrations...
======================
> Network name:    'sepolia'
> Network id:      11155111
> Block gas limit: 30000000 (0x1c9c380)


1_initial_migration.js
======================

   Deploying 'SimpleToken'
   -----------------------
   > transaction hash:    0x639b2fa0da1b8f927a36c9f37b2f38de03b0bce81e1781c856fcc33fd56de702
   > Blocks: 1            Seconds: 14
   > contract address:    0x53918F83d9EdCA49F8b759521817eee28d33bc2e
   > block number:        4708624
   > block timestamp:     1700185224
   > account:             0x840bAEb9979233405c5626DFe99C288c3173c45F
   > balance:             3.962513634803138664
   > gas used:            3213663 (0x31095f)
   > gas price:           10 gwei
   > value sent:          0 ETH
   > total cost:          0.03213663 ETH

   > Saving artifacts
   -------------------------------------
   > Total cost:          0.03213663 ETH

Summary
=======
> Total deployments:   1
> Final cost:          0.03213663 ETH
```

#### faq
```bash
Compiling your contracts...
===========================
> Everything is up to date, there is nothing to compile.


Migrations dry-run (simulation)
===============================
> Network name:    'sepolia-fork'
> Network id:      11155111
> Block gas limit: 30000000 (0x1c9c380)


1_initial_migration.js
======================

   Deploying 'SimpleToken'
   -----------------------
   > block number:        4708561
   > block timestamp:     1700184469
   > account:             0x840bAEb9979233405c5626DFe99C288c3173c45F
   > balance:             3.962513634803138664
   > gas used:            3213663 (0x31095f)
   > gas price:           10 gwei
   > value sent:          0 ETH
   > total cost:          0.03213663 ETH

   -------------------------------------
   > Total cost:          0.03213663 ETH

Summary
=======
> Total deployments:   1
> Final cost:          0.03213663 ETH


Error: There was a timeout while attempting to connect to the network at undefined.
       Check to see that your provider is valid.
       If you have a slow internet connection, try configuring a longer timeout in your Truffle config. Use the networks[networkName].networkCheckTimeout property to do this.
    at Timeout._onTimeout (/usr/local/lib/node_modules/truffle/build/webpack:/packages/provider/index.js:65:1)
    at listOnTimeout (node:internal/timers:569:17)
    at processTimers (node:internal/timers:512:7)

```



The error message suggests that there was a timeout while attempting to connect to the network named 'sepolia-fork.' Here are a few steps you can take to troubleshoot and resolve the issue:<br>

错误消息表明在尝试连接到名为'sepolia-fork'的网络时发生了超时。以下是一些建议，可用于排除故障并解决问题：

1. **Check Network Configuration:****检查网络配置**:

   - Ensure that the network configuration in your Truffle project is correctly set up. Verify that the 'sepolia-fork' network is defined in your Truffle configuration file (`truffle-config.js` or `truffle.js`).<br>确保Truffle项目中的网络配置设置正确。验证在Truffle配置文件（`truffle-config.js`或`truffle.js`）中是否正确定义了名为'sepolia-fork'的网络。
   - Double-check the network configuration for any typos or missing information.<br>仔细检查网络配置，查看是否存在拼写错误或缺失的信息。

2. **Provider Validity:****提供程序的有效性**:

   - Confirm that the provider specified for the 'sepolia-fork' network is valid and accessible. It seems like there might be an issue with the connection to the specified provider.<br>确认为'sepolia-fork'网络指定的提供程序是否有效且可访问。似乎与连接到指定提供程序存在问题。
   - If you are using a custom provider, make sure it's running and accessible.<br>如果使用自定义提供程序，请确保其正在运行并且可访问。

3. **Internet Connection:****互联网连接：**

   - Check your internet connection to ensure it's stable. The error message mentions a timeout, which could be related to a slow or unstable connection.<br>检查您的互联网连接以确保其稳定。错误消息提到了超时，这可能与互联网连接缓慢或不稳定有关。

4. **Timeout Configuration:****超时配置:**

   - If your internet connection is slow or if the network is taking longer to respond, you can try configuring a longer timeout in your Truffle configuration.如果互联网连接较慢或网络响应时间较长，可以尝试在Truffle配置中配置较长的超时。
   - Look for the `networkCheckTimeout` property in your Truffle configuration file and adjust its value to a higher timeout duration.查找Truffle配置文件中的`networkCheckTimeout`属性，并将其值调整为更长的超时持续时间。

   ```
   javascriptCopy code
   networks: {
     'sepolia-fork': {
       // other configurations
       networkCheckTimeout: 1000000, // Adjust the timeout value as needed
     },
   },
   ```

   Set the `networkCheckTimeout` to a higher value, such as 1000000 (1 second) or more, depending on your network conditions.将`networkCheckTimeout`设置为较高的值，例如1000000（1秒）或更长，具体取决于您的网络条件

5. **Provider URL:****提供程序URL：**

   - Ensure that the provider URL (if applicable) is correct. If you're using a service like Infura, make sure the URL is up-to-date and properly configured.确保提供程序URL（如果适用）是正确的。如果使用类似Infura的服务，请确保URL是最新且正确配置的。

6. **Check for Updates:****检查更新**

   - Ensure that you are using the latest version of Truffle. You can update Truffle using the following command:确保您使用的是最新版本的Truffle。您可以使用以下命令更新Truffle：

     ```
     npm install -g truffle
     ```

After checking and adjusting these aspects, try running the migration again:在检查和调整这些方面后，尝试再次运行迁移命令：

```
truffle migrate --network sepolia --reset
```

If the issue persists, you may need to further investigate the network setup and the specific conditions of the 'sepolia-fork' network you are connecting to.<br>如果问题仍然存在，可能需要进一步调查网络设置和您连接的'sepolia-fork'网络的具体条件。

## Test contracts on Infura 在 infura 测试合约
Under `test` folder, there are two types, `sol` and `js`. `Truffle` supports both types, but if we use `infura`, we can't run `sol` file. So we only use `js` file as our test file. <br>
在 test 目录下存在 sol 和 js 类型的文件，truffle 支持这两种类型的测试文件。但目前测试发现，如果连接的测试网络为 infura ，则执行
sol 的测试文件会报失败。所以，这里我们连接到 infura 进行测试时，只能使用 js 的测试文件。


```bash
truffle test ./test/simpletoken.js --network sepolia
```
```bash
Using network 'sepolia'.


Compiling your contracts...
===========================
> Everything is up to date, there is nothing to compile.


  Contract: SimpleToken
    ✔ Should put 100000 to the 0x840bAEb9979233405c5626DFe99C288c3173c45F (6215ms)


  1 passing (11s)

```
## Test in local
```bash
$ truffle develop
Truffle Develop started at http://127.0.0.1:9545/

Truffle Develop started at http://127.0.0.1:9545/

Accounts:
(0) 0xc93dab3a469603e98e7efa21009e4e54e1f979e5
(1) 0x1e2b3cb506d1f19f3bf3370000a71852da88d343
(2) 0xa13b714776e9e714ae0ab8d2b817fe7cd717083d
(3) 0x7b24843f4735f64acee19b96052c240aaea23c63
(4) 0xfba813a1673cd6132cf7ed2e1d9ff40b92598154
(5) 0xcaa5e98911e38239cce1eaeb272d62db25be6d0f
(6) 0xeaf7a504d7ca7225c2b283e0a1918eac0cf0c653
(7) 0x457f7fdbb275e8c58a622c778c1289e80e301075
(8) 0x03861b31b605cb7c03883fe94fecbe0f25a1eabf
(9) 0x6d4ac3b55277a291a67e58cd76ceacfeb17414f1

Private Keys:
(0) b0c86d657ad87e99afadcad7f71e502a7e7236b8d422cf760aeacf5710431727
(1) f1d063029c515065e7461076bc5c2b8bf257d2f18dba9171f7fe6d0cedf43782
(2) a244ed5a22fd75f0f9f80a50b229a0bc15ba478e4579c0be09cf8b30793375bc
(3) eaba2f299a737ff1aab77f0dc970a8d342c683ce36855e1c1156449d6205c2cc
(4) 2246cf9c4fbb5178a833e89c380b583dac7154606de9abe23097d7788e5b0e6f
(5) 927aacbd3744d211877bb0c85f979194b4535e39df8a3a625936669233b0244a
(6) 90ec7a4529d62c09f884220b0f1770b7de4b412cecd0e7d82b9ef0cddae27845
(7) 75e8d944d531337019cce514f6f10ddab2af62a337e89f263a7f97ebbf8511e7
(8) 52c7752fd95c15f3f4e7b96a87582529f0657c4cffa7cd66e3030894d357b112
(9) 1b78f4116edadb988d5d1ff8bdaa619db64d54e67cde8bd797a4d51f8e3c6430

```

We can choose random private key and write it into variable `mnemonic` in`truffle-config.js`.


```js

// before
const mnemonic = fs.readFileSync('./sk.txt').toString().trim()

// after
const mnemonic = "0a8d9e2a470aedfabe279f16f629c5054a47d69b7d66d17ba65cdd7ca99876e1"
```

And then we need to change the `host` in `development` to localhost, and change the port to `9545` which is `truffle develop` given, keep `network_id` the same.

```js
development: {
  host: "127.0.0.1",
  port: 9545,
  network_id: "*"
},
```

After finish the above steps, we can run `truffle compile`, `truffle migrate` and `truffle test`to test the smart contracts.


```bash
> Artifacts written to C:\Users\Highland\AppData\Local\Temp\test--33840-ApHyOzehxOdp
> Compiled successfully using:
   - solc: 0.8.0+commit.c7dfd78e.Emscripten.clang



  TestSimpleToken
    √ testInitialBalanceUsingDeployedContract (1802ms)
    √ testTransfer (1723ms)

  Contract: SimpleToken
    √ Should put 100000 to the 0x9A3f188e2C161ff4482AEB045546644B8d67120B (1773ms)
    √ Transfer 100 to other account (2342ms)


  4 passing (32s)

```



