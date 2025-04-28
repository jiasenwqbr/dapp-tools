
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::pb::eth::v2::Log;
use substreams::log;
use crate::pb::pancake;

const SWAP_TOPIC: &str = "0xcA143Ce32Fe78f1f7019d7d551a6402fC5350c73"; // keccak256("Swap(address,uint256,uint256,uint256,uint256,address)")

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
            log::info!("&log.topics[0]:{}",hex::encode(&log.topics[0])); 
            // 解析出事件
            let swap = parse_swap_event(&log, &trx.hash)?;
            swaps.push(swap);
        }
    }

    Ok(pancake::Swaps { swaps })
}

fn parse_swap_event(log: &Log, trx_hash: &Vec<u8>) -> Result<pancake::Swap, substreams::errors::Error> {
    
    Ok(pancake::Swap {
        sender: hex::encode(&log.topics[1][12..]),
        to: hex::encode(&log.topics[2][12..]),
        pair_address: hex::encode(&log.address),
        token0_address: "".to_string(), // token0 需要通过额外查询pair合约拿，这里先留空
        token1_address: "".to_string(),
        trx_hash: hex::encode(trx_hash),
        log_ordinal: log.ordinal,
        amount0_in:hex::encode( &log.data[0..32]),
        amount1_in:hex::encode( &log.data[32..64]),
        amount0_out:hex::encode(&log.data[64..96]),
        amount1_out:hex::encode(&log.data[96..128]),
    })
}


