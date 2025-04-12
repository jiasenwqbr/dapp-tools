use substreams::log;
use std::collections::HashMap;
use substreams::{prelude::BigInt, Hex};
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams_ethereum::{
    pb::eth::v2::{self as eth, Log},
    Event,
};

use crate::abi::pool::events::Swap;

// #[derive(Debug, Deserialize)]
// struct Params {
//     factory_address: String,
//     protocol_type_name: String,
// }

#[derive(Debug)]
struct SwapV1 {
    id: String,
    pair_address:String,
    sender: String,
    to: String,
    amount0_in: BigInt,
    amount0_out: BigInt,
    amount1_in: BigInt,
    amount1_out: BigInt,
    block_number: u64,
    block_time: u64,
    transaction_from:String,
    transaction_to:String,
    transaction_gas_price:BigInt,
    transaction_gas_used:u64,
    transaction_hash:String,
    transaction_public_key:String,
    transaction_max_fee_per_gas:BigInt,
    transaction_max_priority_fee_per_gas:BigInt

}
#[substreams::handlers::map]
pub fn map_swap(block: eth::Block) -> Result<DatabaseChanges, substreams::errors::Error> {
    let block_number = block.number;
    let block_time = block.timestamp_seconds();
    let mut database_changes: DatabaseChanges = Default::default();

    
    save_swaps(block_number,block_time,block,&mut database_changes);
    Ok(database_changes)
}

fn  save_swaps(block_number: u64,block_time: u64,block:eth::Block,database_changes:&mut DatabaseChanges){
    let mut swaps: Vec<SwapV1> = vec![];
    for trx in block.transaction_traces {
        for log in trx.receipt.unwrap().logs {
            if let Some(swap) = extract_swap_event(&log) {
                // add transation info 
                let transaction_from = &trx.from;
                let transaction_from = Hex::encode(transaction_from);
                let transaction_to = &trx.to;
                let transaction_to = Hex::encode(transaction_to);
                let gas_price: &Option<eth::BigInt> =  &trx.gas_price;
                let transaction_gas_price = match gas_price {
                    Some(val) => BigInt::from_signed_bytes_be(&val.bytes),
                    None => BigInt::zero(),
                };
                let transaction_gas_used = &trx.gas_used;
                let transaction_hash = &trx.hash;
                let transaction_hash = Hex::encode(transaction_hash);
                let transaction_public_key = &trx.public_key;
                let transaction_public_key = Hex::encode(transaction_public_key);
                let max_fee_per_gas = &trx.max_fee_per_gas;
                let transaction_max_fee_per_gas = match max_fee_per_gas {
                    Some(val) => BigInt::from_signed_bytes_be(&val.bytes),
                    None => BigInt::zero(),
                };

                let max_priority_fee_per_gas = &trx.max_priority_fee_per_gas;
                let transaction_max_priority_fee_per_gas = match max_priority_fee_per_gas {
                    Some(val) => BigInt::from_signed_bytes_be(&val.bytes),
                    None => BigInt::zero(),
                };
                swaps.push(SwapV1 {
                    id: format!(
                        "{}_{}_{}",
                        Hex::encode(log.address.clone()),
                        Hex::encode(trx.hash.clone()),
                        log.index
                    ),
                    pair_address: Hex::encode(log.address.clone()),
                    sender: Hex::encode(swap.sender),
                    to: Hex::encode(swap.to),
                    amount0_in: swap.amount0_in,
                    amount0_out: swap.amount0_out,
                    amount1_in: swap.amount1_in,
                    amount1_out: swap.amount1_out,
                    block_number: block_number,
                    block_time: block_time,
                    transaction_from :transaction_from,
                    transaction_to:transaction_to,
                    transaction_gas_price:transaction_gas_price.into(),
                    transaction_gas_used:*transaction_gas_used,
                    transaction_hash:transaction_hash,
                    transaction_public_key:transaction_public_key,
                    transaction_max_fee_per_gas:transaction_max_fee_per_gas.into(),
                    transaction_max_priority_fee_per_gas:transaction_max_priority_fee_per_gas.into()
                });
            }
        }
    }
   
    for swap in swaps {
        log::info!("sink-sql: {:?}", swap);
        save_ethereum_block_uniswapv2_swaps(swap, database_changes);
    }
}

fn extract_swap_event(log: &Log) -> Option<Swap> {
    // let m = Swap::match_and_decode(log);
    if let Some(event) = Swap::match_and_decode(log) {
        log::info!("解析到 Swap 事件: {:?}", event);
        Some(event)
    } else {
        None
    }
}
fn save_ethereum_block_uniswapv2_swaps(swap: SwapV1, changes: &mut DatabaseChanges) {
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert("id".to_string(), swap.id);
    changes
        .push_change_composite(
            "ethereum_block_uniswapv2_swaps",
            composite_key,
            1,
            Operation::Create,
        )
        .change("block_number", (None, swap.block_number))
        .change("block_time", (None, swap.block_time))
        .change("swap_sender", (None, swap.sender))
        .change("swap_to", (None, swap.to))
        .change("amount0_in", (None, swap.amount0_in))
        .change("amount0_out", (None, swap.amount0_out))
        .change("amount1_in", (None, swap.amount1_in))
        .change("amount1_out", (None, swap.amount1_out))
        .change("transaction_from", (None,swap.transaction_from))
        .change("transaction_to", (None,swap.transaction_to))
        .change("transaction_gas_price", (None,swap.transaction_gas_price))
        .change("transaction_gas_used", (None,swap.transaction_gas_used))
        .change("transaction_hash", (None,swap.transaction_hash))
        .change("transaction_public_key", (None,swap.transaction_public_key))
        .change("transaction_max_fee_per_gas", (None,swap.transaction_max_fee_per_gas))
        .change("transaction_max_priority_fee_per_gas", (None,swap.transaction_max_priority_fee_per_gas))
        .change("pair_address", (None,swap.pair_address));
}
