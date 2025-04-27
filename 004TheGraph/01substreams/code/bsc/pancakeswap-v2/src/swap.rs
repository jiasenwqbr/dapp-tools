use prost::Message;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::pb::eth::v2::Log;

use crate::pb::pancake;

const SWAP_TOPIC: &str = "0xd78ad95fa46c994b6551d0da85fc275fe6131c3d"; // keccak256("Swap(address,uint256,uint256,uint256,uint256,address)")

#[substreams::handlers::map]
pub fn map_swaps(block: eth::Block) -> Result<pancake::Swaps, substreams::errors::Error> {
    let mut swaps: Vec<pancake::Swap> = vec![];

    for trx in block.transaction_traces {
        for log in trx.receipt.unwrap().logs {
            if log.topics.len() != 3 {
                continue;
            }

            if hex::encode(&log.topics[0]) != SWAP_TOPIC.trim_start_matches("0x") {
                continue;
            }

            // 解析出事件
            let swap = parse_swap_event(&log, &trx.hash)?;
            swaps.push(swap);
        }
    }

    Ok(pancake::Swaps { swaps })
}

fn parse_swap_event(log: &Log, trx_hash: &Vec<u8>) -> Result<pancake::Swap, substreams::errors::Error> {
    use substreams_ethereum::pb::eth::v2::BigInt;

    let amount0_in = BigInt::decode(&log.data[0..32])?;
    let amount1_in = BigInt::decode(&log.data[32..64])?;
    let amount0_out: BigInt = BigInt::decode(&log.data[64..96])?;
    let amount1_out: BigInt = BigInt::decode(&log.data[96..128])?;
    Ok(pancake::Swap {
        sender: hex::encode(&log.topics[1][12..]),
        to: hex::encode(&log.topics[2][12..]),
        pair_address: hex::encode(&log.address),
        token0_address: "".to_string(), // token0 需要通过额外查询pair合约拿，这里先留空
        token1_address: "".to_string(),
        trx_hash: hex::encode(trx_hash),
        log_ordinal: log.ordinal,
        amount0_in:hex::encode( amount0_in.bytes),
        amount1_in:hex::encode( amount1_in.bytes),
        amount0_out:hex::encode(amount0_out.bytes),
        amount1_out:hex::encode(amount1_out.bytes),
    })
}


