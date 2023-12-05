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





## Install on mac

```shell
curl -L https://foundry.paradigm.xyz | bash
```

The error 

```shell

  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100  1942  100  1942    0     0   1132      0  0:00:01  0:00:01 --:--:-- 1896k
Installing foundryup...
######################################################################## 100.0%

warning: libusb not found. You may need to install it manually on MacOS via Homebrew (brew install libusb).

Detected your preferred shell is zsh and added foundryup to PATH. Run 'source /Users/a1234/.zshenv' or start a new terminal session to use foundryup.
Then, simply run 'foundryup' to install Foundry.

```

```shell
brew install libusb
```

```shell
foundryup
```



## create project

```shell
forge init 018-uniswapv1-foundry --no-commit --force
```

```shell
cd 018-uniswapv1-foundry 
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

