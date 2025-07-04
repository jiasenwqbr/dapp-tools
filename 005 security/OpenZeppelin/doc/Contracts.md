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

### Backwards Compatibility 向后兼容性

OpenZeppelin Contracts uses semantic versioning to communicate backwards compatibility of its API and storage layout. Patch and minor updates will generally be backwards compatible, with rare exceptions as detailed below. Major updates should be assumed incompatible with previous releases. On this page, we provide details about these guarantees.

OpenZeppelin Contracts 使用语义版本控制来传达其 API 和存储布局的向后兼容性。补丁和次要更新通常向后兼容，但下文所述的少数例外情况除外。主要更新应假定与先前版本不兼容。在本页面，我们提供有关这些保证的详细信息。

#### API

In backwards compatible releases, all changes should be either additions or modifications to internal implementation details. Most code should continue to compile and behave as expected. The exceptions to this rule are listed below.

在向后兼容的版本中，所有更改都应为对内部实现细节的添加或修改。大多数代码应继续按预期进行编译和运行。此规则的例外情况如下所列。

#### Security

Infrequently a patch or minor update will remove or change an API in a breaking way, but only if the previous API is considered insecure. These breaking changes will be noted in the changelog and release notes, and published along with a security advisory.

补丁或小更新偶尔会移除或更改 API，但前提是之前的 API 被认为不安全。这些重大更改将在变更日志和发行说明中注明，并随安全公告一起发布。

#### Draft or Pre-Final ERCs 草案或预最终版 ERC

ERCs that are not Final can change in incompatible ways. For this reason, we avoid shipping implementations of ERCs before they are Final. Some exceptions are made for ERCs that have been published for a long time and that seem unlikely to change. Breaking changes to the ERC specification are still technically possible in those cases, so these implementations are published in files named `draft-*.sol` to make that condition explicit.

非最终版 ERC 可能会以不兼容的方式进行更改。因此，我们避免在 ERC 最终版之前发布其实现。对于已发布很长时间且不太可能更改的 ERC，我们会做出一些例外。在这种情况下，从技术上讲，对 ERC 规范进行重大更改仍然是可能的，因此这些实现会发布在名为 `draft-*.sol` 的文件中，以明确说明这种情况。

#### Virtual & Overrides

Almost all functions in this library are virtual with some exceptions, but this does not mean that overrides are encouraged. There is a subset of functions that are designed to be overridden. By defining overrides outside of this subset you are potentially relying on internal implementation details. We make efforts to preserve backwards compatibility even in these cases but it is extremely difficult and easy to accidentally break. Caution is advised.

Additionally, some minor updates may result in new compilation errors of the kind "two or more base classes define function with same name and parameter types" or "need to specify overridden contract", due to what Solidity considers ambiguity in inherited functions. This should be resolved by adding an override that invokes the function via `super`.

See [Extending Contracts](https://docs.openzeppelin.com/contracts/5.x/extending-contracts) for more about virtual and overrides.

此库中几乎所有函数都是虚函数，但也有一些例外，但这并不意味着鼓励重写。有一些函数子集被设计为可重写。通过定义此子集之外的重写，您可能会依赖内部实现细节。即使在这些情况下，我们也努力保持向后兼容性，但意外中断的可能性极小，而且很容易发生。建议您谨慎操作。

此外，由于 Solidity 认为继承函数存在歧义，一些小更新可能会导致新的编译错误，例如“两个或多个基类定义了具有相同名称和参数类型的函数”或“需要指定重写的合约”。这可以通过添加通过“super”调用该函数的重写来解决。

有关虚函数和重写的更多信息，请参阅[扩展合约](https://docs.openzeppelin.com/contracts/5.x/extending-contracts)。

#### Structs

Struct members with an underscore prefix should be considered "private" and may break in minor versions. Struct data should only be accessed and modified through library functions.

带有下划线前缀的结构体成员应被视为“私有”，并且可能会在小版本中失效。结构体数据只能通过库函数访问和修改。

#### Errors

The specific error format and data that is included with reverts should not be assumed stable unless otherwise specified.

#### Major Releases

Major releases should be assumed incompatible. Nevertheless, the external interfaces of contracts will remain compatible if they are standardized, or if the maintainers judge that changing them would cause significant strain on the ecosystem.

An important aspect that major releases may break is "upgrade compatibility", in particular storage layout compatibility. It will never be safe for a live contract to upgrade from one major release to another

#### Storage Layout

Minor and patch updates always preserve storage layout compatibility. This means that a live contract can be upgraded from one minor to another without corrupting the storage layout. In some cases it may be necessary to initialize new state variables when upgrading, although we expect this to be infrequent.

We recommend using [OpenZeppelin Upgrades Plugins or CLI](https://docs.openzeppelin.com/upgrades-plugins/) to ensure storage layout safety of upgrades.

#### Solidity Version

The minimum Solidity version required to compile the contracts will remain unchanged in minor and patch updates. New contracts introduced in minor releases may make use of newer Solidity features and require a more recent version of the compiler.

### Access Control

Access control—that is, "who is allowed to do this thing"—is incredibly important in the world of smart contracts. The access control of your contract may govern who can mint tokens, vote on proposals, freeze transfers, and many other things. It is therefore **critical** to understand how you implement it, lest someone else [steals your whole system](https://blog.openzeppelin.com/on-the-parity-wallet-multisig-hack-405a8c12e8f7).

#### Ownership and `Ownable`

The most common and basic form of access control is the concept of *ownership*: there’s an account that is the `owner` of a contract and can do administrative tasks on it. This approach is perfectly reasonable for contracts that have a single administrative user.

OpenZeppelin Contracts provides [`Ownable`](https://docs.openzeppelin.com/contracts/5.x/api/access#Ownable) for implementing ownership in your contracts.

