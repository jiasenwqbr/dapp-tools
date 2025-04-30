
use prost::Message;
use substreams::store::{StoreSet, StoreSetRaw};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::pb::eth::v2::Log;
use substreams::{log, output, proto, store};
use crate::eth_utils::address_pretty;
use crate::pb::pcs::Pair;
use crate::{event, utils};
use crate::pb::tokens::Token;
use crate::pb::{self, pancake, pcs};
use substreams::errors::Error;
const SWAP_TOPIC: &str = "0xcA143Ce32Fe78f1f7019d7d551a6402fC5350c73"; // keccak256("Swap(address,uint256,uint256,uint256,uint256,address)")
#[substreams::handlers::map]
pub fn map_swaps(block: eth::Block) -> Result<pancake::Swaps, substreams::errors::Error> {
    let mut swaps: Vec<pancake::Swap> = vec![];

    for trx in block.transaction_traces {
        for log in trx.receipt.unwrap().logs {
            
            if log.topics.len() != 3 {
                continue;
            }
            
            if hex::encode(&log.topics[0]) != SWAP_TOPIC.trim_start_matches("0x") {
                continue;
            }
            log::info!("&log.topics[0]:{}",hex::encode(&log.topics[0])); 
            // 解析出事件
            let swap = parse_swap_event(&log, &trx.hash)?;
            swaps.push(swap);
        }
    }

    Ok(pancake::Swaps { swaps })
}

fn parse_swap_event(log: &Log, trx_hash: &Vec<u8>) -> Result<pancake::Swap, substreams::errors::Error> {
    
    Ok(pancake::Swap {
        sender: hex::encode(&log.topics[1][12..]),
        to: hex::encode(&log.topics[2][12..]),
        pair_address: hex::encode(&log.address),
        token0_address: "".to_string(), // token0 需要通过额外查询pair合约拿，这里先留空
        token1_address: "".to_string(),
        trx_hash: hex::encode(trx_hash),
        log_ordinal: log.ordinal,
        amount0_in:hex::encode( &log.data[0..32]),
        amount1_in:hex::encode( &log.data[32..64]),
        amount0_out:hex::encode(&log.data[64..96]),
        amount1_out:hex::encode(&log.data[96..128]),
    })
}


// #[substreams::handlers::map]
// pub fn map_reserves(blk: pb::eth::Block, pairs: impl store::StoreGet<Vec<u8>> ,tokens: impl store::StoreGet<Vec<u8>>) -> Result<pcs::Reserves, Error> {
//     let mut reserves = pcs::Reserves { reserves: vec![] };

//     for trx in blk.transaction_traces {
//         for log in trx.receipt.unwrap().logs {
//             let addr = address_pretty(&log.address);
//             match pairs.get_last(&format!("pair:{}", addr)) {
//                 None => continue,
//                 Some(pair_bytes) => {
//                     let sig = hex::encode(&log.topics[0]);

//                     if !event::is_pair_sync_event(sig.as_str()) {
//                         continue;
//                     }

//                     let pair: pcs::Pair = proto::decode(&pair_bytes).unwrap();

//                     let token0: Token = utils::get_last_token(&tokens, &pair.token0_address);
//                     let reserve0 =
//                         utils::convert_token_to_decimal(&log.data[0..32], &token0.decimals);
//                     let token1: Token = utils::get_last_token(&tokens, &pair.token1_address);
//                     let reserve1 =
//                         utils::convert_token_to_decimal(&log.data[32..64], &token1.decimals);

//                     let token0_price = utils::get_token_price(reserve0.clone(), reserve1.clone());
//                     let token1_price = utils::get_token_price(reserve1.clone(), reserve0.clone());

//                     reserves.reserves.push(pcs::Reserve {
//                         pair_address: pair.address,
//                         reserve0: reserve0.to_string(),
//                         reserve1: reserve1.to_string(),
//                         log_ordinal: log.block_index as u64,
//                         token0_price: token0_price.to_string(),
//                         token1_price: token1_price.to_string(),
//                     });
//                 }
//             }
//         }
//     }

//     Ok(reserves)
// }

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
pub fn store_pairs(pairs: pcs::Pairs, output: impl StoreSet<Vec<u8>>) {
    for pair in pairs.pairs {
        let encoded = pair.encode_to_vec();

        output.set(
            pair.log_ordinal,
            &format!("pair:{}", pair.address),
            &encoded,
        );

        output.set(
            pair.log_ordinal,
            &format!(
                "tokens:{}",
                utils::generate_tokens_key(
                    pair.token0_address.as_str(),
                    pair.token1_address.as_str(),
                )
            ),
            &encoded,
        );
    }
}