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

We will keep our test files in a `test` directory. Tests are best structured by mirroring the [`contracts` directory](https://docs.openzeppelin.com/learn/developing-smart-contracts#first-contract): for each `.sol` file there, create a corresponding test file.

Time to write our first tests! These will test properties of the `Box` contract [from previous guides](https://docs.openzeppelin.com/learn/developing-smart-contracts#box-contract): a simple contract that lets you `retrieve` a value the owner previously `store` d.

Create a `test` directory in your project root. We will save the test as `test/Box.test.js`. Each test `.js` file commonly has the tests for a single contract, and is named after it.

```javascript
// test/Box.test.js
// Load dependencies
const { expect } = require('chai');

// Start test block
describe('Box', function () {
  before(async function () {
    this.Box = await ethers.getContractFactory('Box');
  });

  beforeEach(async function () {
    this.box = await this.Box.deploy();
    await this.box.waitForDeployment();
  });

  // Test case
  it('retrieve returns a value previously stored', async function () {
    // Store a value
    await this.box.store(42);

    // Test if the returned value is the same one
    // Note that we need to use strings to compare the 256 bit integers
    expect((await this.box.retrieve()).toString()).to.equal('42');
  });
});
```



```

Many books have been written about how to structure unit tests. Check out the Moloch Testing Guide for a set of principles designed for testing Solidity smart contracts.
```

We are now ready to run our tests!

Running `npx hardhat test` will execute all tests in the `test` directory, checking that your contracts work the way you meant them to:

```bash
npx hardhat test


  Box
    ✓ retrieve returns a value previously stored


  1 passing (578ms)
```

’s also a very good idea at this point to set up a Continuous Integration service such as [CircleCI](https://circleci.com/) to make your tests run automatically every time you commit your code to GitHub.

#### Performing complex assertions

Many interesting properties of your contracts may be hard to capture, such as:

- verifying that the contract reverts on errors
- measuring by how much an account’s Ether balance changed
- checking that the proper events are emitted

We recommend using [Hardhat Chai Matchers](https://hardhat.org/hardhat-chai-matchers/docs/overview) to help you test all of these properties, and [Hardhat Network Helpers](https://hardhat.org/hardhat-network-helpers/docs/overview) for simulating time passing on the blockchain. These tools will let you write powerful assertions without having to worry about the low-level details of the underlying Ethereum libraries.

### Connecting to public test networks

After you have [written your contracts](https://docs.openzeppelin.com/learn/developing-smart-contracts), and [tried them out locally](https://docs.openzeppelin.com/learn/deploying-and-interacting) and [tested them thoroughly](https://docs.openzeppelin.com/learn/writing-automated-tests), it’s time to move to a persistent public testing environment, where you and your beta users can start interacting with your application.

We will use **public testing networks** (aka *testnets*) for this, which are networks that operate similar to the main Ethereum network, but where Ether has no value and is free to acquire - making them ideal for testing your contracts at no cost.

In this guide, we will use our beloved [`Box` contract](https://docs.openzeppelin.com/learn/developing-smart-contracts#box-contract), and deploy it to a testnet, while learning:

- [What test networks are available](https://docs.openzeppelin.com/learn/connecting-to-public-test-networks#testnet-list)
- [How to set up your project for working on a testnet](https://docs.openzeppelin.com/learn/connecting-to-public-test-networks#connecting-project-to-network)
- [How to deploy and interact with your testnet contract instances](https://docs.openzeppelin.com/learn/connecting-to-public-test-networks#working-on-testnet)

Remember that deploying to a public test network is a necessary step when developing an Ethereum project. They provide a safe environment for testing that closely mimics the main network - you don’t want to take out your project for a test drive in a network where mistakes will cost you and your users money!

#### Available testnets

There are a number of test networks available for you to choose, each with their own characteristics. The recommended network for testing decentralized applications and smart contracts is Sepolia. (id=11155111)

|      | Each network is identified by a numeric ID. Local networks usually have a large random value, while id=1 is reserved for the main Ethereum network. |
| ---- | ------------------------------------------------------------ |
|      |                                                              |

#### Connecting a project to a public network

While you can spin up your own [Ethereum nodes](https://ethereum.org/en/developers/docs/nodes-and-clients/run-a-node/) connected to a testnet, the easiest way to access a testnet is via a public node service such as [Alchemy](https://alchemy.com/) or [Infura](https://infura.io/). Alchemy and Infura provide access to public nodes for all testnets and the main network, via both free and paid plans.

|      | We say a node is *public* when it can be accessed by the general public, and manages no accounts. This means that it can reply to queries and relay signed transactions, but cannot sign transactions on its own. |
| ---- | ------------------------------------------------------------ |
|      |                                                              |

In this guide we will use Alchemy, though you can use [Infura](https://infura.io/), or another public node provider of your choice.

Head over to [Alchemy](https://dashboard.alchemyapi.io/signup?referral=53fcee38-b894-4d5f-bd65-885d241f8d29) (includes referral code), sign up, and jot down your assigned API key - we will use it later to connect to the network.

#### Creating a new account

To send transactions in a testnet, you will need a new Ethereum account. There are many ways to do this: here we will use the `mnemonics` package, which will output a fresh mnemonic (a set of 12 words) we will use to derive our accounts:

``` bash
npx mnemonics
drama film snack motion ...
```

Make sure to keep your mnemonic secure. Do not commit secrets to version control. Even if it is just for testing purposes, there are still malicious users out there who will wreak havoc on your testnet deployment for fun!

#### Configuring the network

Since we are using public nodes, we will need to sign all our transactions locally. We will configure the network with our mnemonic and an Alchemy endpoint.

```
This part assumes you have already set up a project. If you haven’t, head over to the guide on Setting up a Solidity project.
```

We need to update our configuration file with a new network connection to the testnet. Here we will use Sepolia, but you can use whichever you want:

```
// hardhat.config.js
+ const { alchemyApiKey, mnemonic } = require('./secrets.json');
...
  module.exports = {
+    networks: {
+     sepolia: {
+       url: `https://eth-sepolia.g.alchemy.com/v2/${alchemyApiKey}`,
+       accounts: { mnemonic: mnemonic },
+     },
+   },
...
};
```

Note in the first line that we are loading the project id and mnemonic from a `secrets.json` file, which should look like the following, but using your own values. Make sure to `.gitignore` it to ensure you don’t commit secrets to version control!

```json
{
  "mnemonic": "drama film snack motion ...",
  "alchemyApiKey": "JPV2..."
}
```

```

Instead of a secrets.json file, you can use whatever secret-management solution you like for your project. A popular and simple option is to use dotenv for injecting secrets as environment variables.
```

We can now test out that this configuration is working by listing the accounts we have available for the Sepolia network. Remember that yours will be different, as they depend on the mnemonic you used.

```bash
$ npx hardhat console --network sepolia
Welcome to Node.js v20.17.0.
Type ".help" for more information.
> accounts = (await ethers.getSigners()).map(signer => signer.address)
[
  '0x6B1c3A2f2160a7Cb2ebc7Fc861b8dB71476C30E7',
  '0xC1310ade58A75E6d4fCb8238f9559188Ea3808f9',
...
]
```

We can also test the connection to the node, by querying our account balance.

```bash
> (await ethers.provider.getBalance(accounts[0])).toString()
'0'
```

Empty! This points to our next task: getting testnet funds so that we can send transactions.

#### Funding the testnet account

Most public testnets have a faucet: a site that will provide you with a small amount of test Ether for free. If you are on Sepolia, head to [Alchemy’s free Sepolia faucet](https://www.alchemy.com/faucets/ethereum-sepolia), [Infura’s free Sepolia faucet](https://www.infura.io/faucet), or [Google’s free Sepolia faucet](https://cloud.google.com/application/web3/faucet/ethereum/sepolia) to get free testETH.

Armed with a funded account, let’s deploy our contracts to the testnet!

#### Working on a testnet

With a project configured to work on a public testnet, we can now finally [deploy our `Box` contract](https://docs.openzeppelin.com/learn/deploying-and-interacting#deploying-a-smart-contract). The command here is exactly the same as if you were on your [local development network](https://docs.openzeppelin.com/learn/deploying-and-interacting#local-blockchain), though it will take a few seconds to run as new blocks are mined.

```bash
npx hardhat run --network sepolia scripts/deploy.js
Deploying Box...
Box deployed to: 0x1b99CCaCea0e4046db618770dEF72180F8138641
```

That’s it! Your `Box` contract instance will be forever stored in the testnet, and publicly accessible to anyone.

You can see your contract on a block explorer such as [Etherscan](https://etherscan.io/). Remember to access the explorer on the testnet where you deployed your contract, such as [sepolia.etherscan.io](https://sepolia.etherscan.io/) for Sepolia.

You can also interact with your instance as you regularly would, either using the [console](https://docs.openzeppelin.com/learn/deploying-and-interacting#interacting-from-the-console), or [programmatically](https://docs.openzeppelin.com/learn/deploying-and-interacting#interacting-programatically).

```bash
npx hardhat console --network sepolia
Welcome to Node.js v20.17.0.
Type ".help" for more information.
> const Box = await ethers.getContractFactory('Box');
undefined
> const box = await Box.attach('0x1b99CCaCea0e4046db618770dEF72180F8138641');
undefined
> await box.store(42);
{
  hash: '0x330e331d30ee83f96552d82b7fdfa6156f9f97d549a612eeef7283d18b31d107',
...
> (await box.retrieve()).toString()
'42'
```

Keep in mind that every transaction will cost some gas, so you will eventually need to top up your account with more funds.

### Upgrading smart contracts

Smart contracts deployed using [OpenZeppelin Upgrades Plugins](https://docs.openzeppelin.com/upgrades-plugins/) can be **upgraded** to modify their code, while preserving their address, state, and balance. This allows you to iteratively add new features to your project, or fix any bugs you may find [in production](https://docs.openzeppelin.com/learn/preparing-for-mainnet).

Throughout this guide, we will learn:

- [Why upgrades are important](https://docs.openzeppelin.com/learn/upgrading-smart-contracts#whats-in-an-upgrade)
- [Upgrade our Box using the Upgrades Plugins](https://docs.openzeppelin.com/learn/upgrading-smart-contracts#upgrading-a-contract-via-plugins)
- [Learn how upgrades work under the hood](https://docs.openzeppelin.com/learn/upgrading-smart-contracts#how-upgrades-work)
- [Learn how to write upgradeable contracts](https://docs.openzeppelin.com/learn/upgrading-smart-contracts#limitations-of-contract-upgrades)

#### What’s in an upgrade

Smart contracts in Ethereum are immutable by default. Once you create them there is no way to alter them, effectively acting as an unbreakable contract among participants.

以太坊中的智能合约默认是不可变的。一旦创建，就无法更改，相当于参与者之间不可破坏的合约。

However, for some scenarios, it is desirable to be able to modify them. Think of a traditional contract between two parties: if they both agreed to change it, they would be able to do so. On Ethereum, they may desire to alter a smart contract to fix a bug they found (which might even lead to a hacker stealing their funds!), to add additional features, or simply to change the rules enforced by it.然而，在某些情况下，能够修改合约是可取的。想象一下双方之间的传统合约：如果他们双方都同意修改合约，他们就可以这么做。在以太坊上，他们可能希望修改智能合约来修复他们发现的漏洞（这甚至可能导致黑客窃取他们的资金！）、添加额外的功能，或者仅仅改变合约强制执行的规则。

Here’s what you’d need to do to fix a bug in a contract you cannot upgrade:

要修复无法升级的合约中的错误，您需要执行以下操作：

1. Deploy a new version of the contract
2. Manually migrate all state from the old one contract to the new one (which can be very expensive in terms of gas fees!)
3. Update all contracts that interacted with the old contract to use the address of the new one
4. Reach out to all your users and convince them to start using the new deployment (and handle both contracts being used simultaneously, as users are slow to migrate)

1. 部署新版本的合约
2. 手动将所有状态从旧合约迁移到新合约（这可能会产生非常昂贵的 Gas 费用！）
3. 更新所有与旧合约交互的合约，使其使用新合约的地址
4. 联系所有用户，说服他们开始使用新部署（并处理两个合约同时使用的情况，因为用户迁移速度较慢）

To avoid going through this mess, we have built contract upgrades directly into our plugins. This allows us to **change the contract code, while preserving the state, balance, and address**. Let’s see it in action.

为了避免这种麻烦，我们在插件中直接内置了合约升级功能。这样我们就可以**更改合约代码，同时保留状态、余额和地址**。让我们来看看具体操作。

#### Upgrading using the Upgrades Plugins

Whenever you deploy a new contract using `deployProxy` in the [OpenZeppelin Upgrades Plugins](https://docs.openzeppelin.com/upgrades-plugins/), that contract instance can be **upgraded** later. By default, only the address that originally deployed the contract has the rights to upgrade it.

`deployProxy` will create the following transactions:

1. Deploy the implementation contract (our `Box` contract)
2. Deploy the proxy contract and run any initializer function.
   - The proxy deployment automatically deploys a `ProxyAdmin` contract (the admin for our proxy) in the scenario below.

Let’s see how it works, by deploying an upgradeable version of our `Box` contract, using the same setup as when [we deployed earlier](https://docs.openzeppelin.com/learn/deploying-and-interacting#deploying-a-smart-contract):

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

We first need to install the Upgrades Plugin.

Install the [Hardhat Upgrades](https://docs.openzeppelin.com/upgrades-plugins/hardhat-upgrades) plugin.

```bash
npm install --save-dev @openzeppelin/hardhat-upgrades
```

We then need to configure Hardhat to use our `@openzeppelin/hardhat-upgrades` plugin. To do this add the plugin in your `hardhat.config.js` file as follows.

然后，我们需要配置 Hardhat 以使用我们的 @openzeppelin/hardhat-upgrades 插件。具体操作如下：在 hardhat.config.js 文件中添加该插件。

In order to upgrade a contract like `Box` we need to first deploy it as an upgradeable contract, which is a different deployment procedure than we’ve seen so far. We will initialize our Box contract by calling `store` with the value 42.

With Hardhat, we use [scripts](https://hardhat.org/hardhat-runner/docs/advanced/scripts#writing-scripts-with-hardhat) to deploy upgradeable contracts.

We will create a script to deploy our upgradeable Box contract using [`deployProxy`](https://docs.openzeppelin.com/upgrades-plugins/api-hardhat-upgrades#deploy-proxy). We will save this file as `scripts/deploy_upgradeable_box.js`.

```solidity
// scripts/deploy_upgradeable_box.js
const { ethers, upgrades } = require('hardhat');

async function main () {
  const Box = await ethers.getContractFactory('Box');
  console.log('Deploying Box...');
  const box = await upgrades.deployProxy(Box, [42], { initializer: 'store' });
  await box.waitForDeployment();
  console.log('Box deployed to:', await box.getAddress());
}

main();
```

We can then deploy our upgradeable contract.

Using the `run` command, we can deploy the `Box` contract to the `development` network.

```bash
npx hardhat run --network localhost scripts/deploy_upgradeable_box.js
Deploying Box...
Box deployed to: 0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0
```

We can then interact with our `Box` contract to `retrieve` the value that we stored during initialization.

We will use the [Hardhat console](https://hardhat.org/guides/hardhat-console.html) to interact with our upgraded `Box` contract.

We need to specify the address of our proxy contract from when we deployed our `Box` contract.

```bash
$ npx hardhat console --network localhost
Welcome to Node.js v20.17.0.
Type ".help" for more information.
> const Box = await ethers.getContractFactory('Box');
undefined
> const box = await Box.attach('0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0');
undefined
> (await box.retrieve()).toString();
'42'
```

For the sake of the example, let’s say we want to add a new feature: a function that increments the `value` stored in a new version of `Box`.

```solidity
// contracts/BoxV2.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract BoxV2 {
    // ... code from Box.sol

    // Increments the stored value by 1
    function increment() public {
        _value = _value + 1;
        emit ValueChanged(_value);
    }
}
```

After creating the Solidity file, we can now upgrade the instance we had deployed earlier using the `upgradeProxy` function.

`upgradeProxy` will create the following transactions:

1. Deploy the implementation contract (our `BoxV2` contract)
2. Call the `ProxyAdmin` to update the proxy contract to use the new implementation.

We will create a script to upgrade our `Box` contract to use `BoxV2` using [`upgradeProxy`](https://docs.openzeppelin.com/upgrades-plugins/api-hardhat-upgrades#upgrade-proxy). We will save this file as `scripts/upgrade_box.js`. We need to specify the address of our proxy contract from when we deployed our `Box` contract.

创建 Solidity 文件后，我们现在可以使用 UpgradeProxy 函数升级之前部署的实例。

UpgradeProxy 将创建以下交易：

部署实现合约（我们的 BoxV2 合约）

调用 ProxyAdmin 函数更新代理合约以使用新的实现。

我们将创建一个脚本，使用 UpgradeProxy 升级 Box 合约以使用 BoxV2。我们将此文件保存为 scripts/upgrade_box.js。我们需要指定部署 Box 合约时代理合约的地址。

```javascript
// scripts/upgrade_box.js
const { ethers, upgrades } = require('hardhat');

async function main () {
  const BoxV2 = await ethers.getContractFactory('BoxV2');
  console.log('Upgrading Box...');
  await upgrades.upgradeProxy('0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0', BoxV2);
  console.log('Box upgraded');
}

main();
```

We can then deploy our upgradeable contract.

Using the `run` command, we can upgrade the `Box` contract on the `development` network.

```bash
npx hardhat run --network localhost scripts/upgrade_box.js
Compiled 1 Solidity file successfully (evm target: paris).
Upgrading Box...
Box upgraded
```

Done! Our `Box` instance has been upgraded to the latest version of the code, **while keeping its state and the same address as before**. We didn’t need to deploy a new one at a new address, nor manually copy the `value` from the old `Box` to the new one.

Let’s try it out by invoking the new `increment` function, and checking the `value` afterwards:

We need to specify the address of our proxy contract from when we deployed our `Box` contract.

```bash
npx hardhat console --network localhost
Welcome to Node.js v20.17.0.
Type ".help" for more information.
> const BoxV2 = await ethers.getContractFactory('BoxV2');
undefined
> const box = await BoxV2.attach('0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0');
undefined
> await box.increment();
...
> (await box.retrieve()).toString();
'43'
```

That’s it! Notice how the `value` of the `Box` was preserved throughout the upgrade, as well as its address. And this process is the same regardless of whether you are working on a local blockchain, a testnet, or the main network.

Let’s see how the [OpenZeppelin Upgrades Plugins](https://docs.openzeppelin.com/upgrades-plugins/) accomplish this.

#### How upgrades work

*This section will be more theory-heavy than others: feel free to skip over it and return later if you are curious.*

When you create a new upgradeable contract instance, the [OpenZeppelin Upgrades Plugins](https://docs.openzeppelin.com/upgrades-plugins/) actually deploys three contracts:

1. The contract you have written, which is known as the *implementation contract* containing the *logic*.
2. A *proxy* to the *implementation contract*, which is the contract that you actually interact with.
3. A *ProxyAdmin* to be the admin of the *proxy*.

当你创建一个新的可升级合约实例时，[OpenZeppelin 升级插件](https://docs.openzeppelin.com/upgrades-plugins/) 实际上会部署三个合约：

1. 你编写的合约，即包含*逻辑*的*实现合约*。
2. 一个指向*实现合约*的*代理*，也就是你实际与之交互的合约。
3. 一个*ProxyAdmin*，作为*代理*的管理员。

Here, the *proxy* is a simple contract that just *delegates* all calls to an implementation contract. A *delegate call* is similar to a regular call, except that all code is executed in the context of the caller, not of the callee. Because of this, a `transfer` in the implementation contract’s code will actually transfer the proxy’s balance, and any reads or writes to the contract storage will read or write from the proxy’s own storage.

这里，*代理*是一个简单的合约，它只是将所有调用*委托*给一个实现合约。*委托调用*类似于常规调用，不同之处在于所有代码都在调用方上下文中执行，而不是在被调用方上下文中执行。因此，实现合约代码中的“转账”实际上会转移代理的余额，而对合约存储的任何读写操作都将从代理自身的存储中进行读写。

This allows us to **decouple** a contract’s state and code: the proxy holds the state, while the implementation contract provides the code. And it also allows us to **change** the code by just having the proxy delegate to a different implementation contract.

这使我们能够**解耦**合约的状态和代码：代理保存状态，而实现合约提供代码。并且，它还允许我们只需将代理委托给不同的实现合约即可**更改**代码。

An upgrade then involves the following steps:

1. Deploy the new implementation contract.
2. Send a transaction to the proxy that updates its implementation address to the new one.

升级包含以下步骤：

1. 部署新的实现合约。
2. 向代理发送交易，将其实现地址更新为新的地址。

```note
You can have multiple proxies using the same implementation contract, so you can save gas using this pattern if you plan to deploy multiple copies of the same contract.
```

Any user of the smart contract always interacts with the proxy, **which never changes its address**. This allows you to roll out an upgrade or fix a bug without requesting your users to change anything on their end - they just keep interacting with the same address as always.

```

If you want to learn more about how OpenZeppelin proxies work, check out Proxies.

```

#### Limitations of contract upgrades 

可升级合约的限制

While any smart contract can be made upgradeable, some restrictions of the Solidity language need to be worked around. These come up when writing both the initial version of contract and the version we’ll upgrade it to.

虽然任何智能合约都可以升级，但需要解决 Solidity 语言的一些限制。这些限制在编写合约的初始版本和升级到的版本时都会出现。

##### Initialization

Upgradeable contracts cannot have a `constructor`. To help you run initialization code, [**OpenZeppelin Contracts**](https://docs.openzeppelin.com/contracts/5.x/) provides the [`Initializable`](https://docs.openzeppelin.com/contracts/5.x/api/proxy#Initializable) base contract that allows you to tag a method as [`initializer`](https://docs.openzeppelin.com/contracts/5.x/api/proxy#Initializable-initializer--), ensuring it can be run only once.

As an example, let’s write a new version of the `Box` contract with an initializer, storing the address of an `admin` who will be the only one allowed to change its contents.

```solidity
// contracts/AdminBox.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

contract AdminBox is Initializable {
    uint256 private _value;
    address private _admin;

    // Emitted when the stored value changes
    event ValueChanged(uint256 value);

    function initialize(address admin) public initializer {
        _admin = admin;
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() initializer {}

    // Stores a new value in the contract
    function store(uint256 value) public {
        require(msg.sender == _admin, "AdminBox: not admin");
        _value = value;
        emit ValueChanged(value);
    }

    // Reads the last stored value
    function retrieve() public view returns (uint256) {
        return _value;
    }
}
```









































