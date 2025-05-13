use std::convert::TryFrom;
use std::str::FromStr;
use substreams::store::{StoreAdd, StoreDelete, StoreNew, StoreSet, StoreSetIfNotExists, StoreSetRaw};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams::{log, proto, store,Hex,hex};
use crate::eth_utils::{self, address_pretty};
use crate::pb::pcs::event::Type;
use crate::rpc::{create_rpc_calls, create_rpc_calls2};
use crate::utils::zero_big_decimal;
use crate::{db, event, rpc, utils};
use crate::pb::tokens::Token;
use crate::pb::{self, pcs};
use substreams::errors::Error;
use substreams::prelude::StoreGetRaw;
use substreams::store::StoreGet;

use substreams::store::StoreSetIfNotExistsRaw;
use bigdecimal::BigDecimal;
use substreams::prelude::StoreAddInt64;
use crate::event::pcs_event::Event;
use crate::event::PcsEvent;
use substreams_ethereum::pb::eth as ethpb;
const INITIALIZE_METHOD_HASH: [u8; 4] = hex!("1459457a");
// const SWAP_TOPIC: &str = "0xcA143Ce32Fe78f1f7019d7d551a6402fC5350c73"; // keccak256("Swap(address,uint256,uint256,uint256,uint256,address)");
#[substreams::handlers::map]
pub fn map_pairs(blk: pb::eth::Block) -> Result<pcs::Pairs, Error> {
    let mut pairs = pcs::Pairs { pairs: vec![] };

    for trx in blk.transaction_traces {
        /* PCS Factory address */
        //0xbcfccbde45ce874adcb698cc183debcf17952812
        if hex::encode(&trx.to) != "ca143ce32fe78f1f7019d7d551a6402fc5350c73" {
            continue;
        }

        for log in trx.receipt.unwrap().logs {
            let sig = hex::encode(&log.topics[0]);

            if !event::is_pair_created_event(sig.as_str()) {
                continue;
            }
            log::info!("Writing pair key: {}", address_pretty(&log.data[12..32]));
            pairs.pairs.push(pcs::Pair {
                address: address_pretty(&log.data[12..32]),
                token0_address: address_pretty(&log.topics[1][12..]),
                token1_address: address_pretty(&log.topics[2][12..]),
                creation_transaction_id: address_pretty(&trx.hash),
                block_num: blk.number,
                log_ordinal: log.block_index as u64,
            })
            
        }
    }

    Ok(pairs)
}

#[substreams::handlers::store]
pub fn store_pairs(pairs: pcs::Pairs, output: store::StoreSetRaw) {
    log::info!("Building pair state");
    for pair in pairs.pairs {
        output.set(
            pair.log_ordinal,
            format!("pair:{}", pair.address),
            &proto::encode(&pair).unwrap(),
        );
        output.set(
            pair.log_ordinal as u64,
            format!(
                "tokens:{}",
                utils::generate_tokens_key(
                    pair.token0_address.as_str(),
                    pair.token1_address.as_str(),
                )
            ),
            &proto::encode(&pair).unwrap(),
        );
    }
}

#[substreams::handlers::map]
pub fn map_reserves(blk: pb::eth::Block, pairs: StoreGetRaw, tokens: store::StoreGetRaw) -> Result<pcs::Reserves, Error> {
    let mut reserves = pcs::Reserves { reserves: vec![] };
    log::info!(" map_reserves - block number is : {:?}", blk.number);
    for trx in blk.transaction_traces {
        for log in trx.receipt.unwrap().logs {
            let addr = address_pretty(&log.address);
            match pairs.get_last(&format!("pair:{}", addr)) {
                None => continue,
                Some(pair_bytes) => {
                    let sig = hex::encode(&log.topics[0]);

                    if !event::is_pair_sync_event(sig.as_str()) {
                        continue;
                    }

                    let pair: pcs::Pair = proto::decode(&pair_bytes).unwrap();

                    let token0: Token = utils::get_last_token(&tokens, &pair.token0_address);
                    let reserve0 =
                        utils::convert_token_to_decimal(&log.data[0..32], &token0.decimals);
                    let token1: Token = utils::get_last_token(&tokens, &pair.token1_address);
                    let reserve1 =
                        utils::convert_token_to_decimal(&log.data[32..64], &token1.decimals);

                    let token0_price = utils::get_token_price(reserve0.clone(), reserve1.clone());
                    let token1_price = utils::get_token_price(reserve1.clone(), reserve0.clone());

                    reserves.reserves.push(pcs::Reserve {
                        pair_address: pair.address,
                        reserve0: reserve0.to_string(),
                        reserve1: reserve1.to_string(),
                        log_ordinal: log.block_index as u64,
                        token0_price: token0_price.to_string(),
                        token1_price: token1_price.to_string(),
                    });
                }
            }
        }
    }
    log::info!(" pair value: {:?}", reserves);

    Ok(reserves)
}


