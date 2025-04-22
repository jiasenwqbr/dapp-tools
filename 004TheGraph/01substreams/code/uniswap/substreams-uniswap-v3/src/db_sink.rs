use std::collections::HashMap;
use std::ops::Div;
use substreams::{key, log, scalar::{BigDecimal, BigInt}, store::{DeltaArray, DeltaBigDecimal, DeltaBigInt, DeltaExt, DeltaProto, Deltas, StoreGet, StoreGetBigDecimal, StoreGetBigInt, StoreGetInt64, StoreGetProto}, Hex};
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};

use crate::{pb::uniswap::{events::{self, position_event::Type, DecreaseLiquidityPosition, IncreaseLiquidityPosition, PoolSqrtPrice, PositionEvent}, Erc20Token, Events, Pool, Pools}, utils::{self, pool_windows_id_fields, time_as_i64_address_as_str, token_windows_id_fields}};
use crate::pb::uniswap::events::pool_event::Type::{Burn as BurnEvent, Mint as MintEvent, Swap as SwapEvent};
// pub fn save_data(data: String,data_type:String,timestamp:i64,changes: &mut DatabaseChanges) {
//     let id = format!("{}_{}",timestamp,data_type);
//     let mut keys: HashMap<String, String> = HashMap::new();
//     keys.insert("id".to_string(), id.to_string());

//     changes.push_change_composite("ethereum_unswap_v3_json", keys, 1, Operation::Create)
//     .change("timestamp_sub", (None,timestamp))
//     .change("data_type", (None,data_type))
//     .change("data", (None,data));
// }


pub fn pool_count_deltas(timestamp:i64,pool_count_deltas:Deltas<DeltaBigInt>,changes:&mut DatabaseChanges) {
    //save_ethereum_uniswap_v3_
    pool_count_deltas.iter().for_each(|delta| {
        let id = "0x1F98431c8aD98523631AE4a59f267346ea31F984".to_string();
        save_ethereum_uniswap_v3_pool_count_deltas(id,timestamp,changes,delta.new_value.clone());
    });
}

fn save_ethereum_uniswap_v3_pool_count_deltas(
    id:String,
    timestamp :i64,
    changes: &mut DatabaseChanges,
    pool_count:BigInt
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id.to_string());
    changes.push_change_composite("ethereum_uniswap_v3_factory", keys, 1, Operation::Update)
    .change("update_at", (None,timestamp))
    .change("pool_count", (None,pool_count));
}

pub fn tx_count_deltas(tx_count_deltas:&Deltas<DeltaBigInt>,changes:&mut DatabaseChanges){
    for delta in tx_count_deltas.iter().key_first_segment_eq("factory"){
        let id = "0x1F98431c8aD98523631AE4a59f267346ea31F984".to_string();
        save_ethereum_uniswap_v3_tx_count(delta.new_value.clone(),id,changes);
    }
}

fn save_ethereum_uniswap_v3_tx_count(tx_count:BigInt,id:String,changes:&mut DatabaseChanges){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id.to_string());
    changes.push_change_composite("ethereum_uniswap_v3_factory", keys, 1, Operation::Update)
    .change("tx_count", (None,tx_count));
}

pub fn swaps_volume_deltas(swaps_volume_deltas:&Deltas<DeltaBigDecimal>,changes:&mut DatabaseChanges){
    for delta in swaps_volume_deltas
    .iter()
    .key_first_segment_eq("factory")
    .key_last_segment_in([ "totalVolumeUSD",
    "untrackedVolumeUSD",
    "totalFeesUSD",
    "totalVolumeETH",
    "totalFeesETH",]){
        save_ethereum_uniswap_v3_swaps_volume_deltas(delta,changes);
    }
}
fn save_ethereum_uniswap_v3_swaps_volume_deltas(delta:&DeltaBigDecimal,changes:&mut DatabaseChanges){
    
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), "0x1F98431c8aD98523631AE4a59f267346ea31F984".to_string());
    let change = changes.push_change_composite("ethereum_uniswap_v3_factory", keys, 1, Operation::Update);
    
    let key: &String = &delta.key;
    if "untrackedVolumeUSD" == key {
        change.change("untracked_volume_usd", (None,delta.new_value.clone()));
    }
    if "totalFeesUSD"==key {
        change.change("total_fees_usd", (None,delta.new_value.clone()));
    }
    if "totalVolumeETH"==key {
        change.change("total_volume_eth", (None,delta.new_value.clone()));
    }
    if "totalFeesETH" == key {
        change.change("total_fees_eth", (None,delta.new_value.clone()));
    }
}

pub fn derived_factory_tvl_deltas(derived_factory_tvl_deltas:&Deltas<DeltaBigDecimal>,changes:&mut DatabaseChanges){
    for delta in derived_factory_tvl_deltas
        .iter()
        .key_first_segment_eq("factory")
        .key_last_segment_in([
            "totalValueLockedUSD",
            "totalValueLockedUSDUntracked",
            "totalValueLockedETH",
            "totalValueLockedETHUntracked",
        ]){

            save_ethereum_uniswap_v3_derived_factory_tvl_deltas(delta,changes);
        }

}
fn save_ethereum_uniswap_v3_derived_factory_tvl_deltas(delta:&DeltaBigDecimal,changes:&mut DatabaseChanges){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), "0x1F98431c8aD98523631AE4a59f267346ea31F984".to_string());
    let change = changes.push_change_composite("ethereum_uniswap_v3_factory", keys, 1, Operation::Update);
    let key: &String = &delta.key;
    if "totalValueLockedUSD" == key {
        change.change("total_value_locked_usd", (None,delta.new_value.clone()));
    }
    if "totalValueLockedUSDUntracked" == key {
        change.change("total_value_locked_usd_untracked", (None,delta.new_value.clone()));
    }
    if "totalValueLockedETH" == key {
        change.change("total_value_locked_eth", (None,delta.new_value.clone()));
    }
    if "totalValueLockedETHUntracked" == key {
        change.change("total_value_locked_eth_untracked", (None,delta.new_value.clone()));
    }
}

pub fn pools_created_pool_entity_changes(changes: &mut DatabaseChanges, pools : &Pools){
    for pool in &pools.pools {
        save_uniswap_v3_pools(changes,pool);
    }
}
fn save_uniswap_v3_pools(changes:&mut DatabaseChanges,pool:&Pool){
    let bigint0 = BigInt::zero();
    let bigdecimal0 = BigDecimal::zero();
    let id = format!("0x{}",&pool.address);
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_uniswap_v3_pools", keys, 1, Operation::Create)
    .change("created_at_timestamp", (None,BigInt::from(pool.created_at_timestamp)))
    .change("created_at_block_number", (None,pool.created_at_block_number))
    .change("token0", (None,pool.token0.as_ref().unwrap().address.clone()))
    .change("token1", (None,pool.token1.as_ref().unwrap().address.clone()))
    .change("fee_tier", (None,&pool.fee_tier))
    .change("liquidity", (None,&bigint0))
    .change("sqrt_price", (None,&bigint0))
    .change("fee_growth_global_0x128", (None,&bigint0))
    .change("fee_growth_global_1x128", (None,&bigint0))
    .change("token0_price", (None,&bigdecimal0))
    .change("token1_price", (None,&bigdecimal0))
    .change("tick", (None,&bigint0))
    .change("observation_index", (None,&bigint0))
    .change("volume_token0", (None,&bigdecimal0))
    .change("volume_token1", (None,&bigdecimal0))
    .change("volume_usd", (None,&bigdecimal0))
    .change("untracked_volume_usd", (None,&bigdecimal0))
    .change("fees_usd", (None,&bigdecimal0))
    .change("tx_count", (None,&bigint0))
    .change("collected_fees_token0", (None,&bigdecimal0))
    .change("collected_fees_token1", (None,&bigdecimal0))
    .change("collected_fees_usd", (None,&bigdecimal0))
    .change("total_value_locked_token0", (None,&bigdecimal0))
    .change("total_value_locked_token1", (None,&bigdecimal0))
    .change("total_value_locked_eth", (None,&bigdecimal0))
    .change("total_value_locked_usd", (None,&bigdecimal0))
    .change("total_value_locked_usd_untracked", (None,&bigdecimal0))
    .change("total_value_locked_eth_untracked", (None,&bigdecimal0))
    .change("liquidity_provider_count", (None,&bigint0))
    ;

}

pub fn sqrt_price_and_tick_pool_entity_change(changes:&mut DatabaseChanges, 
    pool_sqrt_price_deltas:&Deltas<DeltaProto<PoolSqrtPrice>>){
        for delta in pool_sqrt_price_deltas.iter().key_first_segment_eq("pool"){
            let pool_address = key::segment_at(&delta.key, 1);
            save_uniswap_v3_sqrt_price_and_tick_pool(
                pool_address,
                &delta.new_value.sqrt_price,
                &delta.new_value.tick,
                changes
            );
        }

}

fn save_uniswap_v3_sqrt_price_and_tick_pool(
    pool_address:&str,
    sqrt_price:&String,
    tick:&String,
    changes:&mut DatabaseChanges
){
    let id = format!("0x{}",pool_address);
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
    .change("sqrt_price", (None,sqrt_price))
    .change("tick", (None,tick));
}


pub fn liquidities_pool_entity_change(changes:&mut DatabaseChanges, pool_liquidities_store_deltas:&Deltas<DeltaBigInt>){

    for delta in pool_liquidities_store_deltas.iter().key_first_segment_eq("pool") {
        let pool_address = key::segment_at(&delta.key, 1);
        save_uniswap_v3_iquidities_pool_entity_change(pool_address,&delta.new_value,changes);
    }

}

fn save_uniswap_v3_iquidities_pool_entity_change(pool_address:&str,liquidity:&BigInt,changes:&mut DatabaseChanges){
    let id = format!("0x{}",pool_address);
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
    .change("liquidity", (None,liquidity));
}

pub fn fee_growth_global_pool_entity_change(changes:&mut DatabaseChanges, updates :&Vec<events::FeeGrowthGlobal>){
    for update in updates {
        let pool_address =  &update.pool_address;
        let id = format!("0x{}",pool_address);
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), id);
        let change = changes.push_change_composite("ethereum_uniswap_v3_pools", keys, 1, Operation::Update);
        if update.token_idx == 0 {
            change.change("fee_growth_global_0x128", (None,&update.new_value));
        } else if update.token_idx == 1 {
            change.change("fee_growth_global_1x128", (None,&update.new_value));
        }
    } 
}

