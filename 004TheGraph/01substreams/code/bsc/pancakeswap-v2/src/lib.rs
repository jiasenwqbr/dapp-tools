use std::collections::HashMap;

use pb::{eth::BigInt, pcs::{self, Pair}};
use serde::Deserialize;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams_ethereum::{
    pb::eth::v2::{self as eth, Log},
    Event
};
extern crate core;

mod pb;
mod swap;
mod event;
mod eth_utils ;
mod utils;

// #[substreams::handlers::map]
// pub fn map_pairs(blk: pb::eth::Block) -> Result<pcs::Pairs, Error> {
//     let mut pairs = pcs::Pairs { pairs: vec![] };

//     for trx in blk.transaction_traces {
//         /* PCS Factory address */
//         //0xbcfccbde45ce874adcb698cc183debcf17952812
//         if hex::encode(&trx.to) != "ca143ce32fe78f1f7019d7d551a6402fc5350c73" {
//             continue;
//         }

//         for log in trx.receipt.unwrap().logs {
//             let sig = hex::encode(&log.topics[0]);

//             if !event::is_pair_created_event(sig.as_str()) {
//                 continue;
//             }

//             pairs.pairs.push(pcs::Pair {
//                 address: address_pretty(&log.data[12..32]),
//                 token0_address: address_pretty(&log.topics[1][12..]),
//                 token1_address: address_pretty(&log.topics[2][12..]),
//                 creation_transaction_id: address_pretty(&trx.hash),
//                 block_num: blk.number,
//                 log_ordinal: log.block_index as u64,
//             })
//         }
//     }

//     Ok(pairs)
// }


// use substreams::store::StoreNew;
// #[substreams::handlers::store]
// pub fn store_pairs(pairs: pcs::Pairs, output:StoreSetRaw) {
//     log::info!("Building pair state");
//     for pair in pairs.pairs {
//         output.set(
//             pair.log_ordinal,
//             format!("pair:{}", pair.address),
//             &proto::encode(&pair).unwrap(),
//         );
//         output.set(
//             pair.log_ordinal as u64,
//             format!(
//                 "tokens:{}",
//                 utils::generate_tokens_key(
//                     pair.token0_address.as_str(),
//                     pair.token1_address.as_str(),
//                 )
//             ),
//             &proto::encode(&pair).unwrap(),
//         );
//     }
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