#[substreams::handlers::store]
pub fn store_reserves(clock: substreams::pb::substreams::Clock, reserves: pcs::Reserves, pairs: store::StoreGetRaw, output: store::StoreSetRaw) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id: i64 = timestamp_seconds / 86400;
    let hour_id: i64 = timestamp_seconds / 3600;

    output.delete_prefix(0, &format!("pair_day:{}:", day_id - 1));
    output.delete_prefix(0, &format!("pair_hour:{}:", hour_id - 1));

    for reserve in reserves.reserves {
        match pairs.get_last(&format!("pair:{}", reserve.pair_address)) {
            None => continue,
            Some(pair_bytes) => {
                let pair: pcs::Pair = proto::decode(&pair_bytes).unwrap();

                output.set(
                    reserve.log_ordinal,
                    format!("price:{}:{}:token0", pair.address, pair.token0_address),
                    &Vec::from(reserve.token0_price),
                );
                output.set(
                    reserve.log_ordinal,
                    format!("price:{}:{}:token1", pair.address, pair.token1_address),
                    &Vec::from(reserve.token1_price),
                );

                output.set_many(
                    reserve.log_ordinal,
                    &vec![
                        format!(
                            "reserve:{}:{}:reserve0",
                            reserve.pair_address, pair.token0_address
                        ),
                        format!("pair_day:{}:{}:reserve0", day_id, pair.token0_address),
                        format!("pair_hour:{}:{}:reserve0", hour_id, pair.token0_address),
                    ],
                    &Vec::from(reserve.reserve0),
                );

                output.set_many(
                    reserve.log_ordinal,
                    &vec![
                        format!(
                            "reserve:{}:{}:reserve1",
                            reserve.pair_address, pair.token1_address
                        ),
                        format!("pair_day:{}:{}:reserve1", day_id, pair.token1_address),
                        format!("pair_hour:{}:{}:reserve1", hour_id, pair.token1_address),
                    ],
                    &Vec::from(reserve.reserve1),
                )
            }
        }
    }
}


