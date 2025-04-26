extern crate core;
use substreams::errors::Error;

use crate::pb::pcs;
mod eth;
mod macros;
mod pb;
mod utils;
#[substreams::handlers::map]
pub fn map_pairs(blk: pb::eth::Block) -> Result<pcs::Pairs, Error> {
    let mut pairs: pcs::Pairs = pcs::Pairs { pairs: vec![] };

    // for trx in blk.transaction_traces {
    //     /* PCS Factory address */
    //     //0xbcfccbde45ce874adcb698cc183debcf17952812
    //     if hex::encode(&trx.to) != "ca143ce32fe78f1f7019d7d551a6402fc5350c73" {
    //         continue;
    //     }

    //     for log in trx.receipt.unwrap().logs {
    //         let sig = hex::encode(&log.topics[0]);
    //     }
    // }

    Ok(pairs)
}
