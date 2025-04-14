
use ethabi::{Address, Contract, Param, ParamType, Token};
use substreams::log;
use std::collections::HashMap;
use substreams::{prelude::BigInt, Hex};
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams_ethereum::{
    pb::eth::{ rpc::{RpcCall, RpcCalls}, v2::{self as eth, Log}}, rpc::RpcBatch, Event, Function
};
use crate::{abi::{erc20, pool::events::Swap}, persistence::persistence};

// ERC20 Transfer event signature
const TRANSFER_EVENT_SIGNATURE: &str = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";


#[substreams::handlers::map]
pub fn map_erc20( params: String,block: eth::Block) -> Result<DatabaseChanges, substreams::errors::Error> {
    let block_number = block.number;
    let block_time = block.timestamp_seconds();
    let mut database_changes: DatabaseChanges = Default::default();

    // let mut tokens = vec![];

    // Track seen tokens to avoid duplicates
    let mut seen_tokens = std::collections::HashSet::new();
    for transaction in block.transaction_traces {
        for log in transaction.receipt.unwrap().logs{
            if log.topics.get(0).map(|t| t.as_slice()) == Some(Hex::decode(TRANSFER_EVENT_SIGNATURE).unwrap().as_slice()) {
                let token_address = Hex::encode(&log.address);
                if seen_tokens.contains(&token_address) {
                    continue;
                }
                seen_tokens.insert(token_address.clone());

                // Create batch RPC calls to get token info
                let mut batch = RpcBatch::new();
                
               
               
            }

        }
    }



    Ok(database_changes)
}

// Helper function to parse string responses
fn parse_string_response(response: &serde_json::Value) -> String {
    if let Some(result) = response.get("result") {
        if let Some(hex_str) = result.as_str() {
            if let Ok(bytes) = Hex::decode(hex_str.trim_start_matches("0x")) {
                // String data starts at byte 64 (32 bytes for offset + 32 bytes for length)
                if bytes.len() >= 64 {
                    let len = u32::from_be_bytes(bytes[32..36].try_into().unwrap()) as usize;
                    if bytes.len() >= 64 + len {
                        return String::from_utf8_lossy(&bytes[64..64+len]).to_string();
                    }
                }
            }
        }
    }
    "".to_string()
}

// Helper function to parse decimals
fn parse_decimals_response(response: &serde_json::Value) -> u32 {
    if let Some(result) = response.get("result") {
        if let Some(hex_str) = result.as_str() {
            if let Ok(bytes) = Hex::decode(hex_str.trim_start_matches("0x")) {
                if !bytes.is_empty() {
                    return bytes[bytes.len() - 1] as u32;
                }
            }
        }
    }
    0
}

// Helper function to parse uint256 responses
fn parse_uint_response(response: &serde_json::Value) -> String {
    if let Some(result) = response.get("result") {
        if let Some(hex_str) = result.as_str() {
            return hex_str.trim_start_matches("0x").to_string();
        }
    }
    "0".to_string()
}

struct ERC20Token{
    address : String,
    name :String,
    symbol:String,
    decimals:u32,
    total_supply : String
}

