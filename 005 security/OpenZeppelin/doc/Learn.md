## Learn

### Setting up a Node project

New software industries often start out with every project sharing the same technology stack. The Ethereum ecosystem is no exception, and the language of choice is [JavaScript](https://en.wikipedia.org/wiki/JavaScript). Many Ethereum libraries, including OpenZeppelin software, are written in JavaScript or one of its variants.

JavaScript code is traditionally run on a web browser as part of a website, but it can be also executed as a standalone process using [Node](https://nodejs.org/).

This guide will help you get your Node development environment set up, which you’ll need to use the different OpenZeppelin tools and other third party products.

|      | If you are already familiar with Node, npm and Git, feel free to skip this guide! |
| ---- | ------------------------------------------------------------ |
|      |                                                              |

#### Installing Node

There are multiple ways to get Node on your machine: you can get it either via a [package manager](https://nodejs.org/en/download/package-manager/) or by downloading [the installer](https://nodejs.org/en/download/prebuilt-installer) directly.

|      | If you are running Windows consider using [Windows Subsystem for Linux](https://docs.microsoft.com/en-us/windows/nodejs/setup-on-wsl2) as much of the ecosystem is written for Linux. |
| ---- | ------------------------------------------------------------ |
|      |                                                              |

Once you’re done, run `node --version` on a terminal to check your installation: any [active or maintenance version](https://nodejs.org/en/about/previous-releases) should be compatible with most Ethereum software.

```
$ node --version
v20.17.0
```

#### Creating a project

JavaScript software is often bundled in *packages*, which are distributed via the [npm registry](https://www.npmjs.com/). A package is simply a directory that contains a file called `package.json`, describing the package’s name, version, content, and others. When you build your own project, you will be creating a package, even if you don’t plan to distribute it.

All Node installations include a command-line client for the npm registry, which you’ll use while developing your own projects. To start a new project, create a directory for it:

```bash
$ mkdir learn && cd learn
```

Then we can initialize it:

```bash
npm init -y 
```

Simple as that! Your newly created `package.json` file will evolve as your project grows, such as when installing dependencies with `npm install`.

|      | JavaScript and npm are some of the most used software tools in the world: if you’re ever in doubt, you’ll find plenty of information about them online. |
| ---- | ------------------------------------------------------------ |
|      |                                                              |

#### Using npx 

There are two broads type of packages stored in the npm registry: *libraries* and *executables*. Installed libraries are used like any other piece of JavaScript code, but executables are special.

A third binary was included when installing node: [`npx`](https://blog.npmjs.org/post/162869356040/introducing-npx-an-npm-package-runner). This is used to run executables installed locally in your project.

Whilst [Hardhat](https://hardhat.org/) can be installed globally we recommend installing locally in each project so that you can control the version on a project by project basis.

For clarity we’ll display the full command in our guides including `npx` so we don’t get errors due to the binary not being in the system path:

```bash
$ hardhat init
hardhat: command not found
$ npx hardhat init
👷 Welcome to Hardhat v2.22.12 👷‍
? What do you want to do? …
```

Make sure you are inside your project’s directory when running `npx`! Otherwise, it will download the full executable again just to run that command, which most of the time is *not* what you want.

#### Tracking with Version Control

Before you get coding, you should add [version control software](https://en.wikipedia.org/wiki/Version_control) to your project to track changes.

By far, the most used tool is [Git](https://git-scm.com/), often in conjunction with [GitHub](https://github.com/) for hosting purposes. Indeed, you will find the full source code and history of all OpenZeppelin software in our [GitHub repository](https://github.com/OpenZeppelin).

### Developing smart contracts 

Welcome to the exciting world of smart contract development! This guide will let you get started writing Solidity contracts by going over the following:

- [Setting up a Solidity Project](https://docs.openzeppelin.com/learn/developing-smart-contracts#setting-up-a-solidity-project)
- [Compiling Solidity Source Code](https://docs.openzeppelin.com/learn/developing-smart-contracts#compiling-solidity-source-code)
- [Adding More Contracts](https://docs.openzeppelin.com/learn/developing-smart-contracts#adding-more-contracts)
- [Using OpenZeppelin Contracts](https://docs.openzeppelin.com/learn/developing-smart-contracts#using-openzeppelin-contracts)

#### Setting up a Project

The first step after [creating a project](https://docs.openzeppelin.com/learn/setting-up-a-node-project#creating-a-project) is to install a development tool.

The most popular development frameworks for Ethereum are [Hardhat](https://hardhat.org/) and [Foundry](https://github.com/foundry-rs/foundry). Each has their strengths and it is useful to be comfortable using all of them.

In these guides we will show how to develop, test and deploy smart contracts using Hardhat, and we cover its most common use with [ethers.js](https://docs.ethers.io/).

To get started with Hardhat we will install it in our [project directory](https://docs.openzeppelin.com/learn/setting-up-a-node-project#creating-a-project).

```bash
 npm install --save-dev hardhat
```

Once installed, we can run `npx hardhat`. This will create a Hardhat config file (`hardhat.config.js`) in our project directory.

```bash
$ npx hardhat
888    888                      888 888               888
888    888                      888 888               888
888    888                      888 888               888
8888888888  8888b.  888d888 .d88888 88888b.   8888b.  888888
888    888     "88b 888P"  d88" 888 888 "88b     "88b 888
888    888 .d888888 888    888  888 888  888 .d888888 888
888    888 888  888 888    Y88b 888 888  888 888  888 Y88b.
888    888 "Y888888 888     "Y88888 888  888 "Y888888  "Y888

👷 Welcome to Hardhat v2.22.12 👷‍

✔ What do you want to do? · Create an empty hardhat.config.js
Config file created
```

#### First contract

We store our Solidity source files (`.sol`) in a `contracts` directory. This is equivalent to the `src` directory you may be familiar with from other languages.

We can now write our first simple smart contract, called `Box`: it will let people store a value that can be later retrieved.

We will save this file as `contracts/Box.sol`. Each `.sol` file should have the code for a single contract, and be named after it.

```solidity
// contracts/Box.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Box {
    uint256 private _value;

    // Emitted when the stored value changes
    event ValueChanged(uint256 value);

    // Stores a new value in the contract
    function store(uint256 value) public {
        _value = value;
        emit ValueChanged(value);
    }

    // Reads the last stored value
    function retrieve() public view returns (uint256) {
        return _value;
    }
}
```

#### Compiling Solidity

The Ethereum Virtual Machine (EVM) cannot execute Solidity code directly: we first need to compile it into EVM bytecode.

Our `Box.sol` contract uses Solidity 0.8 so we need to first [configure Hardhat to use an appropriate solc version](https://hardhat.org/config/#solidity-configuration).

We specify a Solidity 0.8 solc version in our `hardhat.config.js`.



```js
// hardhat.config.js

/**
 * @type import('hardhat/config').HardhatUserConfig
 */
 module.exports = {
  solidity: "0.8.24",
};
```

Compiling can then be achieved by running a single compile command:

```bash
$ npx hardhat compile
Compiled 1 Solidity file successfully (evm target: paris).
```

The [`compile`](https://hardhat.org/guides/compile-contracts.html#compiling-your-contracts) built-in task will automatically look for all contracts in the `contracts` directory, and compile them using the Solidity compiler using the configuration in [`hardhat.config.js`](https://hardhat.org/config/#solidity-configuration).

You will notice an `artifacts` directory was created: it holds the compiled artifacts (bytecode and metadata), which are .json files. It’s a good idea to add this directory to your `.gitignore`.

#### Adding more contracts

As your project grows, you will begin to create more contracts that interact with each other: each one should be stored in its own `.sol` file.

To see how this looks, let’s add a simple access control system to our `Box` contract: we will store an administrator address in a contract called `Auth`, and only let `Box` be used by those accounts that `Auth` allows.

Because the compiler will pick up all files in the `contracts` directory and subdirectories, you are free to organize your code as you see fit. Here, we’ll store the `Auth` contract in an `access-control` subdirectory:

```solidity
// contracts/access-control/Auth.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Auth {
    address private _administrator;

    constructor(address deployer) {
        // Make the deployer of the contract the administrator
        _administrator = deployer;
    }

    function isAdministrator(address user) public view returns (bool) {
        return user == _administrator;
    }
}
```

To use this contract from `Box` we use an `import` statement, referring to `Auth` by its relative path:

```solidity
// contracts/Box.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Import Auth from the access-control subdirectory
import "./access-control/Auth.sol";

contract Box {
    uint256 private _value;
    Auth private _auth;

    event ValueChanged(uint256 value);

    constructor() {
        _auth = new Auth(msg.sender);
    }

    function store(uint256 value) public {
        // Require that the caller is registered as an administrator in Auth
        require(_auth.isAdministrator(msg.sender), "Unauthorized");

        _value = value;
        emit ValueChanged(value);
    }

    function retrieve() public view returns (uint256) {
        return _value;
    }
}
```



Separating concerns across multiple contracts is a great way to keep each one simple, and is generally a good practice.

However, this is not the only way to split your code into modules. You can also use *inheritance* for encapsulation and code reuse in Solidity, as we’ll see next.

#### Using OpenZeppelin Contracts

Reusable modules and libraries are the cornerstone of great software. [**OpenZeppelin Contracts**](https://docs.openzeppelin.com/contracts/5.x/) contains lots of useful building blocks for smart contracts to build on. And you can rest easy when building on them: they’ve been the subject of multiple audits, with their security and correctness battle-tested.

##### About inheritance

Many of the contracts in the library are not standalone, that is, you’re not expected to deploy them as-is. Instead, you will use them as a starting point to build your own contracts by adding features to them. Solidity provides *multiple inheritance* as a mechanism to achieve this: take a look at the [Solidity documentation](https://solidity.readthedocs.io/en/latest/contracts.html#inheritance) for more details.

For example, the [`Ownable`](https://docs.openzeppelin.com/contracts/5.x/api/access#Ownable) contract marks the deployer account as the contract’s owner, and provides a modifier called `onlyOwner`. When applied to a function, `onlyOwner` will cause all function calls that do not originate from the owner account to revert. Functions to [transfer](https://docs.openzeppelin.com/contracts/5.x/api/access#Ownable-transferOwnership-address-) and [renounce](https://docs.openzeppelin.com/contracts/5.x/api/access#Ownable-renounceOwnership--) ownership are also available.

When used this way, inheritance becomes a powerful mechanism that allows for modularization, without forcing you to deploy and manage multiple contracts.

##### Importing OpenZeppelin Contracts

The latest published release of the OpenZeppelin Contracts library can be downloaded by running:

```
 npm install @openzeppelin/contracts
```

You should always use the library from these published releases: copy-pasting library source code into your project is a dangerous practice that makes it very easy to introduce security vulnerabilities in your contracts.

To use one of the OpenZeppelin Contracts, `import` it by prefixing its path with `@openzeppelin/contracts`. For example, in order to replace our own [`Auth`](https://docs.openzeppelin.com/learn/developing-smart-contracts#auth-contract) contract, we will import `@openzeppelin/contracts/access/Ownable.sol` to add access control to `Box`:

```solidity
// contracts/Box.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Import Ownable from the OpenZeppelin Contracts library
import "@openzeppelin/contracts/access/Ownable.sol";

// Make Box inherit from the Ownable contract
contract Box is Ownable {
    uint256 private _value;

    event ValueChanged(uint256 value);

    constructor() Ownable(msg.sender) {}

    // The onlyOwner modifier restricts who can call the store function
    function store(uint256 value) public onlyOwner {
        _value = value;
        emit ValueChanged(value);
    }

    function retrieve() public view returns (uint256) {
        return _value;
    }
}
```

The [OpenZeppelin Contracts documentation](https://docs.openzeppelin.com/contracts/5.x/) is a great place to learn about developing secure smart contract systems. It features both guides and a detailed API reference: see for example the [Access Control](https://docs.openzeppelin.com/contracts/5.x/access-control) guide to know more about the `Ownable` contract used in the code sample above.

### Deploying and interacting with smart contracts

Unlike most software, smart contracts don’t run on your computer or somebody’s server: they live on the Ethereum network itself. This means that interacting with them is a bit different from more traditional applications.

This guide will cover all you need to know to get you started using your contracts, including:

- [Setting up a Local Blockchain](https://docs.openzeppelin.com/learn/deploying-and-interacting#local-blockchain)
- [Deploying a Smart Contract](https://docs.openzeppelin.com/learn/deploying-and-interacting#deploying-a-smart-contract)
- [Interacting from the Console](https://docs.openzeppelin.com/learn/deploying-and-interacting#interacting-from-the-console)
- [Interacting Programmatically](https://docs.openzeppelin.com/learn/deploying-and-interacting#interacting-programmatically)

#### Setting up a Local Blockchain

Before we begin, we first need an environment where we can deploy our contracts. The Ethereum blockchain (often called "mainnet", for "main network") requires spending real money to use it, in the form of Ether (its native currency). This makes it a poor choice when trying out new ideas or tools.

To solve this, a number of "testnets" (for "test networks") exist: these include the Sepolia and Holesky blockchains. They work very similarly to mainnet, with one difference: you can get Ether for these networks for free, so that using them doesn’t cost you a single cent. However, you will still need to deal with private key management, blocktimes of 12 seconds or more, and actually getting this free Ether.

During development, it is a better idea to instead use a *local* blockchain. It runs on your machine, requires no Internet access, provides you with all the Ether that you need, and mines blocks instantly. These reasons also make local blockchains a great fit for [automated tests](https://docs.openzeppelin.com/learn/writing-automated-tests#setting-up-a-testing-environment).

|      | If you want to learn how to deploy and use contracts on a *public* blockchain, like the Ethereum testnets, head to our [Connecting to Public Test Networks](https://docs.openzeppelin.com/learn/connecting-to-public-test-networks) guide. |
| ---- | ------------------------------------------------------------ |
|      |                                                              |

Hardhat comes with a local blockchain built-in, the [Hardhat Network](https://hardhat.org/hardhat-network/).

Upon startup, Hardhat Network will create a set of unlocked accounts and give them Ether.

```bash
npx hardhat node
```

Hardhat Network will print out its address, `http://127.0.0.1:8545`, along with a list of available accounts and their private keys.

Keep in mind that every time you run Hardhat Network, it will create a brand new local blockchain - the state of previous runs is **not** preserved. This is fine for short-lived experiments, but it means that you will need to have a window open running Hardhat Network for the duration of these guides.

|      | Hardhat will always spin up an instance of **Hardhat Network** when no network is specified and there is no default network configured or the default network is set to `hardhat`. |
| ---- | ------------------------------------------------------------ |
|      |                                                              |

```

You can also run an actual Ethereum node in development mode. These are a bit more complex to set up, and not as flexible for testing and development, but are more representative of the real network.
```

#### Deploying a Smart Contract

In the [Developing Smart Contracts guide](https://docs.openzeppelin.com/learn/developing-smart-contracts) we set up our development environment.

If you don’t already have this setup, please [create](https://docs.openzeppelin.com/learn/setting-up-a-node-project#creating-a-project) and [setup](https://docs.openzeppelin.com/learn/developing-smart-contracts#setting-up-a-solidity-project) the project and then [create](https://docs.openzeppelin.com/learn/developing-smart-contracts#first-contract) and [compile](https://docs.openzeppelin.com/learn/developing-smart-contracts#compiling-solidity-source-code) our Box smart contract.

With our project setup complete we’re now ready to deploy a contract. We’ll be deploying `Box`, from the [Developing Smart Contracts](https://docs.openzeppelin.com/learn/developing-smart-contracts#box-contract) guide. Make sure you have a copy of [Box](https://docs.openzeppelin.com/learn/developing-smart-contracts#box-contract) in `contracts/Box.sol`.

Hardhat uses either [declarative deployments](https://hardhat.org/hardhat-runner/docs/guides/deploying) or [scripts](https://hardhat.org/hardhat-runner/docs/advanced/scripts#writing-scripts-with-hardhat) to deploy contracts.

We will create a script to deploy our Box contract. We will save this file as `scripts/deploy.js`.

```javascript
// scripts/deploy.js
async function main () {
  // We get the contract to deploy
  const Box = await ethers.getContractFactory('Box');
  console.log('Deploying Box...');
  const box = await Box.deploy();
  await box.waitForDeployment();
  console.log('Box deployed to:', await box.getAddress());
}

main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });
```

We use [ethers](https://github.com/ethers-io/ethers.js) in our script, so we need to install it and the [@nomicfoundation/hardhat-ethers plugin](https://hardhat.org/hardhat-runner/plugins/nomicfoundation-hardhat-ethers).

```bash
npm install --save-dev @nomicfoundation/hardhat-ethers ethers
```

We need to add in our [configuration](https://hardhat.org/config/) that we are using the `@nomicfoundation/hardhat-ethers` plugin.

```javascript
// hardhat.config.js
require("@nomicfoundation/hardhat-ethers");

...
module.exports = {
...
};
```

Using the `run` command, we can deploy the `Box` contract to the local network ([Hardhat Network](https://docs.openzeppelin.com/learn/deploying-and-interacting#local-blockchain)):

```bash
npx hardhat run --network localhost scripts/deploy.js
Deploying Box...
Box deployed to: 0x5FbDB2315678afecb367f032d93F642f64180aa3
```



```
Hardhat doesn’t keep track of your deployed contracts. We displayed the deployed address in our script (in our example, 0x5FbDB2315678afecb367f032d93F642f64180aa3). This will be useful when interacting with them programmatically.
```

All done! On a real network this process would’ve taken a couple of seconds, but it is near instant on local blockchains.

```
If you got a connection error, make sure you are running a local blockchain in another terminal.

```



```
Remember that local blockchains do not persist their state throughout multiple runs! If you close your local blockchain process, you’ll have to re-deploy your contracts.
```

#### Interacting from the Console

With our `Box` contract [deployed](https://docs.openzeppelin.com/learn/deploying-and-interacting#deploying-a-smart-contract), we can start using it right away.

We will use the [Hardhat console](https://hardhat.org/guides/hardhat-console.html) to interact with our deployed `Box` contract on our localhost network.

```
We need to specify the address of our Box contract we displayed in our deploy script.
```

```

It’s important that we explicitly set the network for Hardhat to connect our console session to. If we don’t, Hardhat will default to using a new ephemeral network, which our Box contract wouldn’t be deployed to.
```



```bash
$ npx hardhat console --network localhost
Welcome to Node.js v20.17.0.
Type ".help" for more information.
> const Box = await ethers.getContractFactory('Box');
undefined
> const box = Box.attach('0x5FbDB2315678afecb367f032d93F642f64180aa3')
undefined
```

#### Sending transactions

`Box`'s first function, `store`, receives an integer value and stores it in the contract storage. Because this function *modifies* the blockchain state, we need to *send a transaction* to the contract to execute it.

We will send a transaction to call the `store` function with a numeric value:

```bash
> await box.store(42)
{
  hash: '0x3d86c5c2c8a9f31bedb5859efa22d2d39a5ea049255628727207bc2856cce0d3',
...
```

#### Querying state

`Box`'s other function is called `retrieve`, and it returns the integer value stored in the contract. This is a *query* of blockchain state, so we don’t need to send a transaction:

```bash
> await box.retrieve()
42n
```

Because queries only read state and don’t send a transaction, there is no transaction hash to report. This also means that using queries doesn’t cost any Ether, and can be used for free on any network.

```

Our Box contract returns uint256 which is too large a number for JavaScript so instead we get returned a big number object. We can display the big number as a string using (await box.retrieve()).toString().
```

```bash
> (await box.retrieve()).toString()
'42'
```



#### Interacting programmatically

The console is useful for prototyping and running one-off queries or transactions. However, eventually you will want to interact with your contracts from your own code.

In this section, we’ll see how to interact with our contracts from JavaScript, and use [Hardhat to run our script](https://hardhat.org/guides/scripts.html) with our Hardhat configuration.

```
Keep in mind that there are many other JavaScript libraries available, and you can use whichever you like the most. Once a contract is deployed, you can interact with it through any library!
```

##### Setup

Let’s start coding in a new `scripts/index.js` file, where we’ll be writing our JavaScript code, beginning with some boilerplate, including for [writing async code](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/async_function).

```javascript
// scripts/index.js
async function main () {
  // Our code will go here
}

main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });
```

We can test our setup by asking the local node something, such as the list of enabled accounts:

```javascript
// Retrieve accounts from the local node
const accounts = (await ethers.getSigners()).map(signer => signer.address);
console.log(accounts);
```

```

We won’t be repeating the boilerplate code on every snippet, but make sure to always code inside the main function we defined above!
```

Run the code above using `hardhat run`, and check that you are getting a list of available accounts in response.

```bash
npx hardhat run --network localhost ./scripts/index.js
[
  '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266',
  '0x70997970C51812dc3A010C7d01b50e0d17dc79C8',
...
]
```

These accounts should match the ones displayed when you started the [local blockchain](https://docs.openzeppelin.com/learn/deploying-and-interacting#local-blockchain) earlier. Now that we have our first code snippet for getting data out of a blockchain, let’s start working with our contract. Remember we are adding our code *inside* the `main` function we defined above.

##### Getting a contract instance

In order to interact with the [`Box`](https://docs.openzeppelin.com/learn/deploying-and-interacting#box-contract) contract we deployed, we’ll use an [ethers contract instance](https://docs.ethers.org/v6/api/contract/).

An ethers contract instance is a JavaScript object that represents our contract on the blockchain, which we can use to interact with our contract. To attach it to our deployed contract we need to provide the contract address.

```javascript
// Set up an ethers contract, representing our deployed Box instance
const address = '0x5FbDB2315678afecb367f032d93F642f64180aa3';
const Box = await ethers.getContractFactory('Box');
const box = Box.attach(address);
```

```
Make sure to replace the address with the one you got when deploying the contract, which may be different to the one shown here.
```

We can now use this JavaScript object to interact with our contract.

##### Calling the contract

Let’s start by displaying the current value of the `Box` contract.

We’ll need to call the read only `retrieve()` public method of the contract, and [await](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/await) the response:

```javascript
// Call the retrieve() function of the deployed Box contract
const value = await box.retrieve();
console.log('Box value is', value.toString());
```

This snippet is equivalent to the [query](https://docs.openzeppelin.com/learn/deploying-and-interacting#querying-state) we ran earlier from the console. Now, make sure everything is running smoothly by running the script again and checking the printed value:

```bash
$ npx hardhat run --network localhost ./scripts/index.js
Box value is 42
```

##### Sending a transaction

We’ll now send a transaction to `store` a new value in our Box.

Let’s store a value of `23` in our `Box`, and then use the code we had written before to display the updated value:

```javascript
// Send a transaction to store() a new value in the Box
await box.store(23);

// Call the retrieve() function of the deployed Box contract
const value = await box.retrieve();
console.log('Box value is', value.toString());
```

We can now run the snippet, and check that the box’s value is updated!

```bash
npx hardhat run --network localhost ./scripts/index.js
Box value is 23
```

### Writing automated smart contract tests

You may be wondering *how* we’re going to run these tests, since smart contracts are executed inside a blockchain. Using the actual Ethereum network would be very expensive, and while testnets are free, they are also slow (with blocktimes of 12 seconds or more). If we intend to run hundreds of tests whenever we make a change to our code, we need something better.

What we will use is called a *local blockchain*: a slimmed down version of the real thing, disconnected from the Internet, running on your machine. This will simplify things quite a bit: you won’t need to get Ether, and new blocks will be mined instantly.

#### Writing unit tests

We’ll use [Chai](https://www.chaijs.com/) assertions for our unit tests, which are available by installing the Hardhat Toolbox.

```bash
npm install --save-dev @nomicfoundation/hardhat-toolbox
```