#[substreams::handlers::store]
pub fn store_prices(clock: substreams::pb::substreams::Clock, reserves: pcs::Reserves, pairs: store::StoreGetRaw, reserves_store: store::StoreGetRaw, output: store::StoreSetRaw) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id: i64 = timestamp_seconds / 86400;
    let hour_id: i64 = timestamp_seconds / 3600;

    output.delete_prefix(0, &format!("pair_day:{}:", day_id - 1));
    output.delete_prefix(0, &format!("pair_hour:{}:", hour_id - 1));
    output.delete_prefix(0, &format!("token_day:{}:", day_id - 1));

    for reserve in reserves.reserves {
        match pairs.get_last(&format!("pair:{}", reserve.pair_address)) {
            None => continue,
            Some(pair_bytes) => {
                let pair: pcs::Pair = proto::decode(&pair_bytes).unwrap();

                let latest_usd_price: BigDecimal =
                    utils::compute_usd_price(&reserves_store, &reserve);

                if reserve.pair_address.eq(&utils::USDT_WBNB_PAIR)
                    || reserve.pair_address.eq(&utils::BUSD_WBNB_PAIR)
                {
                    output.set(
                        reserve.log_ordinal,
                        format!("dprice:usd:bnb"),
                        &Vec::from(latest_usd_price.to_string()),
                    )
                }

                // sets:
                // * dprice:%s:bnb (tokenA)  - as contributed by any pair's sync to that token
                // * dprice:%s:usd (tokenA)  - same
                // * dreserve:%s:%s:bnb (pair, token)
                // * dreserve:%s:%s:usd (pair, token)
                // * dreserves:%s:bnb (pair)  - sum of both token's reserves
                // derived from:
                // * price:%s:%s (tokenA, tokenB)
                // * reserve:%s:%s (pair, tokenA)
                let usd_price_valid: bool = latest_usd_price.ne(&zero_big_decimal());

                let t0_derived_bnb_price = utils::find_bnb_price_per_token(
                    &reserve.log_ordinal,
                    pair.token0_address.as_str(),
                    &pairs,
                    &reserves_store,
                );

                let t1_derived_bnb_price = utils::find_bnb_price_per_token(
                    &reserve.log_ordinal,
                    pair.token1_address.as_str(),
                    &pairs,
                    &reserves_store,
                );

                let apply = |token_derived_bnb_price: Option<BigDecimal>,
                             token_addr: String,
                             reserve_amount: String|
                 -> BigDecimal {
                    if token_derived_bnb_price.is_none() {
                        return zero_big_decimal();
                    }

                    output.set(
                        reserve.log_ordinal,
                        format!("dprice:{}:bnb", token_addr),
                        &Vec::from(token_derived_bnb_price.clone().unwrap().to_string()),
                    );


                    let reserve_in_bnb = BigDecimal::from_str(reserve_amount.as_str())
                        .unwrap() * token_derived_bnb_price.clone().unwrap();
                        // .mul(token_derived_bnb_price.clone().unwrap());
                    output.set(
                        reserve.log_ordinal,
                        format!("dreserve:{}:{}:bnb", reserve.pair_address, token_addr),
                        &Vec::from(reserve_in_bnb.clone().to_string()),
                    );

                    if usd_price_valid {
                        let derived_usd_price: BigDecimal = token_derived_bnb_price
                            .unwrap() * latest_usd_price.clone();
                            // .mul(latest_usd_price.clone());
                        output.set_many(
                            reserve.log_ordinal,
                            &vec![
                                format!("dprice:{}:usd", token_addr),
                                format!("token_day:{}:dprice:{}:usd", day_id, token_addr),
                            ],
                            &Vec::from(derived_usd_price.to_string()),
                        );

                        let reserve_in_usd = reserve_in_bnb.clone() * latest_usd_price.clone();
                        // mul(latest_usd_price.clone());

                        output.set_many(
                            reserve.log_ordinal,
                            &vec![
                                format!("dreserve:{}:{}:usd", reserve.pair_address, token_addr),
                                format!("pair_day:{}:dreserve:{}:usd", day_id, pair.token0_address),
                                format!("pair_day:{}:dreserve:{}:usd", day_id, pair.token1_address),
                                format!(
                                    "pair_hour:{}:dreserve:{}:usd",
                                    hour_id, pair.token0_address
                                ),
                                format!(
                                    "pair_hour:{}:dreserve:{}:usd",
                                    hour_id, pair.token1_address
                                ),
                            ],
                            &Vec::from(reserve_in_usd.to_string()),
                        );
                    }

                    return reserve_in_bnb;
                };

                let reserve0_bnb = apply(
                    t0_derived_bnb_price,
                    pair.token0_address.clone(),
                    reserve.reserve0.clone(),
                );
                let reserve1_bnb = apply(
                    t1_derived_bnb_price,
                    pair.token1_address.clone(),
                    reserve.reserve1.clone(),
                );

                let reserves_bnb_sum = reserve0_bnb * reserve1_bnb; //.mul(reserve1_bnb);
                if reserves_bnb_sum.ne(&zero_big_decimal()) {
                    output.set(
                        reserve.log_ordinal,
                        format!("dreserves:{}:bnb", reserve.pair_address),
                        &Vec::from(reserves_bnb_sum.to_string()),
                    );
                }
            }
        }
    }
}



