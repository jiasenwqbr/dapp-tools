```bash
mkdir 032-hardhat-nft

cd 032-hardhat-nft

npm init

npm install hardhat

npx hardhat
```

```
npx hardhat
WARNING: You are currently using Node.js v23.10.0, which is not supported by Hardhat. This can lead to unexpected behavior. See https://hardhat.org/nodejs-versions


888    888                      888 888               888
888    888                      888 888               888
888    888                      888 888               888
8888888888  8888b.  888d888 .d88888 88888b.   8888b.  888888
888    888     "88b 888P"  d88" 888 888 "88b     "88b 888
888    888 .d888888 888    888  888 888  888 .d888888 888
888    888 888  888 888    Y88b 888 888  888 888  888 Y88b.
888    888 "Y888888 888     "Y88888 888  888 "Y888888  "Y888

👷 Welcome to Hardhat v2.24.3 👷‍

✔ What do you want to do? · Create an empty hardhat.config.js
✨ Config file created ✨

Give Hardhat a star on Github if you're enjoying it! ⭐️✨

     https://github.com/NomicFoundation/hardhat


DEPRECATION WARNING

 Initializing a project with npx hardhat is deprecated and will be removed in the future.
 Please use npx hardhat init instead.
```

```shell
npm install @openzeppelin/contracts

```

crate a contract in contracts folder

```solidity
// SPDX-License-Identifier: MIT

pragma solidity ^0.8.28;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract MyContract is ERC721 {

    constructor(string memory name, string memory symbol) 
        ERC721(name, symbol) {

    }

}
```

```shell
npx  hardhat compile
```

```shell
npm  install @nomiclabs/hardhat-waffle ethereum-waffle chai @nomiclabs/hardhat-ethers ethers

```

```bash
mkdir test
cd test
touch test.js
```