pub fn total_value_locked_pool_entity_change(changes:&mut DatabaseChanges, derived_tvl_deltas:&Deltas<DeltaBigDecimal>){
   
    for delta in derived_tvl_deltas
        .iter()
        .key_first_segment_eq("pool")
        .key_last_segment_in([
            "totalValueLockedUSD",
            "totalValueLockedETH",
            "totalValueLockedUSDUntracked",
            "totalValueLockedETHUntracked",
        ]){
            let pool_address = key::segment_at(&delta.key, 1);
            save_uniswap_v3_total_value_locked_pool_entity_change(pool_address,delta,changes);

    }
}

fn save_uniswap_v3_total_value_locked_pool_entity_change(pool_address:&str,delta:&DeltaBigDecimal,changes:&mut DatabaseChanges){
    let id = format!("0x{}",pool_address);
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    let change = changes.push_change_composite("ethereum_uniswap_v3_pools", keys, 1, Operation::Update);
    
    let key: &String = &delta.key;
    if "totalValueLockedUSD" == key {
        change.change("total_value_locked_usd", (None,delta.new_value.clone()));
    } else if "totalValueLockedETH" == key {
        change.change("total_value_locked_eth", (None,delta.new_value.clone()));
    } else if "totalValueLockedUSDUntracked" == key {
        change.change("total_value_locked_usd_untracked", (None,delta.new_value.clone()));
    } else if "totalValueLockedETHUntracked" == key {
        change.change("total_value_locked_eth_untracked", (None,delta.new_value.clone()));
    }

}

pub fn total_value_locked_by_token_pool_entity_change(changes:&mut DatabaseChanges, token_tvl_deltas:&Deltas<DeltaBigDecimal>){
    for delta in token_tvl_deltas.iter().key_first_segment_eq("pool") {
        let pool_address = key::segment_at(&delta.key, 1);
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), format!("0x{}",pool_address));
        let change = changes.push_change_composite("ethereum_uniswap_v3_pools", keys, 1, Operation::Update);

        if "token0" == delta.key {
            change.change("total_value_locked_token0", (None,delta.new_value.clone()));
        } else if "token1" == delta.key {
            change.change("total_value_locked_token1", (None,delta.new_value.clone()));
        }
    }
}


pub fn price_pool_entity_change(changes:&mut DatabaseChanges, price_deltas: &Deltas<DeltaBigDecimal>){
    for delta in price_deltas.iter().key_first_segment_eq("pool") {
        let pool_address = key::segment_at(&delta.key, 1);
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), format!("0x{}",pool_address));
        let change = changes.push_change_composite("ethereum_uniswap_v3_pools", keys, 1, Operation::Update);

        if "token0" == delta.key {
            change.change("token0_price", (None,delta.new_value.clone()));
        } else if "token1" == delta.key {
            change.change("token1_price", (None,delta.new_value.clone()));
        }

    }
}

pub fn tx_count_pool_entity_change(changes:&mut DatabaseChanges, tx_count_deltas : &Deltas<DeltaBigInt>) {

    for delta in tx_count_deltas.iter().key_first_segment_eq("pool") {
        let pool_address: &str = key::segment_at(&delta.key, 1);
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), format!("0x{}",pool_address));
        changes.push_change_composite("ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
        .change("tx_count", (None,&delta.new_value));
    }
}


pub fn swap_volume_pool_entity_change(changes:&mut DatabaseChanges, swaps_volume_deltas:&Deltas<DeltaBigDecimal>){
    for delta in swaps_volume_deltas.iter().key_first_segment_eq("pool") {
        let pool_address: &str = key::segment_at(&delta.key, 1);
            let mut keys: HashMap<String, String> = HashMap::new();
            keys.insert("id".to_string(), format!("0x{}",pool_address));
        let field_name = match key::last_segment(&delta.key) {
            "volumeToken0" => "volumeToken0",
            "volumeToken1" => "volumeToken1",
            "volumeUSD" => "volumeUSD",
            "volumeUntrackedUSD" => "untrackedVolumeUSD",
            "feesUSD" => "feesUSD",
            "liquidityProviderCount" => "liquidityProviderCount",
            _ => continue,
        };
        if field_name == "liquidityProviderCount" {
           
            changes.push_change_composite("ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
            .change("liquidity_provider_count", (None,&delta.new_value.to_bigint()));
            continue;
        } else if field_name == "volumeToken0" {
            changes.push_change_composite("ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
            .change("volume_token0", (None,&delta.new_value));
        } else if field_name == "volumeToken1" {
            changes.push_change_composite("ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
            .change("volume_token1", (None,&delta.new_value));
        } else if field_name == "volumeUSD" {
            changes.push_change_composite("ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
            .change("volume_usd", (None,&delta.new_value));
        } else if field_name == "untrackedVolumeUSD" {
            changes.push_change_composite("ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
            .change("untracked_volume_usd", (None,&delta.new_value));
        } else if field_name == "feesUSD" {
            changes.push_change_composite("ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
            .change("fees_usd", (None,&delta.new_value));
        }
    }
}


pub fn tokens_created_token_entity_changes(changes:&mut DatabaseChanges, pools:&Pools, tokens_store:StoreGetInt64) {
    for pool in &pools.pools {
        let ord = pool.log_ordinal;
        let pool_address = &pool.address;
        let token0_addr = pool.token0_ref().address();
        let token1_addr = pool.token1_ref().address();
        match tokens_store.get_at(ord, format!("token:{token0_addr}")) {
            Some(value) => {
                if value.eq(&1) {
                    add_token_entity_change(changes, pool.token0_ref());
                }
            }
            None => {
                panic!("pool contains token that doesn't exist {}", pool_address.as_str())
            }
        }

        match tokens_store.get_at(ord, format!("token:{token1_addr}")) {
            Some(value) => {
                if value.eq(&1) {
                   add_token_entity_change(changes, pool.token1_ref());
                }
            }
            None => {
                panic!("pool contains token that doesn't exist {}", pool_address.as_str())
            }
        }
    }
}


fn add_token_entity_change(changes:&mut DatabaseChanges, token:&Erc20Token) {
    let bigdecimal0 = BigDecimal::from(0);
    let bigint0 = BigInt::from(0);

    let token_addr = &token.address;
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), format!("0x{}",token_addr));
    changes.push_change_composite("ethereum_uniswap_v3_tokens", keys, 1, Operation::Create)
    .change("symbol", (None,&token.symbol))
    .change("token_name", (None,&token.name))
    .change("token_decimals", (None,token.decimals))
    .change("total_supply", (None,&token.total_supply))
    .change("volume", (None,&bigdecimal0))
    .change("volume_usd",(None,&bigdecimal0))
    .change("untracked_volume_usd", (None,&bigdecimal0))
    .change("fees_usd", (None,&bigdecimal0))
    .change("tx_count", (None,&bigint0))
    .change("pool_count", (None,&bigint0))
    .change("total_value_locked", (None,&bigdecimal0))
    .change("total_value_locked_usd", (None,&bigdecimal0))
    .change("total_value_locked_usd_untracked", (None,&bigdecimal0))
    .change("derived_eth", (None,&bigdecimal0));
}


pub fn swap_volume_token_entity_change(changes:&mut DatabaseChanges, swaps_volume_deltas:&Deltas<DeltaBigDecimal>){
    for delta in swaps_volume_deltas.iter().key_first_segment_eq("token") {
        let token_address = key::segment_at(&delta.key, 1);
        let field_name: &str = match key::last_segment(&delta.key) {
            "volume" => "volume",
            "usd" => "volumeUSD",
            "untrackedUSD" => "untrackedVolumeUSD",
            "feesUSD" => "feesUSD",
            _ => continue,
        };
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), format!("0x{}",token_address));
        if field_name == "volume" {
            changes.push_change_composite("ethereum_uniswap_v3_tokens", keys, 1, Operation::Update)
            .change("volume", (None,&delta.new_value));
        } else if field_name == "volumeUSD" {
            changes.push_change_composite("ethereum_uniswap_v3_tokens", keys, 1, Operation::Update)
            .change("volume_usd", (None,&delta.new_value));
        } else if field_name == "untrackedVolumeUSD" {
            changes.push_change_composite("ethereum_uniswap_v3_tokens", keys, 1, Operation::Update)
            .change("untracked_volume_usd", (None,&delta.new_value));
        } else if field_name == "feesUSD" {
            changes.push_change_composite("ethereum_uniswap_v3_tokens", keys, 1, Operation::Update)
            .change("fees_usd", (None,&delta.new_value));
        }
    }

}

pub fn tx_count_token_entity_change(changes:&mut DatabaseChanges, tx_count_deltas:&Deltas<DeltaBigInt>){
    for delta in tx_count_deltas.iter().key_first_segment_eq("token") {
        let token_address = key::segment_at(&delta.key, 1);
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), format!("0x{}",token_address));
        changes.push_change_composite("ethereum_uniswap_v3_tokens", keys, 1, Operation::Update)
        .change("tx_count", (None,&delta.new_value));
    }
}

pub fn total_value_locked_by_token_token_entity_change(changes:&mut DatabaseChanges, token_tvl_deltas:&Deltas<DeltaBigDecimal>){
    for delta in token_tvl_deltas.iter().key_first_segment_eq("token") {
        let token_address = key::last_segment(&delta.key);
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), format!("0x{}",token_address));
        changes.push_change_composite("ethereum_uniswap_v3_tokens", keys, 1, Operation::Update)
        .change("total_value_locked", (None,&delta.new_value));
    }
}

pub fn total_value_locked_usd_token_entity_change(changes:&mut DatabaseChanges, derived_tvl_deltas : &Deltas<DeltaBigDecimal>){
    for delta in derived_tvl_deltas.iter()
    .key_first_segment_eq("token")
    .key_last_segment_eq("totalValueLockedUSD") {
        let token_address = key::segment_at(&delta.key, 1);
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), format!("0x{}",token_address));
        changes.push_change_composite("ethereum_uniswap_v3_tokens", keys, 1, Operation::Update)
        .change("total_value_locked_usd", (None,&delta.new_value));
    }
}