#[substreams::handlers::map]
pub fn map_burn_swaps_events(blk: pb::eth::Block, pairs_store: store::StoreGetRaw, prices_store: store::StoreGetRaw, tokens_store: store::StoreGetRaw) -> Result<pcs::Events, Error> {
    let mut events: pcs::Events = pcs::Events { events: vec![] };

    let mut burn_count: i32 = 0;
    let mut mint_count: i32 = 0;
    let mut swap_count: i32 = 0;

    for trx in blk.transaction_traces {
        let trx_id = address_pretty(trx.hash.as_slice());
        for call in trx.calls {
            if call.state_reverted {
                continue;
            }

            if call.logs.len() == 0 {
                continue;
            }

            let pair_addr = address_pretty(call.address.as_slice());

            let pair: pcs::Pair;
            match pairs_store.get_last(&format!("pair:{}", pair_addr)) {
                None => continue,
                Some(pair_bytes) => pair = proto::decode(&pair_bytes).unwrap(),
            }

            let mut pcs_events: Vec<PcsEvent> = Vec::new();

            for log in call.logs {
                pcs_events.push(event::decode_event(log));
            }

            let mut base_event = pcs::Event {
                log_ordinal: 0,
                pair_address: pair_addr,
                token0: pair.token0_address.clone(),
                token1: pair.token1_address.clone(),
                transaction_id: trx_id.to_string(),
                timestamp: blk
                    .header
                    .as_ref()
                    .unwrap()
                    .timestamp
                    .as_ref()
                    .unwrap()
                    .seconds as u64,
                r#type: None,
            };

            if pcs_events.len() == 4 {
                let ev_tr1 = match pcs_events[0].event.as_ref().unwrap() {
                    Event::PairTransferEvent(pair_transfer_event) => Some(pair_transfer_event),
                    _ => None,
                };

                let ev_tr2 = match pcs_events[1].event.as_ref().unwrap() {
                    Event::PairTransferEvent(pair_transfer_event) => Some(pair_transfer_event),
                    _ => None,
                };

                match pcs_events[3].event.as_ref().unwrap() {
                    Event::PairMintEvent(pair_mint_event) => {
                        let mint_id = format!("{}-{}", trx_id, mint_count);
                        mint_count += 1;

                        event::process_mint(
                            mint_id.as_str(),
                            &mut base_event,
                            &prices_store,
                            &pair,
                            ev_tr1,
                            ev_tr2,
                            pair_mint_event,
                            utils::get_last_token(&tokens_store, pair.token0_address.as_str())
                                .decimals,
                            utils::get_last_token(&tokens_store, pair.token1_address.as_str())
                                .decimals,
                        )
                    }
                    Event::PairBurnEvent(pair_burn_event) => {
                        let burn_id = format!("{}-{}", trx_id, burn_count);
                        burn_count = burn_count + 1;

                        event::process_burn(
                            burn_id.as_str(),
                            &mut base_event,
                            &prices_store,
                            &pair,
                            ev_tr1,
                            ev_tr2,
                            pair_burn_event,
                            utils::get_last_token(&tokens_store, pair.token0_address.as_str())
                                .decimals,
                            utils::get_last_token(&tokens_store, pair.token1_address.as_str())
                                .decimals,
                        );
                    }
                    _ => {
                        log::info!("Error?! Events len is 4"); // fixme: should we panic here or just continue?
                        continue;
                    }
                }
            } else if pcs_events.len() == 3 {
                let ev_tr2 = match pcs_events[0].event.as_ref().unwrap() {
                    Event::PairTransferEvent(pair_transfer_event) => Some(pair_transfer_event),
                    _ => None,
                };

                match pcs_events[2].event.as_ref().unwrap() {
                    Event::PairMintEvent(pair_mint_event) => {
                        let mint_id = format!("{}-{}", trx_id, mint_count);
                        mint_count += 1;

                        event::process_mint(
                            mint_id.as_str(),
                            &mut base_event,
                            &prices_store,
                            &pair,
                            None,
                            ev_tr2,
                            pair_mint_event,
                            utils::get_last_token(&tokens_store, pair.token0_address.as_str())
                                .decimals,
                            utils::get_last_token(&tokens_store, pair.token1_address.as_str())
                                .decimals,
                        )
                    }
                    Event::PairBurnEvent(pair_burn_event) => {
                        let burn_id = format!("{}-{}", trx_id, burn_count);
                        burn_count += 1;

                        event::process_burn(
                            burn_id.as_str(),
                            &mut base_event,
                            &prices_store,
                            &pair,
                            None,
                            ev_tr2,
                            pair_burn_event,
                            utils::get_last_token(&tokens_store, pair.token0_address.as_str())
                                .decimals,
                            utils::get_last_token(&tokens_store, pair.token1_address.as_str())
                                .decimals,
                        );
                    }
                    _ => {
                        log::info!("Error?! Events len is 3"); // fixme: should we panic here or just continue?
                        continue;
                    }
                }
            } else if pcs_events.len() == 2 {
                match pcs_events[1].event.as_ref().unwrap() {
                    Event::PairSwapEvent(pair_swap_event) => {
                        let swap_id = format!("{}-{}", trx_id, swap_count);
                        swap_count += 1;

                        event::process_swap(
                            swap_id.as_str(),
                            &mut base_event,
                            &prices_store,
                            &pair,
                            Some(pair_swap_event),
                            address_pretty(trx.from.as_slice()),
                            utils::get_last_token(&tokens_store, &pair.token0_address).decimals,
                            utils::get_last_token(&tokens_store, &pair.token1_address).decimals,
                        );
                    }
                    _ => {
                        log::info!("Error?! Events len is 2"); // fixme: should we panic here or just continue?
                        continue;
                    }
                }
            } else if pcs_events.len() == 1 {
                match pcs_events[0].event.as_ref().unwrap() {
                    Event::PairTransferEvent(_) => {
                        log::debug!("Events len 1, PairTransferEvent");
                        continue;
                    } // do nothing
                    Event::PairApprovalEvent(_) => {
                        log::debug!("Events len 1, PairApprovalEvent");
                        continue;
                    } // do nothing
                    _ => panic!("unhandled event pattern, with 1 event"),
                };
            } else {
                panic!("unhandled event pattern with {} events", pcs_events.len());
            }

            events.events.push(base_event);
        }
    }

    Ok(events)
}

