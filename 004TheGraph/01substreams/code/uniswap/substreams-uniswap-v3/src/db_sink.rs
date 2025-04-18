use std::collections::HashMap;

use substreams::{key, scalar::{BigDecimal, BigInt}, store::{DeltaBigDecimal, DeltaBigInt, DeltaExt, DeltaProto, Deltas, StoreGet, StoreGetInt64}};
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};

use crate::pb::uniswap::{events::{self, PoolSqrtPrice}, Erc20Token, Pool, Pools};

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
    changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_factory", keys, 1, Operation::Update)
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
    changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_factory", keys, 1, Operation::Update)
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
    let change = changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_factory", keys, 1, Operation::Update);
    
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

pub fn derived_factory_tvl_deltas(derived_factory_tvl_deltas:Deltas<DeltaBigDecimal>,changes:&mut DatabaseChanges){
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
    let change = changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_factory", keys, 1, Operation::Update);
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
    changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_pools", keys, 1, Operation::Create)
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
    changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
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
    changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
    .change("liquidity", (None,liquidity));
}

pub fn fee_growth_global_pool_entity_change(changes:&mut DatabaseChanges, updates :&Vec<events::FeeGrowthGlobal>){
    for update in updates {
        let pool_address =  &update.pool_address;
        let id = format!("0x{}",pool_address);
        let mut keys: HashMap<String, String> = HashMap::new();
        keys.insert("id".to_string(), id);
        let change = changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_pools", keys, 1, Operation::Update);
        if update.token_idx == 0 {
            change.change("feeGrowthGlobal0X128", (None,&update.new_value));
        } else if update.token_idx == 1 {
            change.change("feeGrowthGlobal1X128", (None,&update.new_value));
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
    let change = changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_pools", keys, 1, Operation::Update);
    
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
        let change = changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_pools", keys, 1, Operation::Update);

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
        let change = changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_pools", keys, 1, Operation::Update);

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
        changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
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
           
            changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
            .change("liquidity_provider_count", (None,&delta.new_value.to_bigint()));
            continue;
        } else if field_name == "volumeToken0" {
            changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
            .change("volume_token0", (None,&delta.new_value));
        } else if field_name == "volumeToken1" {
            changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
            .change("volume_token1", (None,&delta.new_value));
        } else if field_name == "volumeUSD" {
            changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
            .change("volume_usd", (None,&delta.new_value));
        } else if field_name == "untrackedVolumeUSD" {
            changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
            .change("untracked_volume_usd", (None,&delta.new_value));
        } else if field_name == "feesUSD" {
            changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_pools", keys, 1, Operation::Update)
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
    changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_tokens", keys, 1, Operation::Create)
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
            changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_tokens", keys, 1, Operation::Update)
            .change("volume", (None,&delta.new_value));
        } else if field_name == "volumeUSD" {
            changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_tokens", keys, 1, Operation::Update)
            .change("volume_usd", (None,&delta.new_value));
        } else if field_name == "untrackedVolumeUSD" {
            changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_tokens", keys, 1, Operation::Update)
            .change("untracked_volume_usd", (None,&delta.new_value));
        } else if field_name == "feesUSD" {
            changes.push_change_composite("ethereum_uniswap_v3.ethereum_uniswap_v3_tokens", keys, 1, Operation::Update)
            .change("fees_usd", (None,&delta.new_value));
        }
    }

}












