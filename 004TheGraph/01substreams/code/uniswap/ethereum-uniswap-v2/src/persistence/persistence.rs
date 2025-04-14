
use serde::Deserialize;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use std::{collections::HashMap, str::FromStr};

use ethabi::ethereum_types::Address;
use substreams::{prelude::BigInt, Hex};
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_helper::{event_handler::EventHandler, hex::Hexable};
use crate::abi::factory::events::PairCreated;
use tycho_substreams::prelude::*;



#[derive(Debug, Deserialize)]
struct Params {
    factory_address: String,
    protocol_type_name: String,
}


pub fn save_ethereum_block(params:String,block: eth::Block, database_changes: &mut DatabaseChanges){
    
    let mut new_pools: Vec<TransactionChanges> = vec![];
    let block_number = block.number;
    let block_time = block.timestamp().seconds;
    let params: Params = serde_qs::from_str(params.as_str()).expect("Unable to deserialize params");
    get_pools(&block, &mut new_pools, &params);

    for (transaction_change_index,transaction_change) in new_pools.iter().enumerate(){
        save_change(
            block_number,
            block_time,
            transaction_change_index,
            transaction_change,
            database_changes
        );
    }

}

fn get_pools(block: &eth::Block, new_pools: &mut Vec<TransactionChanges>, params: &Params) {
    // Extract new pools from PairCreated events
    let mut on_pair_created = |event: PairCreated, _tx: &eth::TransactionTrace, _log: &eth::Log| {
        let tycho_tx: Transaction = _tx.into();

        new_pools.push(TransactionChanges {
            tx: Some(tycho_tx.clone()),
            contract_changes: vec![],
            entity_changes: vec![EntityChanges {
                component_id: event.pair.to_hex(),
                attributes: vec![
                    Attribute {
                        name: "reserve0".to_string(),
                        value: BigInt::from(0).to_signed_bytes_be(),
                        change: ChangeType::Creation.into(),
                    },
                    Attribute {
                        name: "reserve1".to_string(),
                        value: BigInt::from(0).to_signed_bytes_be(),
                        change: ChangeType::Creation.into(),
                    },
                ],
            }],
            component_changes: vec![ProtocolComponent {
                id: event.pair.to_hex(),
                tokens: vec![event.token0.clone(), event.token1.clone()],
                contracts: vec![],
                static_att: vec![
                    // Trading Fee is hardcoded to 0.3%, saved as int in bps (basis points)
                    Attribute {
                        name: "fee".to_string(),
                        value: BigInt::from(30).to_signed_bytes_be(),
                        change: ChangeType::Creation.into(),
                    },
                    Attribute {
                        name: "pool_address".to_string(),
                        value: event.pair.clone(),
                        change: ChangeType::Creation.into(),
                    },
                ],
                change: i32::from(ChangeType::Creation),
                protocol_type: Some(ProtocolType {
                    name: params.protocol_type_name.to_string(),
                    financial_type: FinancialType::Swap.into(),
                    attribute_schema: vec![],
                    implementation_type: ImplementationType::Custom.into(),
                }),
                tx: Some(tycho_tx),
            }],
            balance_changes: vec![
                BalanceChange {
                    token: event.token0,
                    balance: BigInt::from(0).to_signed_bytes_be(),
                    component_id: event.pair.to_hex().as_bytes().to_vec(),
                },
                BalanceChange {
                    token: event.token1,
                    balance: BigInt::from(0).to_signed_bytes_be(),
                    component_id: event.pair.to_hex().as_bytes().to_vec(),
                },
            ],
        })
    };

    let mut eh = EventHandler::new(block);
    

    eh.filter_by_address(vec![Address::from_str(&params.factory_address).unwrap()]);

    eh.on::<PairCreated, _>(&mut on_pair_created);
    eh.handle_events();
}