#[substreams::handlers::store]
pub fn store_totals(
    clock: substreams::pb::substreams::Clock,
    pairs: pcs::Pairs,
    events: pcs::Events,
    output: store::StoreAddInt64,
) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id: i64 = timestamp_seconds / 86400;

    if events.events.len() == 0 && pairs.pairs.len() == 0 {
        return;
    }

    for pair in pairs.pairs {
        output.add(pair.log_ordinal, "global:pair_count".to_string(), 1);
    }

    for event in events.events {
        output.add_many(
            event.log_ordinal,
            &vec![
                format!("token:{}:transaction_count", event.token0),
                format!("token:{}:transaction_count", event.token1),
                format!("pair:{}:transaction_count", event.pair_address),
                format!("global_day:{}:transaction_count", day_id),
                format!("global:transaction_count"),
            ],
            1,
        );

        match event.r#type.unwrap() {
            Type::Swap(swap) => {
                if swap.amount_usd.is_empty() {
                    continue;
                }

                output.add_many(
                    event.log_ordinal,
                    &vec![format!("pair:{}:swap_count", event.pair_address)],
                    1,
                );

                //todo: if we want to set the total transactions for global day we need a
                // key setter store to keep track of the latest computed(summed) values
            }
            Type::Burn(_) => output.add(
                event.log_ordinal,
                format!("pair:{}:burn_count", event.pair_address),
                1,
            ),
            Type::Mint(_) => output.add(
                event.log_ordinal,
                format!("pair:{}:mint_count", event.pair_address),
                1,
            ),
        }
    }
}

use substreams::scalar::BigDecimal as ScalarBigDecimal;
use crate::swap::store::StoreAddBigDecimal;
#[substreams::handlers::store]
pub fn store_volumes(
    clock: substreams::pb::substreams::Clock,
    events: pcs::Events,
    output: store::StoreAddBigDecimal
) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id: i64 = timestamp_seconds / 86400;
    let hour_id: i64 = timestamp_seconds / 3600;

    if events.events.len() == 0 {
        return;
    }

    output.delete_prefix(0, &format!("pair_day:{}:", day_id - 1));
    output.delete_prefix(0, &format!("token_day:{}:", day_id - 1));
    output.delete_prefix(0, &format!("pair_hour:{}:", hour_id - 1));
    output.delete_prefix(0, &format!("global_day:{}", day_id - 1));

    for event in events.events {
        if event.r#type.is_some() {
            match event.r#type.unwrap() {
                Type::Mint(mint) => {
                    let amount_usd = BigDecimal::from_str(mint.amount_usd.as_str()).unwrap();
                    if amount_usd.eq(&zero_big_decimal()) {
                        continue;
                    }
                    let scalar_amount = to_scalar(&amount_usd);
                    
                    output.add(
                        event.log_ordinal,
                        format!("global:liquidity_usd"),
                        // &amount_usd,
                       // ScalarBigDecimal::from(amount_usd),
                        scalar_amount
                    );

                    output.add_many(
                        event.log_ordinal,
                        &vec![
                            format!("token:{}:liquidity", mint.to),
                            format!("pair:{}:total_supply", event.pair_address),
                        ],
                        // &BigDecimal::from_str(mint.liquidity.as_str()).unwrap(),
                        to_scalar(&BigDecimal::from_str(mint.liquidity.as_str()).unwrap()),
                    );
                }
                Type::Burn(burn) => {
                    let amount_usd = BigDecimal::from_str(burn.amount_usd.as_str()).unwrap();
                    if amount_usd.eq(&zero_big_decimal()) {
                        continue;
                    }
                    output.add(
                        event.log_ordinal,
                        format!("global:liquidity_usd"),
                        // &amount_usd.neg(),
                        to_scalar(&amount_usd)

                    );

                    output.add_many(
                        event.log_ordinal,
                        &vec![
                            format!("token:{}:liquidity", burn.to),
                            format!("pair:{}:total_supply", event.pair_address),
                        ],
                        // &BigDecimal::from_str(burn.liquidity.as_str()).unwrap().neg(),
                        to_scalar(&BigDecimal::from_str(burn.liquidity.as_str()).unwrap()),

                    );
                }
                Type::Swap(swap) => {
                    if swap.amount_usd.is_empty() {
                        continue;
                    }
                    let amount_usd = BigDecimal::from_str(swap.amount_usd.as_str()).unwrap();
                    if amount_usd.eq(&zero_big_decimal()) {
                        continue;
                    }
                    let amount_bnb = BigDecimal::from_str(swap.amount_bnb.as_str()).unwrap();

                    let amount_0_total: BigDecimal =
                        utils::compute_amount_total(swap.amount0_out, swap.amount0_in);
                    let amount_1_total: BigDecimal =
                        utils::compute_amount_total(swap.amount1_out, swap.amount1_in);

                    output.add_many(
                        event.log_ordinal,
                        &vec![
                            format!("pair:{}:usd", event.pair_address),
                            format!("pair_day:{}:{}:usd", day_id, event.pair_address),
                            format!("pair_hour:{}:{}:usd", hour_id, event.pair_address),
                            format!("token_day:{}:{}:usd", day_id, event.token0),
                            format!("token_day:{}:{}:usd", day_id, event.token1),
                            format!("global:usd"),
                            format!("global_day:{}:usd", day_id),
                        ],
                        to_scalar(&amount_usd),
                    );

                    output.add_many(
                        event.log_ordinal,
                        &vec![format!("global:bnb"), format!("global_day:{}:bnb", day_id)],
                        // &amount_bnb,
                        to_scalar(&amount_bnb),
                    );

                    output.add_many(
                        event.log_ordinal,
                        &vec![
                            format!("pair:{}:token0", event.pair_address),
                            format!("pair_day:{}:{}:token0", day_id, event.pair_address),
                            format!("pair_hour:{}:{}:token0", day_id, event.pair_address),
                        ],
                        // &amount_0_total,
                        to_scalar( &amount_0_total),
                    );

                    output.add_many(
                        event.log_ordinal,
                        &vec![
                            format!("pair:{}:token1", event.pair_address),
                            format!("pair_day:{}:{}:token1", day_id, event.pair_address),
                            format!("pair_hour:{}:{}:token1", day_id, event.pair_address),
                        ],
                        // &amount_1_total,
                        to_scalar(&amount_1_total),
                    );

                    output.add(
                        event.log_ordinal,
                        format!("token:{}:trade", event.token0),
                        to_scalar( &BigDecimal::from_str(swap.trade_volume0.as_str()).unwrap()),
                    );
                    output.add(
                        event.log_ordinal,
                        format!("token:{}:trade", event.token1),
                        // &BigDecimal::from_str(swap.trade_volume1.as_str()).unwrap(),
                        to_scalar(&BigDecimal::from_str(swap.trade_volume1.as_str()).unwrap()),
                    );
                    output.add(
                        event.log_ordinal,
                        format!("token:{}:trade_usd", event.token0),
                        // &BigDecimal::from_str(swap.trade_volume_usd0.as_str()).unwrap(),
                        to_scalar( &BigDecimal::from_str(swap.trade_volume_usd0.as_str()).unwrap()),
                    );
                    output.add(
                        event.log_ordinal,
                        format!("token:{}:trade_usd", event.token1),
                        // &BigDecimal::from_str(swap.trade_volume_usd1.as_str()).unwrap(),
                        to_scalar(&BigDecimal::from_str(swap.trade_volume_usd1.as_str()).unwrap()),
                    );

                    //todo: token[0,1]Day.dailyVolumeToken, tokenDay[0,1].dailyVolumeBnb ? what about these
                }
            }
        }
    }
}

