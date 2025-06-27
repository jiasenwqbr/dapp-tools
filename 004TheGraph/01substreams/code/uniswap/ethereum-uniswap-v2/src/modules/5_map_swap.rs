
use substreams::log;
use std::collections::HashMap;
use substreams::{prelude::BigInt, Hex};
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams_ethereum::{
    pb::eth::v2::{self as eth, Log},
    Event
};
use crate::{abi::{factory::events::PairCreated, pool::events::{Swap,Sync}}, persistence::persistence};

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
#[derive(Debug)]
struct SyncV1 {
    id : String,
    reserve0 : BigInt,
    reserve1 : BigInt,
    pair_address:String,
    transaction_hash:String,
    block_number: u64,
    block_time: u64,
}
#[derive(Debug)]
struct PairV1 {
    id : String,
    pair_address : String,
    token0_address : String,
    token1_address : String,
    transaction_hash:String,
    block_number: u64,
    block_time: u64,
}

#[substreams::handlers::map]
pub fn map_swap( params: String,block: eth::Block) -> Result<DatabaseChanges, substreams::errors::Error> {
    let block_number = block.number;
    let block_time = block.timestamp_seconds();
    let mut database_changes: DatabaseChanges = Default::default();
    save_swaps(block_number,block_time,&block,&mut database_changes);
    // persistence::save_ethereum_block(params,block, &mut database_changes);
    Ok(database_changes)
}

fn  save_swaps(block_number: u64,block_time: u64,block: &eth::Block,database_changes:&mut DatabaseChanges){
    let mut swaps: Vec<SwapV1> = vec![];
    let mut syncs: Vec<SyncV1> = vec![];
    let mut pairs:Vec<PairV1> = vec![];
    for trx in block.transaction_traces.clone() {
        for log in trx.receipt.unwrap().logs {
            let transaction_hash = &trx.hash;
            
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

            // sync 
            if let Some(sync) = extract_sync_event(&log){
                let reserve0 = sync.reserve0;
                let reserve1 = sync.reserve1;
                let pair_address = Hex::encode(log.address.clone());
                syncs.push(SyncV1{
                    id: format!(
                        "{}_{}_{}",
                        Hex::encode(log.address.clone()),
                        Hex::encode(trx.hash.clone()),
                        log.index
                    ),
                    reserve0:reserve0,
                    reserve1:reserve1,
                    pair_address:pair_address,
                    transaction_hash:Hex::encode(transaction_hash),
                    block_number,
                    block_time,
                }); 
            }

            // paircreated
            if let Some(pair) = extract_pair_create_event(&log){
                let pair_address =  Hex::encode(pair.pair);
                let token0_address = Hex::encode(pair.token0);
                let token1_address = Hex::encode(pair.token1);

                pairs.push(PairV1 { id: format!(
                        "{}_{}_{}",
                        Hex::encode(log.address.clone()),
                        Hex::encode(trx.hash.clone()),
                        log.index
                    ), 
                    pair_address: pair_address, 
                    token0_address: token0_address,
                    token1_address: token1_address, 
                    transaction_hash: Hex::encode(transaction_hash),
                    block_number: block_number, 
                    block_time: block_time });
            }

        }
    
       
    
    }

    
   
    for swap in swaps {
        log::info!("sink-sql-swap: {:?}", swap);
        save_ethereum_block_uniswapv2_swaps(swap, database_changes);
    }

    for sync in syncs {
         log::info!("sink-sql-sync: {:?}", sync);
         save_ethereum_block_uniswapv2_reserves(sync, database_changes);
    }

    for pair in pairs {
        log::info!("sink-pair-created:{:?}",pair);
        save_ethereum_block_uniswapv2_pairs(pair,database_changes);
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

fn extract_sync_event(log:&Log) -> Option<Sync> {
    if let Some(event) = Sync::match_and_decode(log) {
        Some(event)
    } else {
        None
    }
}

fn extract_pair_create_event(log:&Log) -> Option<PairCreated> {
    if let Some(event) = PairCreated::match_and_decode(log){
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

fn  save_ethereum_block_uniswapv2_reserves(sync:SyncV1, changes:&mut DatabaseChanges){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert("id".to_string(), sync.id);
    changes.push_change_composite("ethereum_block_uniswapv2_reserves", composite_key, 1, Operation::Create)
    .change("reserve0", (None,sync.reserve0))
    .change("reserve1", (None,sync.reserve1))
    .change("pair_address", (None,sync.pair_address))
    .change("transaction_hash", (None,sync.transaction_hash))
    .change("block_number", (None,sync.block_number))
    .change("block_time", (None,sync.block_time));

}

fn save_ethereum_block_uniswapv2_pairs(pair:PairV1,changes:&mut DatabaseChanges){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert("id".to_string(), pair.id);
    changes.push_change_composite("ethereum_block_uniswapv2_substream_pairs", composite_key, 1, Operation::Create)
    .change("pair_address", (None,pair.pair_address))
    .change("token0_address", (None,pair.token0_address))
    .change("token1_address", (None,pair.token1_address))
    .change("transaction_hash", (None,pair.transaction_hash))
    .change("block_number", (None,pair.block_number))
    .change("block_time", (None,pair.block_time));

}