fn save_change(
    block_number:u64,
    block_time:i64,
    transaction_change_index:usize,
    transaction_change:&TransactionChanges,
    database_changes:&mut DatabaseChanges
){
    match &transaction_change.tx {
        Some(trans) => {
            let transcation_index = trans.index;
            let from = &trans.from;
            let from = Hex::encode(from);
            let to: &Vec<u8> = &trans.to;
            let to =  Hex::encode(to);
            let balance_changes: &Vec<BalanceChange> = &transaction_change.balance_changes;
            for (balance_change_index,balance_change) in balance_changes.iter().enumerate(){
                let balance: &Vec<u8>  =  &balance_change.balance;
                let balance =  Hex::encode(&balance);
                let component_id: &Vec<u8> = &balance_change.component_id;
                let component_id =  Hex::encode(&component_id);
                let token = &balance_change.token;
                let token =  Hex::encode(token);
                save_ethereum_block_uniswapv2_transcation_change(
                    block_number,
                    block_time,
                    transaction_change_index,
                    transcation_index,
                    balance_change_index,
                    token,
                    &from,
                    &to,
                    balance,
                    component_id,
                    database_changes
                );
            }

            let entity_changes = &transaction_change.entity_changes;
            for (entity_change_index,entity_change) in entity_changes.iter().enumerate(){
                let component_id = &entity_change.component_id;
                let attributes = &entity_change.attributes;

                let reserve0 = &attributes[0].name;
                let reserve0_value: &Vec<u8> =  &attributes[0].value;
                let reserve0_value = BigInt::from_signed_bytes_be(&reserve0_value);
                let reserve0_change = attributes[0].change;
                let reserve1  = &attributes[1].name;
                let reserve1_value = &attributes[1].value;
                let reserve1_value = BigInt::from_signed_bytes_be(&reserve1_value);
                let reserve1_change = attributes[1].change;

                save_ethereum_block_uniswapv2_entity_changes(
                    block_number,
                    block_time,
                    transaction_change_index,
                    transcation_index,
                    entity_change_index,
                    component_id,
                    reserve0,
                    reserve0_value,
                    reserve0_change,
                    reserve1,
                    reserve1_value,
                    reserve1_change,
                    database_changes
                );
                
            }

            let balance_changes = &transaction_change.balance_changes;
            let token0 = &balance_changes[0].token;
            let token0 = Hex::encode(token0);
            let token0_component_id = &balance_changes[0].component_id;     
            let  token0_component_id = Hex::encode(token0_component_id);
            let token0_balance = &balance_changes[0].balance;
            let token0_balance = BigInt::from_signed_bytes_be(&token0_balance);

            let token1 = &balance_changes[1].token;
            let token1 = Hex::encode(token1);
            let token1_component_id = &balance_changes[1].component_id;    
            let token1_component_id = Hex::encode(token1_component_id);
            let token1_balance = &balance_changes[1].balance;
            let token1_balance = BigInt::from_signed_bytes_be(&token1_balance);

            save_ethereum_block_uniswapv2_balance_changes(
                block_number,
                block_time,
                transaction_change_index,
                transcation_index,
                token0,
                token0_component_id,
                token0_balance,
                token1,
                token1_component_id,
                token1_balance,
                database_changes
            );

            let component_changes = &transaction_change.component_changes;
            for (component_change_index,component_change) in component_changes.iter().enumerate(){
                let component_change_id = &component_change.id;
                let token0 = Hex::encode(&component_change.tokens[0]);
                let token1 = Hex::encode(&component_change.tokens[1]);
                let fee_value = BigInt::from_signed_bytes_be(&component_change.static_att[0].value);
                let fee_change = &component_change.static_att[0].change;
                let pool_address = Hex::encode(&component_change.static_att[1].value);
                let pool_change = &component_change.static_att[1].change;
                let change = &component_change.change;
                let protocol_type = &component_change.protocol_type;
                let protocol_type_name:String;
                let protocol_financial_type;
                let protocol_type_implementation_type;
                match protocol_type {
                    Some(val) => {
                        protocol_type_name = val.name.clone();
                        protocol_financial_type = val.financial_type;
                        protocol_type_implementation_type = val.implementation_type;

                    },
                    None => {
                        protocol_type_name = String::new();
                        protocol_financial_type = 0;
                        protocol_type_implementation_type = 0;
                    },
                }
                let tx_from;
                let tx_to;
                let tx_hash;
                let tx_index;
                let tx = &transaction_change.tx;
                match tx {
                    Some(val) => {
                        tx_from = Hex::encode(val.from.clone());
                        tx_to = Hex::encode(val.to.clone());
                        tx_hash = Hex::encode(val.hash.clone());
                        tx_index = val.index;
                    },
                    None => {
                        tx_from = String::new();
                        tx_to = String::new();
                        tx_hash = String::new();
                        tx_index = 0;
                    },
                }
                let id = format!("{}_{}_{}_{}_{}",block_number,block_time,transaction_change_index,transcation_index,component_change_index);

                save_ethereum_block_uniswapv2_component_changes(
                    id ,
                    block_number,
                    block_time,
                    transaction_change_index ,
                    transcation_index ,
                    component_change_index ,
                    component_change_id ,
                    token0 ,
                    token1 ,
                    fee_value ,
                    fee_change ,
                    pool_address ,
                    pool_change ,
                    change,
                    protocol_type_name,
                    protocol_financial_type,
                    protocol_type_implementation_type,
                    tx_from ,
                    tx_to ,
                    tx_hash ,
                    tx_index ,
                    database_changes
                );
                
                
            }

        },
        None => {}
    };

}