pub fn derived_eth_prices_token_entity_change(changes:&mut DatabaseChanges, derived_eth_prices_deltas: &Deltas<DeltaBigDecimal>){
    for delta in derived_eth_prices_deltas.iter().key_first_segment_eq("token") {
        let field_name: &str = match key::last_segment(&delta.key) {
            "eth" => "derivedETH",
            _ => continue,
        };
        let token_address = key::segment_at(&delta.key, 1);
        if field_name == "derivedETH" {
            let mut keys: HashMap<String, String> = HashMap::new();
            keys.insert("id".to_string(), format!("0x{}",token_address));
            changes.push_change_composite("ethereum_uniswap_v3_tokens", keys, 1, Operation::Update)
            .change("derived_eth", (None,&delta.new_value));
        }
    }
}


pub fn whitelist_token_entity_change(changes:&mut DatabaseChanges, tokens_whitelist_pools_deltas: Deltas<DeltaArray<String>>) {
    for delta in tokens_whitelist_pools_deltas.into_iter() {
        let token_address = key::segment_at(&delta.key, 1);
        let whitelist: Vec<_> = delta.new_value.into_iter().map(|item| format!("0x{}", item)).collect();
        for (white_index,white) in whitelist.iter().enumerate(){
            let id = format!("{}#{}",token_address,white_index);
            let mut keys: HashMap<String, String> = HashMap::new();
            keys.insert("id".to_string(), id);

            changes.push_change_composite("ethereum_uniswap_v3_whitelist_token", keys, 1, Operation::Create)
            .change("token", (None,token_address))
            .change("white_pool", (None,white));
        }


    }
}


pub fn create_tick_entity_change(changes:&mut DatabaseChanges, ticks_created: &Vec<events::TickCreated>){
    let bigdecimal0 = BigDecimal::from(0);
    let bigint0 = BigInt::from(0);

    for tick in ticks_created {
        let pool_address = &tick.pool_address;
        let tick_idx = &tick.idx;
        let id = format!("0x{pool_address}#{tick_idx}");

        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), id);
        changes.push_change_composite("ethereum_uniswap_v3_ticks", keys, 1, Operation::Update)
        .change("pool_address", (None,pool_address))
        .change("tick_idx", (None,tick_idx))
        .change("pool", (None,&format!("0x{pool_address}")))
        .change("liquidity_gross", (None,&bigint0))
        .change("liquidity_net", (None,&bigint0))
        .change("price0", (None,&tick.price0))
        .change("price1", (None,&tick.price1))
        .change("volume_token0", (None,&bigdecimal0))
        .change("volume_token1", (None,&bigdecimal0))
        .change("volume_usd", (None,&bigdecimal0))
        .change("untracked_volume_usd", (None,&bigdecimal0))
        .change("fees_usd", (None,&bigdecimal0))
        .change("collected_fees_token0", (None,&bigdecimal0))
        .change("collected_fees_token1", (None,&bigdecimal0))
        .change("collected_fees_usd", (None,&bigdecimal0))
        .change("created_at_timestamp", (None,&bigint0))
        .change("created_at_block_number", (None,&bigint0))
        .change("liquidity_provider_count", (None,&bigint0))
        .change("fee_growth_outside_0x128", (None,&bigint0))
        .change("fee_growth_outside_1x128", (None,&bigint0));
    }
}

pub fn update_tick_entity_change(changes:&mut DatabaseChanges,  ticks_updated: &Vec<events::TickUpdated>){
    for tick in ticks_updated {
        let pool_address = &tick.pool_address;
        let tick_idx = &tick.idx;
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), format!("0x{pool_address}#{tick_idx}"));
        if tick.fee_growth_outside_0x_128.len() != 0 {
            changes.push_change_composite("ethereum_uniswap_v3_ticks", keys.clone(), 1, Operation::Update)
            .change("fee_growth_outside_0x128", (None, &tick.fee_growth_outside_0x_128));
        }
        if tick.fee_growth_outside_1x_128.len() != 0 {
            changes.push_change_composite("ethereum_uniswap_v3_ticks", keys, 1, Operation::Update)
            .change("fee_growth_outside_1x128", (None, &tick.fee_growth_outside_1x_128));
        }
    }
}

pub fn liquidities_tick_entity_change(changes:&mut DatabaseChanges, ticks_liquidities_deltas: &Deltas<DeltaBigInt>){
    for delta in ticks_liquidities_deltas.iter()
    .key_first_segment_eq("tick")
    .key_last_segment_in(["liquidityNet", "liquidityGross"]){
        let pool_id = key::segment_at(&delta.key, 1);
        let tick_idx = key::segment_at(&delta.key, 2);
        
        if &delta.key == "liquidityNet"{
            let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), format!("0x{pool_id}#{tick_idx}"));
            changes.push_change_composite("ethereum_uniswap_v3_ticks", keys, 1, Operation::Update)
            .change("liquidity_net", (None, &delta.new_value));
        }
        if &delta.key == "liquidityGross"{
            let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), format!("0x{pool_id}#{tick_idx}"));
            changes.push_change_composite("ethereum_uniswap_v3_ticks", keys, 1, Operation::Update)
            .change("liquidity_gross", (None, &delta.new_value));
        }

    }
}

pub fn position_create_entity_change(changes:&mut DatabaseChanges, positions: &Vec<events::CreatedPosition>){
    let bigdecimal0 = BigDecimal::from(0);
    for position in positions {
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), position.token_id.clone());
        changes.push_change_composite("ethereum_uniswap_v3_positions", keys, 1, Operation::Create)
        .change("owner", (None,&Hex(utils::ZERO_ADDRESS).to_string().into_bytes()))
        .change("pool", (None,format!("0x{}", &position.pool)))
        .change("token0", (None,format!("0x{}", position.token0)))
        .change("token1", (None,format!("0x{}", position.token1)))
        .change("tick_lower", (None,format!("0x{}#{}", &position.pool, &position.tick_lower)))
        .change("tick_upper", (None,format!("0x{}#{}", &position.pool, &position.tick_upper)))
        .change("liquidity", (None,&"0".to_string()))
        .change("deposited_token0", (None,&bigdecimal0))
        .change("deposited_token1", (None,&bigdecimal0))
        .change("withdrawn_token0", (None,&bigdecimal0))
        .change("withdrawn_token1", (None,&bigdecimal0))
        .change("collected_fees_token0", (None,&bigdecimal0))
        .change("collected_fees_token1", (None,&bigdecimal0))
        .change("transaction", (None,format!("0x{}", position.transaction)))
        .change("fee_growth_inside0_last_x128", (None,&position.fee_growth_inside0_last_x128.clone().unwrap_or("0".to_string())))
        .change("fee_growth_inside1_last_x128", (None,&position.fee_growth_inside1_last_x128.clone().unwrap_or("0".to_string())));
    }
}

pub fn increase_liquidity_position_entity_change(changes:&mut DatabaseChanges,  positions: &Vec<IncreaseLiquidityPosition>){
    for position in positions {
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), position.token_id.clone());
        let change = changes.push_change_composite("ethereum_uniswap_v3_positions", keys, 1, Operation::Update);
        change.change("liquidity", (None,BigInt::try_from(&position.liquidity).unwrap()))
        .change("deposited_token0", (None,&position.deposited_token0))
        .change("deposited_token1", (None,&position.deposited_token1));
        if let Some(fee_growth_inside0_last_x128) = &position.fee_growth_inside0_last_x128 {
            change.change("fee_growth_inside0_last_x128", (None,fee_growth_inside0_last_x128));
        }
        if let Some(fee_growth_inside1_last_x128) = &position.fee_growth_inside1_last_x128 {
            change.change("fee_growth_inside1_last_x128", (None,fee_growth_inside1_last_x128));
        }
    }
}


pub fn decrease_liquidity_position_entity_change(changes:&mut DatabaseChanges, positions: &Vec<DecreaseLiquidityPosition>){
    for position in positions {
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), position.token_id.clone());
        let change = changes.push_change_composite("ethereum_uniswap_v3_positions", keys, 1, Operation::Update);
        change.change("liquidity", (None,BigInt::try_from(&position.liquidity).unwrap()))
        .change("deposited_token0", (None,&position.withdrawn_token0))
        .change("deposited_token1", (None,&position.withdrawn_token1));
        if let Some(fee_growth_inside0_last_x128) = &position.fee_growth_inside0_last_x128 {
            change.change("fee_growth_inside0_last_x128", (None,fee_growth_inside0_last_x128));
        }
        if let Some(fee_growth_inside1_last_x128) = &position.fee_growth_inside1_last_x128 {
            change.change("fee_growth_inside1_last_x128", (None,fee_growth_inside1_last_x128));
        }
    }
}


pub fn collect_position_entity_change(changes:&mut DatabaseChanges, positions: &Vec<events::CollectPosition>){
    for position in positions {
        log::info!("collected_fees_token0 {}", position.collected_fees_token0);
        log::info!("collected_fees_token1 {}", position.collected_fees_token1);
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), position.token_id.clone());
        let change = changes.push_change_composite("ethereum_uniswap_v3_positions", keys, 1, Operation::Update);
        change.change("collected_fees_token0", (None,&position.collected_fees_token0))
        .change("collected_fees_token1", (None,&position.collected_fees_token1));
        if let Some(fee_growth_inside0_last_x128) = &position.fee_growth_inside0_last_x128{
            change.change("fee_growth_inside0_last_x128", (None,fee_growth_inside0_last_x128));
        }
        if let Some(fee_growth_inside1_last_x128) = &position.fee_growth_inside1_last_x128 {
            change.change("fee_growth_inside1_last_x128", (None,fee_growth_inside1_last_x128));
        }
    }
}

pub fn transfer_position_entity_change(changes:&mut DatabaseChanges, positions: &Vec<events::TransferPosition>){
    for position in positions {
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), position.token_id.clone());
        let change = changes.push_change_composite("ethereum_uniswap_v3_positions", keys, 1, Operation::Update);
        change.change("owner", (None,&hex::decode(&position.owner).unwrap()));
    }
}

// --------------------
//  Map Snapshot Position Entities
// --------------------

pub fn snapshot_positions_create_entity_change(changes:&mut DatabaseChanges, positions: &Vec<events::CreatedPosition>){
    for position in positions {
        let id = format!("{}#{}", position.token_id, position.block_number);
        create_snapshot_position(changes, &id, position);
    }
}

