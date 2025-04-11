
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


pub fn save_ethereum_block(block: eth::Block, database_changes: &mut DatabaseChanges){
    
    let mut new_pools: Vec<TransactionChanges> = vec![];
    // let params = String::from("factory_address=c0aee478e3658e2610c5f7a4a2e1777ce9e4f2ac&protocol_type_name=sushiswap_v2_pool");
    // let params: Params = serde_qs::from_str(params.as_str()).expect("Unable to deserialize params");
    let params: Params = Params {
        factory_address :String::from("c0aee478e3658e2610c5f7a4a2e1777ce9e4f2ac") ,
        protocol_type_name: String::from("c0aee478e3658e2610c5f7a4a2e1777ce9e4f2ac")

    };
    let block_number = block.number;
    let block_time = block.timestamp().seconds;
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
            let component_changes = &transaction_change.component_changes;
            for (contract_change_index,contract_change) in component_changes.iter().enumerate(){
                let contract_change_id = &contract_change.id;
                let change = &contract_change.change;
                let protocol_type = &contract_change.protocol_type.clone().unwrap(); 
                let protocol_type_financial_type = protocol_type.financial_type;
                let protocol_type_implementation_type = protocol_type.implementation_type;
                let protocol_type_name = &protocol_type.name;
                let protocol_type_attribute_schema: &Vec<Attribute> =  &protocol_type.attribute_schema;
                let mut protocol_type_attribute_schema_vec: Vec<HashMap<String,String>> = Vec::new();
                for attribute in protocol_type_attribute_schema {
                    let mut map:HashMap<String, String>  = HashMap::new();
                    map.insert("name".to_string(), attribute.name.clone());
                    map.insert("".to_string(), attribute.change.to_string());
                    map.insert("".to_string(),   Hex::encode(&attribute.value));

                    protocol_type_attribute_schema_vec.push(map);
                }
                let protocol_type_attribute_schema = serde_json::to_string(&protocol_type_attribute_schema_vec).unwrap();
                
            

                let contracts = &contract_change.contracts;
                for (contract_index,contract) in contracts.iter().enumerate(){
                 let contract =  Hex::encode(&contract);
                    save_ethereum_block_uniswapv2_contract_changes_contracts(
                        block_number,
                        block_time,
                        transaction_change_index,
                        transcation_index,
                        contract_change_index,
                        &contract_change_id,
                        change.clone(),
                        protocol_type_financial_type,
                        protocol_type_implementation_type,
                        protocol_type_name,
                        &protocol_type_attribute_schema,
                        contract_index,
                        contract,
                        database_changes
                    );
                }
                // let contract_changes: &Vec<ContractChange> = &transaction_change.contract_changes;
                // for (contract_change_index,contract_change) in contract_changes.iter().enumerate(){

                // }



                // let static_att: &Vec<Attribute> = &contract_change.static_att;
                // let tokens: &Vec<Vec<u8>> = &contract_change.tokens;
                // let tx: &Option<Transaction> = &contract_change.tx;
                
            }



            // let entity_changes = &transaction_change.entity_changes;
            // for (entity_change_index,entity_change) in entity_changes.iter().enumerate(){
            //     let attributes = &entity_change.attributes;
            //     let component_id = &entity_change.component_id;
            // }

        },
        None => {}
    };

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

    changes.push_change_composite("ethereum_block_uniswapv2_component_change", composite_key, 1, Operation::Create)
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


fn save_ethereum_block_uniswapv2_contract_changes_contracts(
    block_number:u64,
    block_time:i64,
    transaction_change_index:usize,
    transcation_index:u64,
    contract_change_index:usize,
    contract_change_id:&String,
    change:i32,
    protocol_type_financial_type:i32,
    protocol_type_implementation_type:i32,
    protocol_type_name:&String,
    protocol_type_attribute_schema:&String,
    contract_index:usize,
    contract:String,
    changes:&mut DatabaseChanges
){
    let id = format!("{}_{}_{}_{}_{}",block_number,block_time,transaction_change_index,transcation_index,contract_change_index);
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
    .change("contract_change_id", (None,contract_change_id))
    .change("change", (None,change))
    .change("protocol_type_financial_type", (None,protocol_type_financial_type))
    .change("protocol_type_implementation_type", (None,protocol_type_implementation_type))
    .change("protocol_type_name", (None,protocol_type_name))
    .change("protocol_type_attribute_schema", (None,protocol_type_attribute_schema))
    .change("contract_index", (None,contract_index as u64))
    .change("contract", (None,contract));

}