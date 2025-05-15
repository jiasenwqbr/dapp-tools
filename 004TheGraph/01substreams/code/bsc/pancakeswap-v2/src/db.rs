
use std::collections::HashMap;

use anyhow::Ok;
use substreams::log;
use substreams::pb::substreams::Clock;
use substreams::store::{self, DeltaExt, DeltaProto};
use substreams_database_change::pb::database::table_change::Operation;
use substreams_database_change::pb::database::DatabaseChanges;

use crate::pb::pcs::{Burn, Events, Mint, Pair, Swap};
use crate::pb::tokens::Token;
use crate::save_bsc_pancake_v2_swaps;
pub fn process(
    block: &Clock,
    pcs_token_deltas: store::Deltas<DeltaProto<Token>>,
    pair_deltas: store::Deltas<DeltaProto<Pair>>,
    total_deltas: store::Deltas<store::DeltaInt64>,
    volumes_deltas: store::Deltas<store::DeltaBigDecimal>,
    reserves_deltas: store::Deltas<store::DeltaString>,
    events: Events,
    pcs_tokens_store: &store::StoreGetRaw,
    database_changes: &mut DatabaseChanges
){
   
    handle_pcs_token_deltas(pcs_token_deltas,database_changes);
    handle_pair_deltas( pair_deltas,database_changes);
    handle_events(events,database_changes);
    log::info!("the database_changes are: {:?}",database_changes);
    
}

fn handle_pcs_token_deltas( pcs_token_deltas: store::Deltas<DeltaProto<Token>>,changes:&mut DatabaseChanges){
    for delta in pcs_token_deltas.iter().key_first_segment_eq("token"){
        save_bsc_pancake_v2_pcs_substream_token(delta,changes);
    }
}

fn save_bsc_pancake_v2_pcs_substream_token(delta:&DeltaProto<Token>,changes:&mut DatabaseChanges){
    let token_name = &delta.new_value.name;
    let symbol = &delta.new_value.symbol;
    let token_decimals = &delta.new_value.decimals;
    let address = &delta.new_value.address;
    let id = format!("{}",address);
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id.to_string());
    changes.push_change_composite("bsc_pancake_v2_pcs_substream_token", keys, 1, Operation::Create)
    .change("token_name", (None,token_name))
    .change("symbol", (None,symbol))
    .change("token_decimals", (None,*token_decimals));

}

fn handle_pair_deltas(pair_deltas: store::Deltas<DeltaProto<Pair>>,changes:&mut DatabaseChanges){
    for delta in pair_deltas.iter().key_first_segment_eq("pair"){
        save_bsc_pancake_v2_pcs_substream_pair(delta,changes);
    }   
}

fn save_bsc_pancake_v2_pcs_substream_pair(delta:&DeltaProto<Pair>,changes:&mut DatabaseChanges){
    let asddress = &delta.new_value.address;
    let block_num = &delta.new_value.block_num;
    let creation_transaction_id = &delta.new_value.creation_transaction_id;
    let log_ordinal = &delta.new_value.log_ordinal;
    let token0_address = &delta.new_value.token0_address;
    let token1_address = &delta.new_value.token1_address;
    let id = format!("{}",asddress);
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id.to_string());
    changes.push_change_composite("bsc_pancake_v2_pcs_substream_pair", keys, 1, Operation::Create)
    .change("asddress", (None,asddress))
    .change("block_num", (None,*block_num))
    .change("creation_transaction_id", (None,creation_transaction_id))
    .change("log_ordinal", (None,*log_ordinal))
    .change("token0_address", (None,token0_address))
    .change("token1_address", (None,token1_address));
}

fn handle_events( events: Events,changes:&mut DatabaseChanges){
    for event in events.events.iter(){
       let log_ordinal = event.log_ordinal;
       let pair_address = &event.pair_address;
       let timestamp = event.timestamp;
       let token0 = &event.token0;
       let token1 = &event.token1;
       let transaction_id = &event.transaction_id;
       match  event.r#type.as_ref().unwrap() {

        crate::pb::pcs::event::Type::Swap(swap) => save_bsc_pancake_v2_pcs_substream_swaps(log_ordinal,pair_address,timestamp,token0,token1,transaction_id,swap,changes),
        crate::pb::pcs::event::Type::Burn(burn) => save_bsc_pancake_v2_pcs_substream_burn(log_ordinal,pair_address,timestamp,token0,token1,transaction_id,burn,changes),
        crate::pb::pcs::event::Type::Mint(mint) => save_bsc_pancake_v2_pcs_substream_mint(log_ordinal,pair_address,timestamp,token0,token1,transaction_id,mint,changes),
        }
    }    
}

fn save_bsc_pancake_v2_pcs_substream_swaps(log_ordinal: u64,
    pair_address: &String,
    timestamp: u64,
    token0: &String,
    token1: &String,
    transaction_id: &String,
    swap:&Swap,
    changes: &mut DatabaseChanges){

        let swap_id = &swap.id;
        let amount0_in = &swap.amount0_in;
        let amount0_out = &swap.amount0_out;
        let amount1_in = &swap.amount1_in;
        let amount1_out = &swap.amount1_out;
        let amount_bnb = &swap.amount_bnb;
        let amount_usd = &swap.amount_usd;
        let from = &swap.from;
        let to = &swap.to;
        let log_address = &swap.log_address;
        let sender = &swap.sender;
        let trade_volume0 = &swap.trade_volume0;
        let trade_volume1 = &swap.trade_volume1;
        let trade_volume_usd0 = &swap.trade_volume_usd0;
        let trade_volume_usd1 = &swap.trade_volume_usd1;
        let volume_token0 = &swap.volume_token0;
        let volume_token1 = &swap.volume_token1;
        let volume_usd = &swap.volume_usd;


        let id: String = format!("{}_{}_{}",transaction_id,log_ordinal,swap_id);
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), id.to_string());
        changes.push_change_composite("bsc_pancake_v2_pcs_substream_swaps", keys, 1, Operation::Create)
        .change("pair_address", (None,pair_address))
        .change("block_timestamp", (None,timestamp))
        .change("token0", (None,token0))
        .change("token1", (None,token1))
        .change("transaction_id", (None,transaction_id))
        .change("swap_id", (None,swap_id))
        .change("amount0_in", (None,amount0_in))
        .change("amount0_out", (None,amount0_out))
        .change("amount1_in", (None,amount1_in))
        .change("amount1_out", (None,amount1_out))
        .change("amount_bnb", (None,amount_bnb))
        .change("amount_usd", (None,amount_usd))
        .change("swap_from", (None,from))
        .change("swap_to", (None,to))
        .change("log_address", (None,log_address))
        .change("sender", (None,sender))
        .change("trade_volume0", (None,trade_volume0))
        .change("trade_volume1", (None,trade_volume1))
        .change("trade_volume_usd0", (None,trade_volume_usd0))
        .change("trade_volume_usd1", (None,trade_volume_usd1))
        .change("volume_token0", (None,volume_token0))
        .change("volume_token1", (None,volume_token1))
        .change("volume_usd", (None,volume_usd));
}
fn save_bsc_pancake_v2_pcs_substream_burn(log_ordinal: u64,
    pair_address: &String,
    timestamp: u64,
    token0: &String,
    token1: &String,
    transaction_id: &String,
    swap:&Burn,
    changes: &mut DatabaseChanges){}

fn save_bsc_pancake_v2_pcs_substream_mint(log_ordinal: u64,
    pair_address: &String,
    timestamp: u64,
    token0: &String,
    token1: &String,
    transaction_id: &String,
    mint:&Mint,
    changes: &mut DatabaseChanges){}