fn create_snapshot_position(changes : &mut DatabaseChanges, id: &String, position: &events::CreatedPosition){
    let mut keys: HashMap<String, String> = HashMap::new();
    
    keys.insert("id".to_string(), id.to_string());
    changes.push_change_composite("ethereum_uniswap_v3_position_snapshot", keys, 1, Operation::Create)
    .change("owner", (None,&utils::ZERO_ADDRESS.to_vec()))
    .change("pool", (None,format!("0x{}", &position.pool)))
    .change("position", (None,&position.token_id))
    .change("block_number", (None,position.block_number))
    .change("position_timestamp", (None,position.timestamp))
    .change("liquidity", (None,&"0".to_string()))
    .change("deposited_token0", (None,&"0".to_string()))
    .change("deposited_token1", (None,&"0".to_string()))
    .change("withdrawn_token0", (None,&"0".to_string()))
    .change("withdrawn_token1", (None,&"0".to_string()))
    .change("collected_fees_token0", (None,&"0".to_string()))
    .change("collected_fees_token1", (None,&"0".to_string()))
    .change("transaction", (None,&format!("0x{}", &position.transaction)))
    .change("fee_growth_inside0_last_x128", (None,&position.fee_growth_inside0_last_x128.clone().unwrap_or("0".to_string())))
    .change("fee_growth_inside1_last_x128", (None,&position.fee_growth_inside1_last_x128.clone().unwrap_or("0".to_string())));
      
}

pub fn increase_liquidity_snapshot_position_entity_change(
    changes : &mut DatabaseChanges,
    block_number: u64,
    positions: &Vec<IncreaseLiquidityPosition>,
    store_positions: &StoreGetProto<PositionEvent>,
){
    for (position_index,position )in positions.iter().enumerate() {
        let id = format!("{}#{}#{}", position.token_id, block_number,position_index);
        fetch_and_update_snapshot_position(changes, &position.token_id, &id, &store_positions);
        increase_liquidity_snapshot_position(changes, &id, &position);
    }
}

fn  fetch_and_update_snapshot_position(
    changes : &mut DatabaseChanges,
    token_id: &String,
    snapshot_id: &String,
    store_positions: &StoreGetProto<PositionEvent>){
        if let Some(position) = store_positions.get_last(format!("position_created:{}", token_id)) {
            match position.r#type.unwrap() {
                // Type::CreatedPosition(position) => create_snapshot_position(changes, snapshot_id, &position),
                _ => {}
            }
        }
    
        if let Some(position) = store_positions.get_last(format!("position_increase_liquidity:{}", token_id)) {
            match position.r#type.unwrap() {
                Type::IncreaseLiquidityPosition(position) => {
                    increase_liquidity_snapshot_position(changes, snapshot_id, &position)
                }
                _ => {}
            }
        }
    
        if let Some(position) = store_positions.get_last(format!("position_decrease_liquidity:{}", token_id)) {
            match position.r#type.unwrap() {
                Type::DecreaseLiquidityPosition(position) => {
                    decrease_liquidity_snapshot_position(changes, snapshot_id, &position)
                }
                _ => {}
            }
        }
    
        if let Some(position) = store_positions.get_last(format!("position_collect:{}", token_id)) {
            match position.r#type.unwrap() {
                Type::CollectPosition(position) => collection_snapshot_position(changes, snapshot_id, &position),
                _ => {}
            }
        }
    
        if let Some(position) = store_positions.get_last(format!("position_transfer:{}", token_id)) {
            match position.r#type.unwrap() {
                Type::TransferPosition(position) => transfer_snapshot_position(changes, snapshot_id, &position),
                _ => {}
            }
        }
}

fn decrease_liquidity_snapshot_position(
    changes:&mut DatabaseChanges, 
    id: &String,
    position: &events::DecreaseLiquidityPosition,
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id.to_string());
    let change = changes.push_change_composite("ethereum_uniswap_v3_position_snapshot", keys, 1, Operation::Update)
    .change("liquidity", (None,&position.liquidity))
    .change("withdrawn_token0", (None,&position.withdrawn_token0))
    .change("withdrawn_token1", (None,&position.withdrawn_token1));
    if let Some(fee_growth_inside0_last_x128) = &position.fee_growth_inside0_last_x128 {
        change.change("fee_growth_inside0_last_x128", (None,fee_growth_inside0_last_x128));
    }
    if let Some(fee_growth_inside1_last_x128) = &position.fee_growth_inside1_last_x128 {
        change.change("fee_growth_inside1_last_x128", (None,fee_growth_inside1_last_x128));
    }
}
fn collection_snapshot_position(changes:&mut DatabaseChanges, id: &String, position: &events::CollectPosition) {
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id.to_string());
    let change = changes.push_change_composite("ethereum_uniswap_v3_position_snapshot", keys, 1, Operation::Update)
    .change("collected_fees_token0", (None,&position.collected_fees_token0))
    .change("collected_fees_token1", (None,&position.collected_fees_token1));
    if let Some(fee_growth_inside0_last_x128) = &position.fee_growth_inside0_last_x128 {
        change.change("fee_growth_inside0_last_x128", (None,fee_growth_inside0_last_x128));
    }
    if let Some(fee_growth_inside1_last_x128) = &position.fee_growth_inside1_last_x128 {
        change.change("fee_growth_inside1_last_x128", (None,fee_growth_inside1_last_x128));
    }
}


fn  increase_liquidity_snapshot_position(
    changes : &mut DatabaseChanges, 
    id: &String, 
    position: &IncreaseLiquidityPosition){

        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), id.to_string());
        let change = changes.push_change_composite("ethereum_uniswap_v3_position_snapshot", keys, 1, Operation::Update)
        .change("deposited_token0", (None,&position.deposited_token0))
        .change("deposited_token1", (None,&position.deposited_token1));
        if let Some(fee_growth_inside0_last_x128) = &position.fee_growth_inside0_last_x128 {
            change.change("fee_growth_inside0_last_x128", (None,fee_growth_inside0_last_x128));
        }
        if let Some(fee_growth_inside1_last_x128) = &position.fee_growth_inside1_last_x128 {
            change.change("fee_growth_inside1_last_x128", (None,fee_growth_inside1_last_x128));
        }


}
fn transfer_snapshot_position(changes:&mut DatabaseChanges, id: &String, position: &events::TransferPosition) {

    let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), id.to_string());
        changes.push_change_composite("ethereum_uniswap_v3_position_snapshot", keys, 1, Operation::Update)
        .change("owner", (None,&hex::decode(&position.owner).unwrap()));
}

pub fn decrease_liquidity_snapshot_position_entity_change(
    changes : &mut DatabaseChanges, 
    block_number: u64,
    positions: &Vec<events::DecreaseLiquidityPosition>,
    store_positions: &StoreGetProto<PositionEvent>,
){
    for position in positions {
        let id = format!("{}#{}", position.token_id, block_number);
        fetch_and_update_snapshot_position(changes, &position.token_id, &id, &store_positions);
        decrease_liquidity_snapshot_position(changes, &id, &position)
    }
}

pub fn collect_snapshot_position_entity_change(
    changes : &mut DatabaseChanges, 
    block_number: u64,
    positions: &Vec<events::CollectPosition>,
    store_positions: &StoreGetProto<PositionEvent>,){
        for position in positions {
            let id = format!("{}#{}", position.token_id, block_number);
            fetch_and_update_snapshot_position(changes, &position.token_id, &id, &store_positions);
            collection_snapshot_position(changes, &id, &position);
        }

}

pub fn transfer_snapshot_position_entity_change(
    changes : &mut DatabaseChanges, 
    block_number: u64,
    positions: &Vec<events::TransferPosition>,
    store_positions: &StoreGetProto<PositionEvent>,
){
    for position in positions {
        let id = format!("{}#{}", position.token_id, block_number);
        fetch_and_update_snapshot_position(changes, &position.token_id, &id, &store_positions);
        transfer_snapshot_position(changes, &id, &position);
    }

}

pub fn transaction_entity_change(changes : &mut DatabaseChanges, transactions: &Vec<events::Transaction>){
    for (transaction_index ,transaction) in transactions.iter().enumerate() {
        let id = format!("0x{}#{}", transaction.id,transaction_index);
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), id.to_string());
        changes.push_change_composite("ethereum_uniswap_v3_transaction", keys, 1, Operation::Create)
        .change("block_number", (None,transaction.block_number))
        .change("transaction_timestamp", (None,transaction.timestamp))
        .change("gas_used", (None,transaction.gas_used));
        //.change("gas_price", (None,transaction.gas_price.clone()));
    }
}