fn save_ethereum_block_uniswapv2_component_changes(
    id :String,
    block_number:u64,
    block_time:i64,
    transaction_change_index:usize,
    transcation_index:u64,
    component_change_index:usize,
    component_change_id:&String,
    token0 :String,
    token1:String,
    fee_value:BigInt,
    fee_change:&i32,
    pool_address :String,
    pool_change:&i32,
    change:&i32,
    protocol_type_name:String,
    protocol_financial_type:i32,
    protocol_type_implementation_type:i32,
    tx_from:String,
    tx_to :String,
    tx_hash:String,
    tx_index:u64,
    changes:&mut DatabaseChanges
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert(
        "id".to_string(),
        id,
    );
    changes.push_change_composite("ethereum_block_uniswapv2_component_changes", composite_key, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("block_time", (None,block_time))
    .change("transaction_change_index", (None,transaction_change_index as u64))
    .change("transcation_index", (None,transcation_index as u64))
    .change("component_change_index", (None,component_change_index as u64))
    .change("component_change_id", (None,component_change_id))
    .change("token0", (None,token0))
    .change("token1", (None,token1))
    .change("fee_value", (None,fee_value))
    .change("fee_change", (None,*fee_change))
    .change("pool_address", (None,pool_address))
    .change("pool_change", (None,*pool_change))
    .change("change", (None,*change))
    .change("protocol_type_name", (None,protocol_type_name))
    .change("protocol_financial_type", (None,protocol_financial_type))
    .change("protocol_type_implementation_type", (None,protocol_type_implementation_type))
    .change("tx_from", (None,tx_from))
    .change("tx_to", (None,tx_to))
    .change("tx_hash", (None,tx_hash))
    .change("tx_index", (None,tx_index));
}

fn save_ethereum_block_uniswapv2_balance_changes(
    block_number:u64,
    block_time:i64,
    transaction_change_index:usize,
    transcation_index:u64,
    token0:String,
    token0_component_id:String,
    token0_balance:BigInt,
    token1:String,
    token1_component_id:String,
    token1_balance:BigInt,
    changes:&mut DatabaseChanges 
){
    let id = format!("{}_{}_{}_{}",block_number,block_time,transaction_change_index,transcation_index);
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert(
        "id".to_string(),
        id,
    );
    changes.push_change_composite("ethereum_block_uniswapv2_balance_changes", composite_key, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("block_time", (None,block_time))
    .change("transaction_change_index", (None,transaction_change_index as u64))
    .change("transcation_index", (None,transcation_index as u64))
    .change("token0", (None,token0))
    .change("token0_component_id", (None,token0_component_id))
    .change("token0_balance", (None,token0_balance))
    .change("token1", (None,token1))
    .change("token1_component_id", (None,token1_component_id))
    .change("token1_balance", (None,token1_balance));
}


fn save_ethereum_block_uniswapv2_entity_changes(
    block_number:u64,
    block_time:i64,
    transaction_change_index:usize,
    transcation_index:u64,
    entity_change_index:usize,
    component_id:&String,
    reserve0:&String,
    reserve0_value:BigInt,
    reserve0_change:i32,
    reserve1:&String,
    reserve1_value:BigInt,
    reserve1_change:i32,
    changes:&mut DatabaseChanges
){
    let id = format!("{}_{}_{}_{}_{}",block_number,block_time,transaction_change_index,transcation_index,entity_change_index);
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert(
        "id".to_string(),
        id,
    );
    changes.push_change_composite("ethereum_block_uniswapv2_entity_changes", composite_key, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("block_time", (None,block_time))
    .change("transaction_change_index", (None,transaction_change_index as u64))
    .change("transcation_index", (None,transcation_index as u64))
    .change("entity_change_index", (None,entity_change_index as u64))
    .change("component_id", (None,component_id))
    .change("reserve0", (None,reserve0))
    .change("reserve0_value", (None,reserve0_value))
    .change("reserve0_change", (None,reserve0_change))
    .change("reserve1",(None,reserve1))
    .change("reserve1_value", (None,reserve1_value))
    .change("reserve1_change", (None,reserve1_change));
}

fn save_ethereum_block_uniswapv2_transcation_change(
    block_number:u64,
    block_time:i64,
    transaction_change_index:usize,
    transcation_index:u64,
    balance_change_index:usize,
    token:String,
    trans_from:&String,
    trans_to:&String,
    balance:String,
    component_id:String,
    changes:&mut DatabaseChanges
){
    let id = format!("{}_{}_{}_{}_{}",block_number,block_time,transaction_change_index,transcation_index,balance_change_index);
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert(
        "id".to_string(),
        id,
    );

    changes.push_change_composite("ethereum_block_uniswapv2_transcation_change", composite_key, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("block_time", (None,block_time))
    .change("transaction_change_index", (None,transaction_change_index as u64))
    .change("transcation_index", (None,transcation_index as u64))
    .change("token", (None,token))
    .change("trans_from", (None,trans_from))
    .change("trans_to", (None,trans_to))
    .change("balance", (None,balance))
    .change("component_id", (None,component_id));
}
