# solana-dex-with-sol-events
Stream Solana Dex events and SOL Transfer events with [substreams](https://substreams.streamingfast.io).


## Attension 

To meet my new Solana data requirements, I have expanded it and added some features. It now includes Sol transfer events and pumpfun events.

**This is due to my specific needs and it is not universal. Please be cautious when using it.**

## Usage

- First step into the project raydium_amm folder
```bash
cd raydium_amm
```

- Second step, run the substreams(After You install substreams scripts and config the system env)

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
    
For more information, refer to the [protobuf specification](raydium_amm/proto/raydium_amm.proto).