pub fn to_scalar(amount: &BigDecimal) -> ScalarBigDecimal {
    ScalarBigDecimal::try_from(amount.to_string()).unwrap()
}

// todo: create pcs-token proto
#[substreams::handlers::store]
pub fn store_pcs_tokens(
    pairs: pcs::Pairs,
    tokens: store::StoreGetRaw,
    output: store::StoreSetIfNotExistsRaw,
) {
    
    let mut token0_retry: bool = false;
    let mut token0: Token = Token {
        address: "".to_string(),
        name: "".to_string(),
        symbol: "".to_string(),
        decimals: 0,
    };
    let mut token1_retry: bool = false;
    let mut token1: Token = Token {
        address: "".to_string(),
        name: "".to_string(),
        symbol: "".to_string(),
        decimals: 0,
    };

    for pair in pairs.pairs {
        let token0_option_from_store: Option<Vec<u8>> =
            tokens.get_last(&format!("token:{}", pair.token0_address));
        if token0_option_from_store.is_none() {
            log::info!(
                "token {} is not in the store, retrying rpc calls",
                pair.token0_address,
            );
            let token0_res = rpc::retry_rpc_calls(&pair.token0_address);
            if token0_res.is_err() {
                continue; // skip to next execution, we don't have a valid token
            }

            token0 = token0_res.unwrap();

            token0_retry = true;
            log::info!(
                "successfully found token {} after rpc calls",
                pair.token0_address
            );
        }

        if !token0_retry {
            // didn't need to retry as we have the token in the store
            token0 = proto::decode(&token0_option_from_store.unwrap()).unwrap();
        }

        output.set_if_not_exists(
            pair.log_ordinal,
            format!("token:{}", token0.address),
            &proto::encode(&token0).unwrap(),
        );

        let token1_option_from_store: Option<Vec<u8>> =
            tokens.get_last(&format!("token:{}", pair.token1_address));
        if token1_option_from_store.is_none() {
            log::info!(
                "token {} is not in the store, retrying rpc calls",
                pair.token1_address
            );
            let token1_res = rpc::retry_rpc_calls(&pair.token1_address);
            if token1_res.is_err() {
                continue; // skip to next execution, we don't have a valid token
            }

            token1 = token1_res.unwrap();

            token1_retry = true;
            log::info!(
                "successfully found token {} after rpc calls",
                pair.token1_address
            );
        }

        if !token1_retry {
            // didn't need to retry as we have the token in the store
            token1 = proto::decode(&token1_option_from_store.unwrap()).unwrap();
        }

        output.set_if_not_exists(
            pair.log_ordinal,
            format!("token:{}", token1.address),
            &proto::encode(&token1).unwrap(),
        );
    }
}

