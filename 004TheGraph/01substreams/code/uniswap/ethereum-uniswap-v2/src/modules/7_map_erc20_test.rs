
use substreams::{hex, Hex};
use substreams_ethereum::{ rpc::RpcBatch};
use substreams_ethereum::{
    pb::eth::v2::{self as eth},
};
use crate::abi::erc20::functions::{Name,Symbol,Decimals,TotalSupply};

// ERC20 Transfer event signature
const TRANSFER_EVENT_SIGNATURE: &str = "ddf252ad";


#[substreams::handlers::map]
pub fn map_erc20_test(block: eth::Block) -> Result<_, substreams::errors::Error> {
    let block_number = block.number;
    // Track seen tokens to avoid duplicates
    let mut seen_tokens = std::collections::HashSet::new();
    // let mut erc20s:Vec<Erc20v1> = vec![];

    for transaction in block.transaction_traces {
         // let mut tokens = vec![];
        
        for log in transaction.receipt.unwrap().logs{
            // if log.topics.get(0).map(|t| t.as_slice()) == Some(Hex::decode(TRANSFER_EVENT_SIGNATURE).unwrap().as_slice()) {
             if log.topics.len() > 0 {
                
                let topic0 = &log.topics[0];
                let topic0_hex = Hex::encode(topic0);
                substreams::log::debug!("topic0_hex: {}  ", topic0_hex);
                if topic0_hex.starts_with(TRANSFER_EVENT_SIGNATURE) {
                         substreams::log::debug!("is ok: {}  ", TRANSFER_EVENT_SIGNATURE);
                }

             }
             if log.topics.len() > 0 && (Hex::encode(&log.topics[0][..8] )== TRANSFER_EVENT_SIGNATURE ){
                let token_address = Hex::encode(&log.address);
                if seen_tokens.contains(&token_address) {
                    continue;
                }
                seen_tokens.insert(token_address.clone());


                let batch = RpcBatch::new();
                let responses = batch.add(Name{}, log.address.clone())
                .add(Symbol{}, log.address.clone())
                .add(Decimals{}, log.address.clone())
                .add(TotalSupply{}, log.address.clone())
                .execute()
                .unwrap()
                .responses;

                let mut name = String::new();
                match substreams_ethereum::rpc::RpcBatch::decode::<_,Name>(&responses[0]) {
                    Some(decode_name) => {
                        name = decode_name.to_string();
                        substreams::log::debug!("decode_name ok: {}", name);
                    },
                    None => {
                        substreams::log::debug!("failed to get name");
                    },
                }
               
                let mut symbol= String::new();
                match substreams_ethereum::rpc::RpcBatch::decode::<_,Decimals>(&responses[1]) {
                    Some(decoded_symbol) => {
                        symbol = decoded_symbol.to_string();
                        substreams::log::debug!("decoded_decimals ok: {}", symbol);
                    }
                    None => {
                        substreams::log::debug!("failed to get symbol");
                    }
                };

                let mut decimals = u64::default();
                match substreams_ethereum::rpc::RpcBatch::decode::<_,Decimals>(&responses[2]) {
                    Some(decoded_decimals) => {
                        decimals = decoded_decimals.to_u64();
                        substreams::log::debug!("decoded_decimals ok: {}", decimals);
                    }
                    None => {
                        substreams::log::debug!("failed to get decimals");
                    }
                };

                let mut total_supply = String::new();
                match substreams_ethereum::rpc::RpcBatch::decode::<_,TotalSupply>(&responses[3]) {
                    Some(decoded_total_supply) => {
                        total_supply = decoded_total_supply.to_string();
                        substreams::log::debug!("decoded_total_supply ok: {}", total_supply);
                    },
                    None => {
                         substreams::log::debug!("failed to get total_supply");
                    },
                }
                let transaction_hash = Hex::encode(&transaction.hash);


                let id = format!("{}_{}_{}",block_number,transaction_hash,Hex::encode(&log.address));
                // erc20s.push(Erc20v1{
                //     id,
                //     name,
                //     symbol,
                //     decimals,
                //     total_supply,
                // });

                
            }
        }
    }

    Ok(())
}

// #[derive(Debug)]
// pub struct Erc20v1 {
//     id : String,
//     name : String,
//     symbol : String,
//     decimals:u64,
//     total_supply:String,
// }
