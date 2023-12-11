



## Foundry

**Foundry is a blazing fast, portable and modular toolkit for Ethereum application development written in Rust.**

Foundry consists of:

-   **Forge**: Ethereum testing framework (like Truffle, Hardhat and DappTools).
-   **Cast**: Swiss army knife for interacting with EVM smart contracts, sending transactions and getting chain data.
-   **Anvil**: Local Ethereum node, akin to Ganache, Hardhat Network.
-   **Chisel**: Fast, utilitarian, and verbose solidity REPL.

## Documentation

https://book.getfoundry.sh/

## Usage

### Build

```shell
$ forge build
```

### Test

```shell
$ forge test
```

### Format

```shell
$ forge fmt
```

### Gas Snapshots

```shell
$ forge snapshot
```

### Anvil

```shell
$ anvil
```

### Deploy

```shell
$ forge script script/Counter.s.sol:CounterScript --rpc-url <your_rpc_url> --private-key <your_private_key>
```

### Cast

```shell
$ cast <subcommand>
```

### Help

```shell
$ forge --help
$ anvil --help
$ cast --help
```

## create project

```shell
forge init 019-uniswapv2-p1  --no-commit --force
```

```shell
cd 019-uniswapv2-p1
```

## forge build

```shell
forge build
```

```shell
[⠊] Compiling...
[⠊] Installing Solc version 0.8.23
[⠆] Successfully installed Solc 0.8.23
[⠰] Compiling 24 files with 0.8.23
[⠑] Solc 0.8.23 finished in 5.11s
Compiler run successful!

```

## forge test

```shell
forge test

```

```shell
[⠆] Compiling...
No files changed, compilation skipped

Running 2 tests for test/Counter.t.sol:CounterTest
[PASS] testFuzz_SetNumber(uint256) (runs: 256, μ: 27009, ~: 28409)
[PASS] test_Increment() (gas: 28379)
Test result: ok. 2 passed; 0 failed; 0 skipped; finished in 22.97ms
 
Ran 1 test suites: 2 tests passed, 0 failed, 0 skipped (2 total tests)
```

```shell
forge install OpenZeppelin/openzeppelin-contracts --no-commit 
```