pub fn swaps_mints_burns_created_entity_change(
    changes : &mut DatabaseChanges, 
    pool_events: &Vec<events::PoolEvent>,
    tx_count_store: StoreGetBigInt,
    store_eth_prices: StoreGetBigDecimal,){
        for(pool_event_index,pool_event)  in pool_events.iter().enumerate() {
            if pool_event.r#type.is_none() {
                continue;
            }
    
            let ord = pool_event.log_ordinal;
            let token0_addr = &pool_event.token0;
            let token1_addr = &pool_event.token1;
            if pool_event.r#type.is_some() {
                let pool_address = &pool_event.pool_address;
                let transaction_count: i32 = tx_count_store
                    .get_at(ord, format!("pool:{pool_address}"))
                    .unwrap_or_default()
                    .to_u64() as i32;

                let transaction_id = &pool_event.transaction_id;
                let event_primary_key: String = format!("0x{transaction_id}#{transaction_count}#{pool_event_index}");

                // initializePool has occurred beforehand so there should always be a price
                // maybe just ? instead of returning 1 and bubble up the error if there is one
                let token0_derived_eth_price = store_eth_prices
                    .get_at(ord, format!("token:{token0_addr}:dprice:eth"))
                    .unwrap_or_default();
                let token1_derived_eth_price = store_eth_prices
                    .get_at(ord, format!("token:{token1_addr}:dprice:eth"))
                    .unwrap_or_default();

                let bundle_eth_price = store_eth_prices.get_at(ord, "bundle").unwrap_or_default();
                match pool_event.r#type.as_ref().unwrap() {
                    SwapEvent(swap) => {
                        let amount0 = BigDecimal::try_from(swap.amount_0.as_str()).unwrap();
                        let amount1 = BigDecimal::try_from(swap.amount_1.as_str()).unwrap();

                        let amount0_abs = amount0.absolute();
                        let amount1_abs = amount1.absolute();

                        let amount_total_usd_tracked = utils::get_tracked_amount_usd(
                            &pool_event.token0,
                            &pool_event.token1,
                            &token0_derived_eth_price,
                            &token1_derived_eth_price,
                            &amount0_abs,
                            &amount1_abs,
                            &bundle_eth_price, // get the value from the store_eth_price
                        )
                        .div(BigDecimal::from(2 as i32));
                        
                        let id = format!("0x{}", &event_primary_key);
                        let mut keys: HashMap<String, String> = HashMap::new();
                        keys.insert("id".to_string(), id.to_string());
                        changes.push_change_composite("ethereum_uniswap_v3_swap", keys, 1, Operation::Create)
                        .change("transaction", (None,format!("0x{transaction_id}")))
                        .change("swap_timestamp", (None,pool_event.timestamp))
                        .change("pool", (None,format!("0x{pool_address}")))
                        .change("token0", (None,format!("0x{}", pool_event.token0)))
                        .change("token1", (None,format!("0x{}", pool_event.token1)))
                        .change("sender", (None,&hex::decode(&swap.sender).unwrap()))
                        .change("recipient", (None,&hex::decode(&swap.recipient).unwrap()))
                        .change("origin", (None,&hex::decode(&swap.origin).unwrap()))
                        .change("amount0", (None,&amount0))
                        .change("amount1", (None,&amount1))
                        .change("amount_usd", (None,&amount_total_usd_tracked))
                        .change("sqrt_price_x96", (None,&BigInt::try_from(swap.sqrt_price.to_string()).unwrap()))
                        .change("tick", (None,&BigInt::try_from(swap.tick.to_string()).unwrap()))
                        .change("log_index", (None,pool_event.log_index));
                    }
                    MintEvent(mint) => {
                        let amount0 = BigDecimal::try_from(mint.amount_0.as_str()).unwrap();
                        let amount1 = BigDecimal::try_from(mint.amount_1.as_str()).unwrap();

                        let amount_usd: BigDecimal = utils::calculate_amount_usd(
                            &amount0,
                            &amount1,
                            &token0_derived_eth_price,
                            &token1_derived_eth_price,
                            &bundle_eth_price,
                        );
                        let id = format!("0x{}", &event_primary_key);
                        let mut keys: HashMap<String, String> = HashMap::new();
                        keys.insert("id".to_string(), id.to_string());
                        changes.push_change_composite("ethereum_uniswap_v3_mint", keys, 1, Operation::Create)
                        .change("transaction", (None,format!("0x{transaction_id}")))
                        .change("mint_timestamp", (None,pool_event.timestamp))
                        .change("pool", (None,format!("0x{pool_address}")))
                        .change("token0", (None,format!("0x{}", pool_event.token0)))
                        .change("token1", (None,format!("0x{}", pool_event.token1)))
                        .change("owner", (None,&hex::decode(&mint.owner).unwrap()))
                        .change("sender", (None,&hex::decode(&mint.sender).unwrap()))
                        .change("origin", (None,&hex::decode(&mint.origin).unwrap()))
                        .change("amount_usd", (None,&amount_usd))
                        .change("tick_lower", (None,&mint.tick_lower))
                        .change("tick_upper", (None,&mint.tick_upper))
                        .change("log_index", (None,pool_event.log_index));
                    }
                    BurnEvent(burn) => {
                        let amount0: BigDecimal = BigDecimal::try_from(burn.amount_0.as_str()).unwrap();
                        let amount1: BigDecimal = BigDecimal::try_from(burn.amount_1.as_str()).unwrap();

                        let amount_usd: BigDecimal = utils::calculate_amount_usd(
                            &amount0,
                            &amount1,
                            &token0_derived_eth_price,
                            &token1_derived_eth_price,
                            &bundle_eth_price,
                        );

                        let id = format!("0x{}", &event_primary_key);
                        let mut keys: HashMap<String, String> = HashMap::new();
                        keys.insert("id".to_string(), id.to_string());
                        changes.push_change_composite("ethereum_uniswap_v3_burn", keys, 1, Operation::Create)
                        .change("transaction", (None,format!("0x{transaction_id}")))
                        .change("pool", (None,format!("0x{pool_address}")))
                        .change("token0", (None,format!("0x{}", pool_event.token0)))
                        .change("token1", (None,format!("0x{}", pool_event.token1)))
                        .change("burn_timestamp", (None,pool_event.timestamp))
                        .change("owner", (None,&hex::decode(&burn.owner).unwrap()))
                        .change("origin", (None,&hex::decode(&burn.origin).unwrap()))
                        .change("amount", (None,&burn.amount))
                        .change("amount0", (None,amount0))
                        .change("amount1", (None,amount1))
                        .change("amount_usd", (None,amount_usd))
                        .change("tick_lower", (None,&burn.tick_lower))
                        .change("tick_upper", (None,&burn.tick_upper));
                        
                    }
                }
            }

        }

}

pub fn uniswap_day_data_create(changes:&mut DatabaseChanges,  tx_count_deltas: &Deltas<DeltaBigInt>){
    uniswap_day_data_create_entity(changes, &tx_count_deltas);
}
pub fn uniswap_day_data_create_entity(changes:&mut DatabaseChanges,  tx_count_deltas: &Deltas<DeltaBigInt>) {
    for delta in tx_count_deltas
        .iter()
        .key_first_segment_eq("UniswapDayData")
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
    {
        if !delta.new_value.is_one() {
            continue;
        }

        let day_id = key::segment_at(&delta.key, 1).parse::<i64>().unwrap();
        let day_start_timestamp = (day_id * 86400) as i32;
        create_uniswap_day_data(changes, day_id, day_start_timestamp, &delta);
    }
}
fn create_uniswap_day_data(changes:&mut DatabaseChanges, day_id: i64, day_start_timestamp: i32, delta: &DeltaBigInt) {
    let bigdecimal0 = BigDecimal::zero();
    let id = day_id.to_string();
    let id = format!("{}", id);
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), id.to_string());
        changes.push_change_composite("ethereum_uniswap_v3_uniswap_day_data", keys, 1, Operation::Create)
        .change("day_start_timestamp", (None,day_start_timestamp))
        .change("volume_eth", (None,&bigdecimal0))
        .change("volume_usd", (None,&bigdecimal0))
        .change("volume_usd_untracked", (None,&bigdecimal0))
        .change("total_value_locked_usd", (None,&bigdecimal0))
        .change("fees_usd", (None,&bigdecimal0))
        .change("tx_count", (None,&delta.new_value));
}

pub fn uniswap_day_data_update(
    changes:&mut DatabaseChanges,
    swaps_volume_deltas: &Deltas<DeltaBigDecimal>,
    derived_factory_tvl_deltas: &Deltas<DeltaBigDecimal>,
    tx_count_deltas: &Deltas<DeltaBigInt>,
 ){
    tx_count_uniswap_day_data_update(changes, &tx_count_deltas);
    totals_uniswap_day_data_update(changes, &derived_factory_tvl_deltas);
    volumes_uniswap_day_data_update(changes, &swaps_volume_deltas);
 }

 pub fn tx_count_uniswap_day_data_update(    changes:&mut DatabaseChanges,tx_count_deltas: &Deltas<DeltaBigInt>) {
    for delta in tx_count_deltas
        .iter()
        .key_first_segment_eq("UniswapDayData")
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
    {
        let day_id = key::segment_at(&delta.key, 1);
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), day_id.to_string());
        changes.push_change_composite("ethereum_uniswap_v3_uniswap_day_data", keys, 1, Operation::Update)
        .change("tx_count", (None,&delta.new_value));
    }
}

pub fn totals_uniswap_day_data_update(changes:&mut DatabaseChanges, derived_factory_tvl_deltas: &Deltas<DeltaBigDecimal>) {
    for delta in derived_factory_tvl_deltas
        .iter()
        .key_first_segment_eq("UniswapDayData")
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
    {
        let day_id = key::segment_at(&delta.key, 1);
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), day_id.to_string());
        changes.push_change_composite("ethereum_uniswap_v3_uniswap_day_data", keys, 1, Operation::Update)
        .change("total_value_locked_usd", (None,&delta.new_value));
    }
}

pub fn volumes_uniswap_day_data_update(changes:&mut DatabaseChanges, swaps_volume_deltas: &Deltas<DeltaBigDecimal>) {
    for delta in swaps_volume_deltas
        .iter()
        .key_first_segment_eq("UniswapDayData")
        .key_last_segment_in(["volumeETH", "volumeUSD", "feesUSD"])
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
    {
        let day_id = key::segment_at(&delta.key, 1);
        if delta.key == "volumeETH"{
            let mut keys: HashMap<String, String> = HashMap::new();
            keys.insert("id".to_string(), day_id.to_string());
            let change = changes.push_change_composite("ethereum_uniswap_v3_uniswap_day_data", keys, 1, Operation::Update);
            change.change("volume_eth", (None,&delta.new_value));
        }
        if delta.key == "volumeUSD"{
            let mut keys: HashMap<String, String> = HashMap::new();
            keys.insert("id".to_string(), day_id.to_string());
            let change = changes.push_change_composite("ethereum_uniswap_v3_uniswap_day_data", keys, 1, Operation::Update);
            change.change("volume_usd", (None,&delta.new_value));
        }
        if delta.key == "feesUSD" {
            let mut keys: HashMap<String, String> = HashMap::new();
            keys.insert("id".to_string(), day_id.to_string());
            let change = changes.push_change_composite("ethereum_uniswap_v3_uniswap_day_data", keys, 1, Operation::Update);
            change.change("fees_usd", (None,&delta.new_value));
        }
    }
}



pub fn pool_windows_create(changes:&mut DatabaseChanges,  tx_count_deltas: &Deltas<DeltaBigInt>){
    upsert_entity_change_pool_windows(changes, tx_count_deltas);
}

pub fn upsert_entity_change_pool_windows(changes:&mut DatabaseChanges,  tx_count_deltas: &Deltas<DeltaBigInt>) {
    for delta in tx_count_deltas
        .iter()
        .key_first_segment_in(["PoolDayData", "PoolHourData"])
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
        .filter(|d| d.new_value.eq(&BigInt::one()))
    {
        let time_id = key::segment_at(&delta.key, 1).parse::<i64>().unwrap();
        let pool_address = key::segment_at(&delta.key, 2);

        let pool_time_id = format!("0x{pool_address}-{time_id}");
        create_pool_windows_entity(
            changes,
            key::first_segment(&delta.key),
            time_id,
            &pool_time_id,
            pool_address,
        );
    }
}

