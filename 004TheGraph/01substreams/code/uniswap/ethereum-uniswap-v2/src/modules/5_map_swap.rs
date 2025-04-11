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
    from: String,
    to: String,
    amount0_in: BigInt,
    amount0_out: BigInt,
    amount1_in: BigInt,
    amount1_out: BigInt,
    block_number: u64,
    block_time: u64,
}
#[substreams::handlers::map]
pub fn map_swap(block: eth::Block) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut swaps: Vec<SwapV1> = vec![];
    // let params: Params = serde_qs::from_str(params.as_str()).expect("Unable to deserialize params");
    //  get_swaps(&block,&mut swaps,&params,block.number,block.timestamp_seconds());
    let block_number = block.number;
    let block_time = block.timestamp_seconds();
    for trx in block.transaction_traces {
        for log in trx.receipt.unwrap().logs {
            // add transation info 
            if let Some(swap) = extract_swap_event(&log) {
                swaps.push(SwapV1 {
                    id: format!(
                        "{}_{}_{}",
                        Hex::encode(log.address.clone()),
                        Hex::encode(trx.hash.clone()),
                        log.index
                    ),
                    from: Hex::encode(swap.sender),
                    to: Hex::encode(swap.to),
                    amount0_in: swap.amount0_in,
                    amount0_out: swap.amount0_out,
                    amount1_in: swap.amount1_in,
                    amount1_out: swap.amount1_out,
                    block_number: block_number,
                    block_time: block_time,
                });
            }
        }
    }
    let mut database_changes: DatabaseChanges = Default::default();
    for swap in swaps {
        log::info!("sink-sql: {:?}", swap);
        save_ethereum_block_uniswapv2_swaps(swap, &mut database_changes);
    }

    // Ok(BlockChanges { block: Some(tycho_block), changes: new_pools })
    Ok(database_changes)
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

// fn get_swaps(block: &eth::Block, swaps:&mut Vec<SwapV1> ,params: &Params,block_number:u64,block_time:u64){
//     let mut on_pair_swap = |event : Swap,_tx: &eth::TransactionTrace, _log: &eth::Log| {
//         swaps.push(SwapV1 {
//             id:format!("{}_{}",Hex::encode(_log.address.clone()),Hex::encode(_tx.hash.clone())),
//             from:Hex::encode(event.sender) ,
//             to:Hex::encode(event.to) ,
//             amount0_in:event.amount0_in,
//             amount0_out:event.amount0_out,
//             amount1_in:event.amount1_in,
//             amount1_out:event.amount1_out,
//             block_number:block_number,
//             block_time:block_time
//         });
//     };
//     let mut eh: EventHandler<'_> = EventHandler::new(block);
//     eh.filter_by_address(vec![Address::from_str(&params.factory_address).unwrap()]);
//     eh.on::<Swap, _>(&mut on_pair_swap);
//     eh.handle_events();
// }

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
        .change("swap_from", (None, swap.from))
        .change("swap_to", (None, swap.to))
        .change("amount0_in", (None, swap.amount0_in))
        .change("amount0_out", (None, swap.amount0_out))
        .change("amount1_in", (None, swap.amount1_in))
        .change("amount1_out", (None, swap.amount1_out));
}