#[substreams::handlers::map]
pub fn db_out_postgres(
    block: substreams::pb::substreams::Clock,
    pcs_tokens_deltas: store::StoreGetRaw,
    pairs_deltas: store::StoreGetRaw,
    totals_deltas: store::StoreGetRaw,
    volumes_deltas: store::StoreGetRaw,
    reserves_deltas: store::StoreGetRaw,
    events: pcs::Events,
    pcs_tokens_store: store::StoreGetRaw,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    substreams::register_panic_hook();
    let changes: Result<DatabaseChanges, anyhow::Error> = db::process(
        &block,
        pairs_deltas,
        pcs_tokens_deltas,
        totals_deltas,
        volumes_deltas,
        reserves_deltas,
        events,
        &pcs_tokens_store,
    );
     changes
}


#[substreams::handlers::map]
pub fn map_debug_pairs(
    _blk: pb::eth::Block,
    pairs: store::StoreGetRaw,
) -> Result<pcs::Pairs, substreams::errors::Error> {
    log::info!("map_debug_pairs triggered");

    // 打印某个 key 测试是否有内容
    if let Some(val) = pairs.get_last("pair:0xb2678c414ebc63c9cc6d1a0fc45f43e249b50fde") {
        log::info!("Example pair value: {:?}", val.len());
    } else {
        log::info!("No value found for sample pair key");
    }

    Ok(pcs::Pairs { pairs: vec![] })
}

#[substreams::handlers::map]
pub fn map_reserves2(blk: pb::eth::Block, pairs: StoreGetRaw) -> Result<pcs::Reserves, Error> {
    let mut reserves = pcs::Reserves { reserves: vec![] };
    

    for trx in blk.transaction_traces {
        for log in trx.receipt.unwrap().logs {
            let addr = address_pretty(&log.address);
            //log::info!("the addr is {}",addr);
            //log::info!("the addr is {:?}",pairs.get_last(&format!("pair:{}", addr)));
            

            match pairs.get_last(format!("pair:{}", addr)) {
                None => continue,
                Some(pair_bytes) => {
                    let sig = hex::encode(&log.topics[0]);
                    log::info!("the addr is---------------------------- {}",addr);

                    if !event::is_pair_sync_event(sig.as_str()) {
                        continue;
                    }

                    let pair: pcs::Pair = proto::decode(&pair_bytes).unwrap();
                    log::info!(" pair value: {:?}", pair);
                    reserves.reserves.push(pcs::Reserve {
                        pair_address: pair.address,
                        reserve0: String::new(),
                        reserve1: String::new(),
                        log_ordinal: log.block_index as u64,
                        token0_price: String::new(),
                        token1_price: String::new(),
                    });
                }
            }
        }
    }

    log::info!(" pair value: {:?}", reserves);
    Ok(reserves)
}