fn create_pool_windows_entity(
    changes:&mut DatabaseChanges,
    table_name: &str,
    time_id: i64,
    pool_time_id: &String,
    pool_addr: &str,
) {
    // todo 
    let id: String = format!("{}", pool_time_id);
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id.to_string());
    
    match table_name {
        "PoolDayData" => {
            changes.push_change_composite("ethereum_uniswap_v3_pool_day_data", keys, 1, Operation::Create)
            .change("time_id", (None,time_id))
            .change("pool_time_id", (None,pool_time_id))
            .change("pool", (None,format!("0x{}", pool_addr)))
            .change("liquidity", (None,BigInt::zero()))
            .change("sqrt_price", (None,BigInt::zero()))
            .change("token0_price", (None,BigInt::zero()))
            .change("token1_price", (None,BigDecimal::zero()))
            .change("tick", (None,BigInt::zero()))
            .change("fee_growth_global_0x128", (None,BigInt::zero()))
            .change("fee_growth_global_1x128", (None,BigInt::zero()))
            .change("total_value_locked_usd", (None,BigDecimal::zero()))
            .change("volume_token0", (None,BigDecimal::zero()))
            .change("volume_token1", (None,BigDecimal::zero()))
            .change("volume_usd", (None,BigDecimal::zero()))
            .change("fees_usd", (None,BigDecimal::zero()))
            .change("tx_count", (None,BigInt::zero()))
            .change("open", (None,BigDecimal::zero()))
            .change("high", (None,BigDecimal::zero()))
            .change("low", (None,BigDecimal::zero()))
            .change("close", (None,BigDecimal::zero()))
            .change("per_day", (None,(time_id * 86400) as i32));
        }
        "PoolHourData" => {
            changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Create)
            .change("time_id", (None,time_id))
            .change("pool_time_id", (None,pool_time_id))
            .change("pool", (None,format!("0x{}", pool_addr)))
            .change("liquidity", (None,BigInt::zero()))
            .change("sqrt_price", (None,BigInt::zero()))
            .change("token0_price", (None,BigInt::zero()))
            .change("token1_price", (None,BigDecimal::zero()))
            .change("tick", (None,BigInt::zero()))
            .change("fee_growth_global_0x128", (None,BigInt::zero()))
            .change("fee_growth_global_1x128", (None,BigInt::zero()))
            .change("total_value_locked_usd", (None,BigDecimal::zero()))
            .change("volume_token0", (None,BigDecimal::zero()))
            .change("volume_token1", (None,BigDecimal::zero()))
            .change("volume_usd", (None,BigDecimal::zero()))
            .change("fees_usd", (None,BigDecimal::zero()))
            .change("tx_count", (None,BigInt::zero()))
            .change("open", (None,BigDecimal::zero()))
            .change("high", (None,BigDecimal::zero()))
            .change("low", (None,BigDecimal::zero()))
            .change("close", (None,BigDecimal::zero()))
            .change("period_start", (None,(time_id * 3600) as i32));
        }
        _ => {}
    }

    

}



pub fn pool_windows_update(
    changes:&mut DatabaseChanges,
    timestamp: i64,
    tx_count_deltas: &Deltas<DeltaBigInt>,
    swaps_volume_deltas: &Deltas<DeltaBigDecimal>,
    events: &Events,
    pool_sqrt_price_store: &StoreGetProto<PoolSqrtPrice>,
    pool_liquidities_store_deltas: &Deltas<DeltaBigInt>,
    price_deltas: &Deltas<DeltaBigDecimal>,
    store_prices: &StoreGetBigDecimal,
    derived_tvl_deltas: &Deltas<DeltaBigDecimal>,
    min_windows_deltas: &Deltas<DeltaBigDecimal>,
    max_windows_deltas: &Deltas<DeltaBigDecimal>,
){
    tx_count_pool_windows(changes, &tx_count_deltas);
    mint_burn_prices_pool_windows(changes, timestamp, &events.pool_events, &store_prices);
    prices_pool_windows(changes, &price_deltas);
    prices_min_pool_windows(changes, &min_windows_deltas);
    prices_max_pool_windows(changes, &max_windows_deltas);
    prices_close_pool_windows(changes, &price_deltas);
    liquidities_and_sqrt_tick_pool_windows(changes, &pool_liquidities_store_deltas, &pool_sqrt_price_store);
    sqrt_price_and_tick_pool_windows(changes, timestamp, &pool_sqrt_price_store, &events.pool_events);
    swap_volume_pool_windows(changes, &swaps_volume_deltas);
    fee_growth_global_x128_pool_windows(changes, timestamp, &events.fee_growth_global_updates);
    total_value_locked_usd_pool_windows(changes, &derived_tvl_deltas);
}

pub fn tx_count_pool_windows( changes:&mut DatabaseChanges, tx_count_deltas: &Deltas<DeltaBigInt>) {
    for delta in tx_count_deltas
        .iter()
        .key_first_segment_in(["PoolDayData", "PoolHourData"])
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
    {
        let (table_name, time_id, pool_address) = pool_windows_id_fields(&delta.key);

        // tables
        //     .update_row(table_name, format!("0x{pool_address}-{time_id}"))
        //     .set("txCount", &delta.new_value);

        match table_name {
            "PoolDayData" => {
                let id: String = format!("0x{pool_address}-{time_id}");
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), id.to_string());

                changes.push_change_composite("ethereum_uniswap_v3_pool_day_data", keys, 1, Operation::Update)
                .change("tx_count", (None,&delta.new_value));

            }
            "PoolHourData" => {
                let id: String = format!("0x{pool_address}-{time_id}");
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), id.to_string());

                changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update)
                .change("tx_count", (None,&delta.new_value));
            }
            _ => {}
        }
    }
}


fn  mint_burn_prices_pool_windows( 
    changes:&mut DatabaseChanges, 
    timestamp: i64,
    pool_events: &Vec<events::PoolEvent>,
    store_prices: &StoreGetBigDecimal,){
        for pool_event in pool_events {
            if pool_event.r#type.is_none() {
                continue;
            }
    
            let day_id = timestamp / 86400;
            let hour_id = timestamp / 3600;
    
            if pool_event.r#type.is_some() {
                let token0_address = &pool_event.token0;
                let token1_address = &pool_event.token1;
                let pool_address = &pool_event.pool_address;
                let pool_day_id = format!("0x{pool_address}-{day_id}");
                let pool_hour_id = format!("0x{pool_address}-{hour_id}");

                let mut token0_price = BigDecimal::zero();
                let mut token1_price = BigDecimal::zero();
                match store_prices.get_last(format!("pool:{pool_address}:{token0_address}:token0")) {
                    None => {} // do nothing
                    Some(val) => {
                        token0_price = val;
                    }
                }

                match store_prices.get_last(format!("pool:{pool_address}:{token1_address}:token1")) {
                    None => {} // do nothing
                    Some(val) => {
                        token1_price = val;
                    }
                }

                match pool_event.r#type.as_ref().unwrap() {
                    events::pool_event::Type::Swap(_) => {
                        continue; // the swap event will be taken care of by the prices_pool_windows
                    }
                    _ => {}
                }

                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), pool_day_id);
                changes.push_change_composite("ethereum_uniswap_v3_pool_day_data", keys, 1, Operation::Update)
                .change("open", (None,&token0_price))
                .change("close", (None,&token0_price))
                .change("high", (None,&token0_price))
                .change("low", (None,&token0_price))
                .change("token1_price", (None,&token1_price))
                .change("token0_price", (None,&token0_price));


                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), pool_hour_id);
                changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update)
                .change("open", (None,&token0_price))
                .change("close", (None,&token0_price))
                .change("high", (None,&token0_price))
                .change("low", (None,&token0_price))
                .change("token1_price", (None,&token1_price))
                .change("token0_price", (None,&token0_price));
            }
        }

}

pub fn prices_pool_windows(changes:&mut DatabaseChanges, price_deltas: &Deltas<DeltaBigDecimal>) {
    for delta in price_deltas
        .iter()
        .key_first_segment_in(["PoolDayData", "PoolHourData"])
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
    {
        let (table_name, time_id, pool_address) = pool_windows_id_fields(&delta.key);
        let field_name = match key::last_segment(&delta.key) {
            "token0" => "token0Price",
            "token1" => "token1Price",
            _ => continue,
        };

        let pool_hour_id = format!("0x{pool_address}-{time_id}");
        match table_name {
            "PoolDayData" => {
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), pool_hour_id);
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_day_data", keys, 1, Operation::Update);
                match field_name {
                    "token0Price" => {
                        change.change("token0_price", (None,&delta.new_value));
                    }
                    "token1Price" => {
                        change.change("token1_price", (None,&delta.new_value));
                    }
                    _ => {}
                }
                
            }
            "PoolHourData" => {
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), pool_hour_id);
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update);
                match field_name {
                    "token0Price" => {
                        change.change("token0_price", (None,&delta.new_value));
                    }
                    "token1Price" => {
                        change.change("token1_price", (None,&delta.new_value));
                    }
                    _ => {}
                }

            }
            _ => {}
        }
    }
}

pub fn prices_min_pool_windows(changes:&mut DatabaseChanges,  min_pool_prices_deltas: &Deltas<DeltaBigDecimal>) {
    for delta in min_pool_prices_deltas
        .iter()
        .key_first_segment_in(["PoolDayData", "PoolHourData"])
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
        .key_last_segment_in(["low", "open"])
    {
        let (table_name, time_id, pool_address) = pool_windows_id_fields(&delta.key);
        let pool_time_id = format!("0x{pool_address}-{time_id}");

        match table_name {
            "PoolDayData" => {
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), pool_time_id);
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_day_data", keys, 1, Operation::Update);
                change.change(key::last_segment(&delta.key), (None,&delta.new_value));
            }
            "PoolHourData" => {
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), pool_time_id);
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update);
                change.change(key::last_segment(&delta.key), (None,&delta.new_value));
            }
            _ => {}
        }
    }
}
pub fn prices_max_pool_windows(changes:&mut DatabaseChanges, max_pool_prices_deltas: &Deltas<DeltaBigDecimal>) {
    for delta in max_pool_prices_deltas
        .iter()
        .key_first_segment_in(["PoolDayData", "PoolHourData"])
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
    {
        let (table_name, time_id, pool_address) = pool_windows_id_fields(&delta.key);
        let pool_time_id = format!("0x{pool_address}-{time_id}");
        match table_name {
            "PoolDayData" => {
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), pool_time_id);
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_day_data", keys, 1, Operation::Update);
                change.change("high", (None,&delta.new_value));
            }
            "PoolHourData" => {
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), pool_time_id);
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update);
                change.change("high", (None,&delta.new_value));
            }
            _ => {}
        }
    }
}


