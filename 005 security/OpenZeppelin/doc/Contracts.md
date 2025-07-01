## Contracts

**A library for secure smart contract development.** Build on a solid foundation of community-vetted code.

- Implementations of standards like [ERC20](https://docs.openzeppelin.com/contracts/5.x/erc20) and [ERC721](https://docs.openzeppelin.com/contracts/5.x/erc721).
- Flexible [role-based permissioning](https://docs.openzeppelin.com/contracts/5.x/access-control) scheme.
- Reusable [Solidity components](https://docs.openzeppelin.com/contracts/5.x/utilities) to build custom contracts and complex decentralized systems.

```
OpenZeppelin Contracts uses semantic versioning to communicate backwards compatibility of its API and storage layout. For upgradeable contracts, the storage layout of different major versions should be assumed incompatible, for example, it is unsafe to upgrade from 4.9.3 to 5.0.0. Learn more at Backwards Compatibility.
```

Hardhat (npm)

```
$ npm install @openzeppelin/contracts
```

Foundry (git)

```bash
forge install OpenZeppelin/openzeppelin-contracts
```

Usage

Once installed.you can use the contracts in the library by importing them:

```solidity
// contracts/MyNFT.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract MyNFT is ERC721 {
    constructor() ERC721("MyNFT", "MNFT") {}
}
```



### Contract Wizard

https://docs.openzeppelin.com/contracts/5.x/wizard

#### Extending Contracts

Most of the OpenZeppelin Contracts are expected to be used via [inheritance](https://solidity.readthedocs.io/en/latest/contracts.html#inheritance): you will *inherit* from them when writing your own contracts.

This is the commonly found `is` syntax, like in `contract MyToken is ERC20`.

```

Unlike contracts, Solidity librarys are not inherited from and instead rely on the using for syntax.

OpenZeppelin Contracts has some librarys: most are in the Utils directory.
```

##### Overriding

Inheritance is often used to add the parent contract’s functionality to your own contract, but that’s not all it can do. You can also *change* how some parts of the parent behave using *overrides*.

For example, imagine you want to change [`AccessControl`](https://docs.openzeppelin.com/contracts/5.x/api/access#AccessControl) so that [`revokeRole`](https://docs.openzeppelin.com/contracts/5.x/api/access#AccessControl-revokeRole-bytes32-address-) can no longer be called. This can be achieved using overrides:

```solidity
// contracts/AccessControlModified.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

contract AccessControlModified is AccessControl {
    error AccessControlNonRevocable();

    // Override the revokeRole function
    function revokeRole(bytes32, address) public pure override {
        revert AccessControlNonRevocable();
    }
}
```

The old `revokeRole` is then replaced by our override, and any calls to it will immediately revert. We cannot *remove* the function from the contract, but reverting on all calls is good enough.

##### Calling `super`

Sometimes you want to *extend* a parent’s behavior, instead of outright changing it to something else. This is where `super` comes in.

The `super` keyword will let you call functions defined in a parent contract, even if they are overridden. This mechanism can be used to add additional checks to a function, emit events, or otherwise add functionality as you see fit.

```
For more information on how overrides work, head over to the official Solidity documentation.
```

Here is a modified version of [`AccessControl`](https://docs.openzeppelin.com/contracts/5.x/api/access#AccessControl) where [`revokeRole`](https://docs.openzeppelin.com/contracts/5.x/api/access#AccessControl-revokeRole-bytes32-address-) cannot be used to revoke the `DEFAULT_ADMIN_ROLE`:

```solidity
// contracts/AccessControlNonRevokableAdmin.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

contract AccessControlNonRevokableAdmin is AccessControl {
    error AccessControlNonRevokable();

    function revokeRole(bytes32 role, address account) public override {
        if (role == DEFAULT_ADMIN_ROLE) {
            revert AccessControlNonRevokable();
        }

        super.revokeRole(role, account);
    }
}
```

The `super.revokeRole` statement at the end will invoke `AccessControl`'s original version of `revokeRole`, the same code that would’ve run if there were no overrides in place.

```
The same rule is implemented and extended in AccessControlDefaultAdminRules, an extension that also adds enforced security measures for the DEFAULT_ADMIN_ROLE.
```

##### Security

The maintainers of OpenZeppelin Contracts are mainly concerned with the correctness and security of the code as published in the library, and the combinations of base contracts with the official extensions from the library.

Custom overrides, especially to hooks, can disrupt important assumptions and may introduce security risks in the code that was previously secure. While we try to ensure the contracts remain secure in the face of a wide range of potential customizations, this is done in a best-effort manner. While we try to document all important assumptions, this should not be relied upon. Custom overrides should be carefully reviewed and checked against the source code of the contract they are customizing to fully understand their impact and guarantee their security.

The way functions interact internally should not be assumed to stay stable across releases of the library. For example, a function that is used in one context in a particular release may not be used in the same context in the next release. Contracts that override functions should revalidate their assumptions when updating the version of OpenZeppelin Contracts they are built on.

OpenZeppelin 合约的维护者主要关注库中发布的代码的正确性和安全性，以及基础合约与库中官方扩展的组合。

自定义覆盖（尤其是对钩子的覆盖）可能会破坏重要的假设，并可能给先前安全的代码带来安全风险。虽然我们尽力确保合约在面对各种潜在定制时仍然安全，但这已是尽最大努力。虽然我们尽力记录所有重要的假设，但不应依赖这些假设。应仔细审查自定义覆盖，并根据其所定制合约的源代码进行检查，以充分了解其影响并确保其安全性。

不应假设函数内部交互的方式在库的不同版本之间保持稳定。例如，在特定版本中某个上下文中使用的函数可能不会在下一个版本中在相同的上下文中使用。覆盖函数的合约应在更新其所基于的 OpenZeppelin 合约版本时重新验证其假设。