#[substreams::handlers::map]
fn map_tokens(blk: pb::eth::Block) -> Result<pb::tokens::Tokens, Error> {
    let mut tokens = vec![];

    for trx in blk.transaction_traces {
        for call in trx.calls {
            if call.state_reverted {
                continue;
            }
            if call.call_type == ethpb::v2::CallType::Create as i32
                || call.call_type == ethpb::v2::CallType::Call as i32
            // proxy contract creation
            {
                let call_input_len = call.input.len();
                if call.call_type == ethpb::v2::CallType::Call as i32
                    && (call_input_len < 4 || call.input[0..4] != INITIALIZE_METHOD_HASH)
                {
                    // this will check if a proxy contract has been called to create a ERC20 contract.
                    // if that is the case the Proxy contract will call the initialize function on the ERC20 contract
                    // this is part of the OpenZeppelin Proxy contract standard
                    continue;
                }

                if call.call_type == ethpb::v2::CallType::Create as i32 {
                    let mut code_change_len = 0;
                    for code_change in &call.code_changes {
                        code_change_len += code_change.new_code.len()
                    }

                    log::debug!(
                        "found contract creation: {}, caller {}, code change {}, input {}",
                        Hex(&call.address),
                        Hex(&call.caller),
                        code_change_len,
                        call_input_len,
                    );

                    if code_change_len <= 150 {
                        // optimization to skip none viable SC
                        log::info!(
                            "skipping too small code to be a token contract: {}",
                            Hex(&call.address)
                        );
                        continue;
                    }
                } else {
                    log::debug!(
                        "found proxy initialization: contract {}, caller {}",
                        Hex(&call.address),
                        Hex(&call.caller)
                    );
                }

                if call.caller == hex!("0000000000004946c0e9f43f4dee607b0ef1fa1c")
                    || call.caller == hex!("00000000687f5b66638856396bee28c1db0178d1")
                {
                    log::debug!("skipping known caller address");
                    continue;
                }

                let rpc_call_decimal = create_rpc_calls2(&call.address, vec![rpc::DECIMALS]);
                let rpc_responses_unmarshalled_decimal: ethpb::rpc::RpcResponses =
                    substreams_ethereum::rpc::eth_call(&rpc_call_decimal);
                let response_decimal = rpc_responses_unmarshalled_decimal.responses;
                if response_decimal[0].failed {
                    let decimals_error = String::from_utf8_lossy(response_decimal[0].raw.as_ref());
                    log::debug!(
                        "{} is not an ERC20 token contract because of 'eth_call' failures [decimals: {}]",
                        Hex(&call.address),
                        decimals_error,
                    );
                    continue;
                }

                let decoded_decimals = eth_utils::read_uint32(response_decimal[0].raw.as_ref());
                if decoded_decimals.is_err() {
                    log::debug!(
                        "{} is not an ERC20 token contract decimal `eth_call` failed: {}",
                        Hex(&call.address),
                        decoded_decimals.err().unwrap(),
                    );
                    continue;
                }

                let rpc_call_name_symbol = create_rpc_calls2(&call.address, vec![rpc::NAME, rpc::SYMBOL]);
                let rpc_responses_unmarshalled: ethpb::rpc::RpcResponses =
                    substreams_ethereum::rpc::eth_call(&rpc_call_name_symbol);
                let responses = rpc_responses_unmarshalled.responses;
                if responses[0].failed || responses[1].failed {
                    let name_error = String::from_utf8_lossy(responses[0].raw.as_ref());
                    let symbol_error = String::from_utf8_lossy(responses[1].raw.as_ref());

                    log::debug!(
                        "{} is not an ERC20 token contract because of 'eth_call' failures [name: {}, symbol: {}]",
                        Hex(&call.address),
                        name_error,
                        symbol_error,
                    );
                    continue;
                };

                let decoded_name = eth_utils::read_string(responses[1].raw.as_ref());
                if decoded_name.is_err() {
                    log::debug!(
                        "{} is not an ERC20 token contract name `eth_call` failed: {}",
                        Hex(&call.address),
                        decoded_name.err().unwrap(),
                    );
                    continue;
                }

                let mut decoded_symbol = Ok(String::new()) ;
                
                if responses.len()>2 {
                    decoded_symbol= eth_utils::read_string(responses[2].raw.as_ref());
                    if decoded_symbol.is_err() {
                        log::debug!(
                            "{} is not an ERC20 token contract symbol `eth_call` failed: {}",
                            Hex(&call.address),
                            decoded_symbol.err().unwrap(),
                        );
                        continue;
                    }
                }

                let decimals = decoded_decimals.unwrap() as u64;
                let symbol = decoded_symbol.unwrap();
                let name = decoded_name.unwrap();
                log::debug!(
                    "{} is an ERC20 token contract with name {}",
                    Hex(&call.address),
                    name,
                );
                let token = pb::tokens::Token {
                    address: Hex(&call.address).to_string(),
                    name,
                    symbol,
                    decimals,
                };

                tokens.push(token);
            }
        }
    }

    Ok(pb::tokens::Tokens { tokens })
}

#[substreams::handlers::store]
fn store_tokens(tokens: pb::tokens::Tokens, store: store::StoreSetRaw) {
    for token in tokens.tokens {
        let key = format!("token:{}", token.address);
        log::info!("token address is : {}",key);
        store.set(1, key, &proto::encode(&token).unwrap());
    }
}


#[substreams::handlers::map]
fn test_store_tokens(store: store::StoreSetRaw)-> Result<pcs::Reserves, Error> {
    let mut reserves = pcs::Reserves { reserves: vec![] };
    
    Ok(reserves)
}