pub fn prices_close_pool_windows(changes:&mut DatabaseChanges, prices_deltas: &Deltas<DeltaBigDecimal>) {
    for delta in prices_deltas
        .iter()
        .key_first_segment_in(["PoolDayData", "PoolHourData"])
        .operation_eq(substreams::pb::substreams::store_delta::Operation::Delete)
    {
        let (table_name, time_id, pool_address) = pool_windows_id_fields(&delta.key);
        let pool_time_id = format!("0x{pool_address}-{time_id}");
        match table_name {
            "PoolDayData" => {
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), pool_time_id);
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_day_data", keys, 1, Operation::Update);
                change.change("close", (None,&delta.new_value));
            }
            "PoolHourData" => {
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), pool_time_id);
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update);
                change.change("close", (None,&delta.new_value));
            }
            _ => {}
        }
    }
}

pub fn liquidities_and_sqrt_tick_pool_windows(
    changes:&mut DatabaseChanges, 
    pool_liquidities_store_deltas: &Deltas<DeltaBigInt>,
    pool_sqrt_price_store: &StoreGetProto<PoolSqrtPrice>,
) {
    for delta in pool_liquidities_store_deltas
        .iter()
        .key_first_segment_in(["PoolDayData", "PoolHourData"])
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
    {
        let (table_name, time_id, pool_address) = pool_windows_id_fields(&delta.key);

        match table_name {
            "PoolDayData" => {
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), format!("0x{pool_address}-{time_id}"));
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_day_data", keys, 1, Operation::Update);
                change.change("liquidity", (None,&delta.new_value));
                match pool_sqrt_price_store.get_last(format!("pool:{pool_address}")) {
                    None => {
                        log::info!("This is not normal, or do we have some use cases where this will be ok??")
                    }
                    Some(price) => {
                        change.change("sqrt_price", (None,BigInt::try_from(&price.sqrt_price).unwrap()))
                        .change("tick", (None,BigInt::try_from(&price.tick).unwrap()));
                    }
                }
            }
            "PoolHourData" => {
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), format!("0x{pool_address}-{time_id}"));
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update);
                change.change("liquidity", (None,&delta.new_value));
                match pool_sqrt_price_store.get_last(format!("pool:{pool_address}")) {
                    None => {
                        log::info!("This is not normal, or do we have some use cases where this will be ok??")
                    }
                    Some(price) => {
                        change.change("sqrt_price", (None,BigInt::try_from(&price.sqrt_price).unwrap()))
                        .change("tick", (None,BigInt::try_from(&price.tick).unwrap()));
                    }
                }
            }
            _ => {}
        }
    }
}


pub fn sqrt_price_and_tick_pool_windows(
    changes:&mut DatabaseChanges, 
    timestamp: i64,
    pool_sqrt_price_store: &StoreGetProto<PoolSqrtPrice>,
    pool_events: &Vec<events::PoolEvent>,
) {
    let day_id = timestamp / 86400;
    let hour_id = timestamp / 3600;

    for pool_event in pool_events {
        let pool_address = &pool_event.pool_address;

        match pool_sqrt_price_store.get_last(format!("pool:{pool_address}")) {
            None => continue,
            Some(pool_sqrt_price) => {
                let sqrt_price = BigInt::try_from(pool_sqrt_price.sqrt_price).unwrap();
                let tick = BigInt::try_from(pool_sqrt_price.tick).unwrap();
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), format!("0x{pool_address}-{day_id}"));
                changes.push_change_composite("ethereum_uniswap_v3_pool_day_data", keys, 1, Operation::Update)
                .change("sqrt_price", (None,&sqrt_price))
                .change("tick", (None,&tick));

                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), format!("0x{pool_address}-{hour_id}"));
                changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update)
                .change("sqrt_price", (None,&sqrt_price))
                .change("tick", (None,&tick));
                
            }
        }
    }
}

pub fn swap_volume_pool_windows( changes:&mut DatabaseChanges,  swaps_volume_deltas: &Deltas<DeltaBigDecimal>) {
    for delta in swaps_volume_deltas
        .iter()
        .key_first_segment_in(["PoolDayData", "PoolHourData"])
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
        .key_last_segment_in(["volumeToken0", "volumeToken1", "volumeUSD", "feesUSD"])
    {
        let (table_name, time_id, pool_address) = pool_windows_id_fields(&delta.key);
        match table_name {
            "PoolDayData" => {
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), format!("0x{pool_address}-{time_id}"));
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_day_data", keys, 1, Operation::Update);
                match key::last_segment(&delta.key) {
                    "volumeToken0" => {
                        change.change("volume_token0", (None,&delta.new_value));
                    }
                    "volumeToken1" => {
                        change.change("volume_token1", (None,&delta.new_value));
                    }
                    "volumeUSD" => {
                        change.change("volume_usd", (None,&delta.new_value));
                    }
                    "feesUSD" => {
                        change.change("fees_usd", (None,&delta.new_value));
                    }
                     _ => {}
                }
            }
            "PoolHourData" => {
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), format!("0x{pool_address}-{time_id}"));
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update);
                match key::last_segment(&delta.key) {
                    "volumeToken0" => {
                        change.change("volume_token0", (None,&delta.new_value));
                    }
                    "volumeToken1" => {
                        change.change("volume_token1", (None,&delta.new_value));
                    }
                    "volumeUSD" => {
                        change.change("volume_usd", (None,&delta.new_value));
                    }
                    "feesUSD" => {
                        change.change("fees_usd", (None,&delta.new_value));
                    }
                     _ => {}
                }
            }
            _ => {}
        }
    }
}

pub fn fee_growth_global_x128_pool_windows(
    changes:&mut DatabaseChanges, 
    timestamp: i64,
    updates: &Vec<events::FeeGrowthGlobal>,
) {
    for update in updates {
        let day_id = timestamp / 86400;
        let hour_id = timestamp / 3600;

        let pool_address = &update.pool_address;

        if update.token_idx == 0 {
            let mut keys: HashMap<String, String> = HashMap::new();
            keys.insert("id".to_string(), format!("0x{pool_address}-{day_id}"));
            let change = changes.push_change_composite("ethereum_uniswap_v3_pool_day_data", keys, 1, Operation::Update);
            change.change("fee_growth_global_0x128", (None,&BigInt::try_from(&update.new_value).unwrap()));

            let mut keys: HashMap<String, String> = HashMap::new();
            keys.insert("id".to_string(), format!("0x{pool_address}-{hour_id}"));
            let change = changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update);
            change.change("fee_growth_global_0x128", (None,&BigInt::try_from(&update.new_value).unwrap()));


        } else if update.token_idx == 1 {
            let mut keys: HashMap<String, String> = HashMap::new();
            keys.insert("id".to_string(), format!("0x{pool_address}-{day_id}"));
            let change = changes.push_change_composite("ethereum_uniswap_v3_pool_day_data", keys, 1, Operation::Update);
            change.change("fee_growth_global_1x128", (None,&BigInt::try_from(&update.new_value).unwrap()));

            let mut keys: HashMap<String, String> = HashMap::new();
            keys.insert("id".to_string(), format!("0x{pool_address}-{hour_id}"));
            let change = changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update);
            change.change("fee_growth_global_1x128", (None,&BigInt::try_from(&update.new_value).unwrap()));
        }
    }
}

pub fn total_value_locked_usd_pool_windows(changes:&mut DatabaseChanges, derived_tvl_deltas: &Deltas<DeltaBigDecimal>) {
    for delta in derived_tvl_deltas
        .iter()
        .key_first_segment_in(["PoolDayData", "PoolHourData"])
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
        .key_last_segment_eq("totalValueLockedUSD")
    {
        let (table_name, time_id, pool_address) = pool_windows_id_fields(&delta.key);
        match table_name {
            "PoolDayData" => {
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), format!("0x{pool_address}-{time_id}"));
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_day_data", keys, 1, Operation::Update);
                change.change("total_value_locked_usd", (None,&delta.new_value));
            }
            "PoolHourData" => {
                let mut keys: HashMap<String, String> = HashMap::new();
                keys.insert("id".to_string(), format!("0x{pool_address}-{time_id}"));
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update);
                change.change("total_value_locked_usd", (None,&delta.new_value));
            }
            _ => {}
        }
    }
}

pub fn token_windows_create(changes:&mut DatabaseChanges, tx_count_deltas: &Deltas<DeltaBigInt>) {
    create_token_windows(changes, &tx_count_deltas);
}

pub fn create_token_windows(changes:&mut DatabaseChanges, tx_count_deltas: &Deltas<DeltaBigInt>) {
    for delta in tx_count_deltas
        .iter()
        .key_first_segment_in(["TokenDayData", "TokenHourData"])
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
        .filter(|d| d.new_value.eq(&BigInt::one()))
    {
        let (time_id, token_address) = time_as_i64_address_as_str(&delta.key);

        let token_time_id = format!("0x{token_address}-{time_id}");
        create_token_windows_entity(
            changes,
            key::first_segment(&delta.key),
            time_id,
            &token_time_id,
            token_address,
        );
    }
}

