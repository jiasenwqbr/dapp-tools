## install rust

The Rust programming language is a multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency.

Using [rustup](https://rustup.rs/), the official Rust version installer and manager, we will install `rustc` (the compiler for rust) and `cargo` (the package manager for rust) all at once.

### Install Rust for macOS, Linux, WSL or another Unix-like OS 

Using the following command, we can install and configure the Rust tooling on your local system. The following command will automatically download the correct binaries needed for your specific operating system:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

As part of this Rust installer, Rustup will also configure your terminal's `PATH` to include the rust toolchain.

After the installation is complete, restart your terminal or run the following command to manually refresh your new `PATH` settings to make the rust tooling (like `cargo`) available:

```bash
source ~/.bashrc
```



## Install the Solana CLI

For local development, including compiling your Solana programs, you will need to [install the Solana CLI](https://docs.solanalabs.com/cli/install). This command-line tool suite provides all the commands needed to perform common tasks, like:

- creating and managing file-system Solana wallets/keypairs,
- connecting to Solana [clusters](https://solana.com/docs/core/clusters),
- building Solana programs,
- and deploying your programs to the blockchain

### For Linux, macOS, WSL or other Unix-like systems:

Install the Solana CLI tool suite using the official install command:

```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

You can replace `stable` with the release tag matching the software version of your desired release (i.e. `v1.18.1`), or use one of the three symbolic channel names: `stable`, `beta`, or `edge`.

```bash
Please update your PATH environment variable to include the Solana programs:
```

If you get the above message, simply copy and paste the command recommended by the Solana CLI installer to update your `PATH` environment variable.

After running this command. restart your terminal to make sure your Solana binaries are accessible in all the terminal sessions you open afterwards.

To check if your installation was successful, check the Solana CLI version:

```bash
solana --version
```

You can see more versions and releases according to the target [solana/releases](https://github.com/solana-labs/solana/releases)

UPDATING THE SOLANA CLI

In the future, you can use the Solana CLI to update itself based on which latest version is available: `solana-install update`









## install spl-token-cli

```bash
cargo install spl-token-cli
```



```bash
spl-token create-token

```



```bash
spl-token create-account <TOKEN_MINT_ADDRESS>
```



My token balance 

```bash
spl-token balance <my token address>
```

Mint

```bash
spl-token mint <TOKEN_MINT_ADDRESS> 1000
```



```bash
spl-token supply <TOKEN_MINT_ADDRESS>
```





```bash
spl-token authorize <TOKEN_MINT_ADDRESS> mint --disable 
```



```bash
spl-token burn <my token account> 1000
```











