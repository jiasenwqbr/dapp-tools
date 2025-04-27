extern crate core;
use pb::pcs::Pair;
use pb::tokens::{self, Tokens};
use prost::EncodeError;
// use pb::pcs::Pair;
use substreams::errors::Error;
use eth::{address_decode, address_pretty};
use substreams::store::{StoreSet, StoreSetRaw};
use substreams::{log, proto, store};
use crate::event::pcs_event::Event;
use crate::event::PcsEvent;
use crate::pb::database::DatabaseChanges;
use crate::pb::pcs;
use crate::pb::tokens::Token;
use crate::pcs::event::Type;
use crate::utils::zero_big_decimal;
mod eth;
mod macros;
mod pb;
mod utils;
mod event;
mod rpc;
mod swap;

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
use substreams::store::StoreNew;
#[substreams::handlers::store]
pub fn store_pairs(pairs: pcs::Pairs, output:StoreSetRaw) {
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
