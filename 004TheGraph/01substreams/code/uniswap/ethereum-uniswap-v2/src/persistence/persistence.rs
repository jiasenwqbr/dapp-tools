use serde::Deserialize;
use substreams_database_change::pb::database::DatabaseChanges;
use std::str::FromStr;

use ethabi::ethereum_types::Address;
use substreams::prelude::BigInt;
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
    let params = String::from("factory_address=c0aee478e3658e2610c5f7a4a2e1777ce9e4f2ac&protocol_type_name=sushiswap_v2_pool");
    let params: Params = serde_qs::from_str(params.as_str()).expect("Unable to deserialize params");
    let block_number = block.number;
    let block_time = block.timestamp();
    get_pools(&block, &mut new_pools, &params);

    for (index,transaction_change) in new_pools.iter().enumerate(){
        saveChange(block_number,transaction_change,database_changes);
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


fn saveChange(block_number:u64,transaction_change:&TransactionChanges,database_changes:&mut DatabaseChanges){
    let balance_changes: &Vec<BalanceChange> = &transaction_change.balance_changes;

}