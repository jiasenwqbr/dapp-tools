use std::collections::HashMap;

use abi::Pancake_pair::events::Swap;
use eth_utils::address_pretty;
use pb::{pcs::{self, Pair}, tokens::Token};
use serde::Deserialize;
use substreams::{prelude::BigInt, Hex};
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::{self as eth, Log};
use eth_utils::address_decode;
extern crate core;

mod pb;
mod swap;
mod event;
mod eth_utils ;
mod utils;
pub mod abi;
pub mod rpc;
pub mod db;

#[derive(Debug)]
struct SwapV2 {
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

#[derive(Debug, Deserialize)]
struct Params {
    factory_address: String,
    protocol_type_name: String,
}


#[substreams::handlers::map]
pub fn map_postgres( params: String,block: eth::Block) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut database_changes: DatabaseChanges = Default::default();
    //save_swaps(block_number,block_time,&block,&mut database_changes);
    //persistence::save_ethereum_block(params,block, &mut database_changes);
    // pair create

    map_pairs(params,&block,&mut database_changes);
    handle_swap(&block,&mut database_changes);

    Ok(database_changes)
}

fn map_pairs(params: String,block: &eth::Block,changes:&mut DatabaseChanges){
    let params: Params = serde_qs::from_str(params.as_str()).expect("Unable to deserialize params");

    let block_number = block.number;
    let block_time = block.timestamp_seconds();
    let mut pairs = pcs::Pairs { pairs: vec![] };
    for trx in &block.transaction_traces {
        if hex::encode(&trx.to) != params.factory_address {
            continue;
        }
            for log in trx.receipt.clone().unwrap().logs {
            let sig = hex::encode(&log.topics[0]);

            if !event::is_pair_created_event(sig.as_str()) {
                continue;
            }

            pairs.pairs.push(pcs::Pair {
                address: eth_utils::address_pretty(&log.data[12..32]),
                token0_address: eth_utils::address_pretty(&log.topics[1][12..]),
                token1_address: eth_utils::address_pretty(&log.topics[2][12..]),
                creation_transaction_id: eth_utils::address_pretty(&trx.hash),
                block_num: block.number,
                log_ordinal: log.block_index as u64,
            })
        }
    }
    for pair in pairs.pairs{
        save_pair(
            block_number,
            block_time,
            pair,
            changes
        );
    }


    
}

fn save_pair(
    block_number:u64,
    block_time:u64,
    pair:Pair,
    changes:&mut DatabaseChanges
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert("id".to_string(), pair.address.to_string());
    changes
        .push_change_composite(
            "bsc_pancake_v2_pairs_create",
            composite_key,
            1,
            Operation::Create,
        )
        .change("block_number", (None,block_number))
        .change("block_time", (None,block_time))
        .change("token0_address", (None,pair.token0_address))
        .change("token1_address", (None,pair.token1_address))
        .change("creation_transaction_id", (None,pair.creation_transaction_id))
        .change("log_ordinal", (None,pair.log_ordinal));
}


fn handle_swap(block: &eth::Block,database_changes:&mut DatabaseChanges){
    let block_number = block.number;
    let block_time = block.timestamp_seconds();
    let mut swaps: Vec<SwapV2> = vec![];
    for trx in &block.transaction_traces{
        for log in trx.receipt.clone().unwrap().logs {
            if let Some(swap) = extract_swap_event(&log){
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

                swaps.push(
                    SwapV2{
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
                    }
                );
            }
        }
    }
    for swap in swaps{
        save_bsc_pancake_v2_swaps(
            swap,database_changes
        );
    }
}

fn extract_swap_event(log: &Log) -> Option<abi::Pancake_pair::events::Swap> {
    if Swap::match_log(log){
        match  Swap::decode(log) {
            Ok(event) => Some(event),
            Err(_) => None,
        }
    } else {
        None
    }
    
}

fn save_bsc_pancake_v2_swaps(swap: SwapV2, changes: &mut DatabaseChanges) {
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert("id".to_string(), swap.id);
    changes
        .push_change_composite(
            "bsc_pancake_v2_swaps",
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
