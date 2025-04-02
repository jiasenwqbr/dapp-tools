# solana-dex-with-sol-events
Stream Solana Dex events and SOL Transfer events with [substreams](https://substreams.streamingfast.io).

## Usage

```bash
substreams run substreams.yaml raydium_amm_events -e mainnet.sol.streamingfast.io:443 -s {your_start_block_number} -t +1
```
If you see no output, please check that you have set a starting block, e.g. `substreams run substreams.yaml raydium_amm_events -e mainnet.sol.streamingfast.io:443 -s 325766951 -t +1`.

## Suported Events 

### Basic

- InitializeEvent: Raydium AMM initialization event
- DepositEvent：Raydium AMM deposit event
- WithdrawEvent:  Raydium AMM withdraw event
- WithdrawPnlEvent: Raydium AMM withdraw PNL event
- SwapEvent: Raydium AMM swap event

### Features

- TransferEvent: SOL Transfer event filter the lamports at least 100000, and filter the dex(Raydium\ Pumpfun \ Jupiter) 
- TransferWithSeedEvent: SOL Transfer with Seed event filter the lamports at least 100000, and filter the dex(Raydium\ Pumpfun \ Jupiter) 
- PumpfunSwapEvent: Pumpfun swap event（buy or sell）
- PumpfunWithdrawEvent: Pumpfun withdraw event
- PumpfunCreateEvent: Pumpfun create event
    
For more information, refer to the [protobuf specification](proto/raydium_amm.proto).

```shell
cargo clean

cargo build --target wasm32-unknown-unknown --release


RUST_LOG=debug substreams-sink-sql setup "psql://postgres:root@172.20.31.66:5432/blockchain_data?search_path=solana_raydium&schema=solana_raydium&sslmode=disable" ./sink/substreams.dev.yaml 

RUST_LOG=debug  substreams-sink-sql run "psql://postgres:root@172.20.31.66:5432/blockchain_data?search_path=solana_raydium&schema=solana_raydium&sslmode=disable" ./sink/substreams.dev.yaml   --header "x-api-key:"  --on-module-hash-mistmatch=ignore 

```