fn create_token_windows_entity(
    changes:&mut DatabaseChanges, 
    table_name: &str,
    time_id: i64,
    token_day_time_id: &String,
    token_addr: &str,
) {
    match table_name {
        "TokenDayData" => {
            let mut keys: HashMap<String, String> = HashMap::new();
            keys.insert("id".to_string(), token_day_time_id.to_string());
            let change = changes.push_change_composite("ethereum_uniswap_v3_token_day_data", keys, 1, Operation::Create);
            change.change("token", (None,format!("0x{}", token_addr)))
            .change("volume", (None,BigDecimal::zero()))
            .change("volume_usd", (None,BigDecimal::zero()))
            .change("volume_usd_untracked", (None,BigDecimal::zero()))
            .change("total_value_locked", (None,BigDecimal::zero()))
            .change("total_value_locked_usd", (None,BigDecimal::zero()))
            .change("price_usd", (None,BigDecimal::zero()))
            .change("fees_usd", (None,BigDecimal::zero()))
            .change("open", (None,BigDecimal::zero()))
            .change("high", (None,BigDecimal::zero()))
            .change("low", (None,BigDecimal::zero()))
            .change("close", (None,BigDecimal::zero()))
            .change("per_date", (None,(time_id * 86400) as i32));

        }
        "TokenHourData" => {
            let mut keys: HashMap<String, String> = HashMap::new();
            keys.insert("id".to_string(), token_day_time_id.to_string());
            let change = changes.push_change_composite("ethereum_uniswap_v3_token_hour_data", keys, 1, Operation::Create);
            change.change("token", (None,format!("0x{}", token_addr)))
            .change("volume", (None,BigDecimal::zero()))
            .change("volume_usd", (None,BigDecimal::zero()))
            .change("volume_usd_untracked", (None,BigDecimal::zero()))
            .change("total_value_locked", (None,BigDecimal::zero()))
            .change("total_value_locked_usd", (None,BigDecimal::zero()))
            .change("price_usd", (None,BigDecimal::zero()))
            .change("fees_usd", (None,BigDecimal::zero()))
            .change("open", (None,BigDecimal::zero()))
            .change("high", (None,BigDecimal::zero()))
            .change("low", (None,BigDecimal::zero()))
            .change("close", (None,BigDecimal::zero()))
            .change("period_start", (None,(time_id * 3600) as i32));
        }
        _ => {}
    }
}



pub fn token_windows_update(
    changes:&mut DatabaseChanges,
    timestamp: i64,
    swaps_volume_deltas: &Deltas<DeltaBigDecimal>,
    derived_tvl_deltas: &Deltas<DeltaBigDecimal>,
    min_windows_deltas: &Deltas<DeltaBigDecimal>,
    max_windows_deltas: &Deltas<DeltaBigDecimal>,
    derived_eth_prices_deltas: &Deltas<DeltaBigDecimal>,
    token_tvl_deltas: &Deltas<DeltaBigDecimal>,
) {
    swap_volume_token_windows(changes, &swaps_volume_deltas);
    total_value_locked_usd_token_windows(changes, &derived_tvl_deltas);
    total_value_locked_token_windows(changes, timestamp, &token_tvl_deltas);
    total_prices_token_windows(changes, &derived_eth_prices_deltas);
    prices_min_token_windows(changes, &min_windows_deltas);
    prices_max_token_windows(changes, &max_windows_deltas);
    prices_close_token_windows(changes, &derived_eth_prices_deltas);
}

pub fn swap_volume_token_windows(changes:&mut DatabaseChanges, swaps_volume_deltas: &Deltas<DeltaBigDecimal>) {
    for delta in swaps_volume_deltas
        .iter()
        .key_first_segment_in(["TokenDayData", "TokenHourData"])
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
    {
        let (table_name, time_id, token_address) = pool_windows_id_fields(&delta.key);

        let field_name = match key::last_segment(&delta.key) {
            "volume" => "volume",
            "volumeUSD" => "volumeUSD",
            "feesUSD" => "feesUSD",
            "untrackedUSD" => "volumeUSDUntracked",
            _ => continue,
        };

        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), format!("0x{token_address}-{time_id}"));
       
        match table_name {
            "TokenDayData" => {
                let change = changes.push_change_composite("ethereum_uniswap_v3_token_day_data", keys, 1, Operation::Update);
                match field_name {
                    "volume" => {
                        change.change("volume", (None,&delta.new_value));
                    }
                    "volumeUSD" => {
                        change.change("volume_usd", (None,&delta.new_value));
                    }
                    "feesUSD" => {
                        change.change("fees_usd", (None,&delta.new_value));
                    }
                    "volumeUSDUntracked" => {
                        change.change("volume_usd_untracked", (None,&delta.new_value));
                    }
                    _ => {}
                    
                }
            }
            "TokenHourData" => {
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update);
                match field_name {
                    "volume" => {
                        change.change("volume", (None,&delta.new_value));
                    }
                    "volumeUSD" => {
                        change.change("volume_usd", (None,&delta.new_value));
                    }
                    "feesUSD" => {
                        change.change("fees_usd", (None,&delta.new_value));
                    }
                    "volumeUSDUntracked" => {
                        change.change("volume_usd_untracked", (None,&delta.new_value));
                    }
                    _ => {}
                    
                }
            }
            _ => {}
        }


    }
}

pub fn total_value_locked_usd_token_windows(changes:&mut DatabaseChanges, derived_tvl_deltas: &Deltas<DeltaBigDecimal>) {
    for delta in derived_tvl_deltas
        .iter()
        .key_first_segment_in(["TokenDayData", "TokenHourData"])
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
    {
        let (table_name, time_id, token_address) = pool_windows_id_fields(&delta.key);

        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), format!("0x{token_address}-{time_id}"));
        match table_name {
            "TokenDayData" => {
                let change = changes.push_change_composite("ethereum_uniswap_v3_token_day_data", keys, 1, Operation::Update);
                change.change("total_value_locked_usd", (None,&delta.new_value));
            }
            "TokenHourData" => {
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update);
                change.change("total_value_locked_usd", (None,&delta.new_value));
            }
            _ => {}
        }
    }
}


pub fn total_value_locked_token_windows(
    changes:&mut DatabaseChanges,
    timestamp: i64,
    token_tvl_deltas: &Deltas<DeltaBigDecimal>,
) {
    let day_id = timestamp / 86400;
    let hour_id = timestamp / 3600;

    for delta in token_tvl_deltas
        .iter()
        .key_first_segment_eq("token")
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
    {
        let token_address = key::segment_at(&delta.key, 1);
        total_value_locked_token_windows_update(
            changes,
            "TokenDayData",
            format!("0x{token_address}-{day_id}"),
            &delta.new_value,
        );
        total_value_locked_token_windows_update(
            changes,
            "TokenHourData",
            format!("0x{token_address}-{hour_id}"),
            &delta.new_value,
        );
    }
}

fn total_value_locked_token_windows_update(
    changes:&mut DatabaseChanges,
    table_name: &str,
    token_time_id: String,
    value: &BigDecimal,
) {
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), token_time_id.to_string());
    match table_name {
        "TokenDayData" => {
            let change = changes.push_change_composite("ethereum_uniswap_v3_token_day_data", keys, 1, Operation::Update);
            change.change("total_value_locked", (None,value));
        }
        "TokenHourData" => {
            let change = changes.push_change_composite("ethereum_uniswap_v3_token_day_data", keys, 1, Operation::Update);
            change.change("total_value_locked", (None,value));

        }
        _ => {}
    }
    
}


pub fn total_prices_token_windows(changes:&mut DatabaseChanges, derived_eth_prices_deltas: &Deltas<DeltaBigDecimal>) {
    for delta in derived_eth_prices_deltas
        .iter()
        .key_first_segment_in(["TokenDayData", "TokenHourData"])
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
    {
        let (table_name, time_id, token_address) = token_windows_id_fields(&delta.key);
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), format!("0x{token_address}-{time_id}"));
        match table_name {
            "TokenDayData" => {
                let change = changes.push_change_composite("ethereum_uniswap_v3_token_day_data", keys, 1, Operation::Update);
                change.change("price_usd", (None,&delta.new_value));
            }
            "TokenHourData" => {
                let change = changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update);
                change.change("price_usd", (None,&delta.new_value));

            }
            _ => {}
        }
    }
}

pub fn prices_min_token_windows(changes:&mut DatabaseChanges,  min_token_prices_deltas: &Deltas<DeltaBigDecimal>) {
    for delta in min_token_prices_deltas
        .iter()
        .key_first_segment_in(["TokenDayData", "TokenHourData"])
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
        .key_last_segment_in(["low", "open"])
    {
        let (table_name, time_id, token_address) = token_windows_id_fields(&delta.key);
        let token_time_id = format!("0x{token_address}-{time_id}");
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), token_time_id.to_string());


            match table_name {
                "TokenDayData" => {
                    let change = changes.push_change_composite("ethereum_uniswap_v3_token_day_data", keys, 1, Operation::Update);
                    match key::last_segment(&delta.key) {
                        "low" => {
                            change.change("low", (None,&delta.new_value));
                        }
                        "open" => {
                            change.change("open", (None,&delta.new_value));
                        }
                        _ => {}
                    } 
                }
                "TokenHourData" => {
                    let change = changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update);
                    match key::last_segment(&delta.key) {
                        "low" => {
                            change.change("low", (None,&delta.new_value));
                        }
                        "open" => {
                            change.change("open", (None,&delta.new_value));
                        }
                        _ => {}
                    } 
                }
                _ => {}
            }
    }
}

fn prices_max_token_windows(changes:&mut DatabaseChanges, max_token_prices_deltas: &Deltas<DeltaBigDecimal>) {
    for delta in max_token_prices_deltas
        .iter()
        .key_first_segment_in(["TokenDayData", "TokenHourData"])
        .operation_not_eq(substreams::pb::substreams::store_delta::Operation::Delete)
    {
        let (table_name, time_id, token_address) = token_windows_id_fields(&delta.key);
        let token_time_id = format!("0x{token_address}-{time_id}");
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), token_time_id.to_string());
        match table_name {
            "TokenDayData" => {
                    let change = changes.push_change_composite("ethereum_uniswap_v3_token_day_data", keys, 1, Operation::Update);
                    change.change("high", (None,&delta.new_value));
                }
                "TokenHourData" => {
                    let change = changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update);
                    change.change("high", (None,&delta.new_value));
                }
                _ => {}
        }
    }
}


pub fn prices_close_token_windows(changes:&mut DatabaseChanges, eth_prices_deltas: &Deltas<DeltaBigDecimal>) {
    for delta in eth_prices_deltas
        .iter()
        .key_first_segment_in(["TokenDayData", "TokenHourData"])
        .operation_eq(substreams::pb::substreams::store_delta::Operation::Delete)
    {
        let (table_name, time_id, token_address) = token_windows_id_fields(&delta.key);
        let token_time_id = format!("0x{token_address}-{time_id}");
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), token_time_id.to_string());
        match table_name {
            "TokenDayData" => {
                    let change = changes.push_change_composite("ethereum_uniswap_v3_token_day_data", keys, 1, Operation::Update);
                    change.change("close", (None,&delta.new_value));
                }
                "TokenHourData" => {
                    let change = changes.push_change_composite("ethereum_uniswap_v3_pool_hour_data", keys, 1, Operation::Update);
                    change.change("close", (None,&delta.new_value));
                }
                _ => {}
        }
    }
}






