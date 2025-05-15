use std::collections::HashMap;
use anyhow::{anyhow, Context, Error};
use pumpfun::constants::USDC_ADDRESS;
use pumpfun::constants::USDT_ADDRESS;
use raydium_amm::constants::SOL_MINIMUM_LAMPORTS;
use regex;

use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;

pub mod raydium_amm;
use raydium_amm::constants::JUPITER_AGG_PROGRAM_ID;
use raydium_amm::constants::RAYDIUM_AMM_PROGRAM_ID;
use raydium_amm::instruction::AmmInstruction;
use raydium_amm::log::{decode_ray_log, RayLog};

use substreams_solana_utils as utils;
use substreams_solana_utils::pubkey::PubkeyRef;
use substreams_solana_utils::spl_token::TOKEN_PROGRAM_ID;
use substreams_solana_utils::system_program;
use utils::instruction::{
    get_structured_instructions, StructuredInstruction, StructuredInstructions,
};
use utils::log::Log;
use utils::pubkey::Pubkey;
use utils::system_program::{SystemInstruction, SYSTEM_PROGRAM_ID};
use utils::transaction::{get_context, TransactionContext};

// use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties};
use spl_token_substream;

pub mod pumpfun;
use pumpfun::constants::PUMPFUN_PROGRAM_ID;
use pumpfun::instruction::PumpfunInstruction;
use pumpfun::log::PumpfunLog;

use system_program_substream;

pub mod pb;
use pb::raydium_amm::raydium_amm_event::Event;
use pb::raydium_amm::*;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};

#[substreams::handlers::map]
fn db_out(block: Block) -> Result<DatabaseChanges, substreams::errors::Error> {
    let transactions: Vec<RaydiumAmmTransactionEvents> = parse_block(&block);
    let mut database_changes: DatabaseChanges = Default::default();
    let block_number = block.slot;
    transform_block_meta_to_database_changes(&mut database_changes, transactions, block_number);

    let spl_transactions = crate::spl_token_substream::parse_block(&block);
    crate::spl_token_substream::db::transform_block_meta_to_database_changes(&mut database_changes, spl_transactions.unwrap(), block_number);
    Ok(database_changes)
}
fn transform_block_meta_to_database_changes(
    changes: &mut DatabaseChanges,
    transactions: Vec<RaydiumAmmTransactionEvents>,
    block_number: u64,
) {
    
    let mut usd_swaps:Vec<SwapEvent> = vec![];
    for (i, transaction) in transactions.iter().enumerate() {
        let events: Vec<RaydiumAmmEvent> = transaction.events.clone();
        for (j, event) in events.iter().enumerate() {
            // add code here
            if let Some(inner_event) = &event.event {
                let block_time = transaction.block_time.clone();

                let signature = transaction.signature.clone();
                let transaction_index = transaction.transaction_index.clone();
                match inner_event {
                    raydium_amm_event::Event::Initialize(event_data) => {
                        push_initalize(
                            changes,
                            block_time,
                            signature,
                            transaction_index,
                            event_data.clone(),
                            i * j + j,
                            block_number,
                        );
                    }

                    raydium_amm_event::Event::Swap(event_data) => {
                        push_swap(
                            changes,
                            block_time,
                            signature,
                            transaction_index,
                            event_data.clone(),
                            i * j + j,
                            block_number,
                        );
                        let mint_in = event_data.mint_in.clone();
                        let mint_out = event_data.mint_out.clone();
                        if (
                            (
                            mint_in == String::from(USDT_ADDRESS) 
                            || mint_out == String::from(USDT_ADDRESS)
                            )
                            || (
                                mint_in == String::from(USDC_ADDRESS) 
                                || mint_out == String::from(USDC_ADDRESS)
                            )
                        ) && (
                            mint_in == String::from(pumpfun::constants::WSOL_ADDRESS) 
                            || mint_out == String::from(pumpfun::constants::WSOL_ADDRESS)
                        )
                         {
                            usd_swaps.push(event_data.clone());
                        }
                    }
                    raydium_amm_event::Event::Transfer(event_data) => {
                        push_transfer(
                            changes,
                            block_time,
                            signature,
                            transaction_index,
                            event_data.clone(),
                            i * j + j,
                            block_number,
                        );
                    }

                    raydium_amm_event::Event::Deposit(event_data) => {
                        push_deposit(
                            changes,
                            block_time,
                            signature,
                            transaction_index,
                            event_data.clone(),
                            i * j + j,
                            block_number,
                        );
                    }
                    raydium_amm_event::Event::Withdraw(event_data) => {
                        push_withdraw(
                            changes,
                            block_time,
                            signature,
                            transaction_index,
                            event_data.clone(),
                            i * j + j,
                            block_number,
                        );
                    }
                    raydium_amm_event::Event::WithdrawPnl(event_data) => {
                        push_withdraw_pnl(
                            changes,
                            block_time,
                            signature,
                            transaction_index,
                            event_data.clone(),
                            i * j + j,
                            block_number,
                        );
                    }
                    raydium_amm_event::Event::TransferWithSeed(event_data) => {
                        push_transfer_with_seed(
                            changes,
                            block_time,
                            signature,
                            transaction_index,
                            event_data.clone(),
                            i * j + j,
                            block_number,
                        );
                    }
                    raydium_amm_event::Event::PumpfunSwap(event_data) => {
                        push_transfer_pump_fun_swap(
                            changes,
                            block_time,
                            signature,
                            transaction_index,
                            event_data.clone(),
                            i * j + j,
                            block_number,
                        );
                    }
                    raydium_amm_event::Event::PumpfunWithdraw(event_data) => {
                        push_transfer_pump_fun_withdraw(
                            changes,
                            block_time,
                            signature,
                            transaction_index,
                            event_data.clone(),
                            i * j + j,
                            block_number,
                        );
                    }
                    raydium_amm_event::Event::PumpfunCreate(event_data) => {
                        push_transfer_pump_fun_create(
                            changes,
                            block_time,
                            signature,
                            transaction_index,
                            event_data.clone(),
                            i * j + j,
                            block_number,
                        );
                    }
                }
            }
        }
    }
    let mut usd_sum = 0;
    let mut usd_sol_vec : Vec<SwapUsdSol> = vec![];
    for (_i,swap) in usd_swaps.iter().enumerate(){
        let mut usd_sol : SwapUsdSol = SwapUsdSol{
           usd:0,
           sol:0,
           price:0.0
        };
        let direction = swap.direction.clone();
        if direction == String::from("coin"){
            usd_sol.usd = swap.amount_in*1000;
            usd_sol.sol = swap.amount_out;
            usd_sol.price =  (swap.amount_in * 1000) as f64/(swap.amount_out as f64)
        } else {
            usd_sol.usd = swap.amount_out*1000;
            usd_sol.sol = swap.amount_in;
            usd_sol.price =  (swap.amount_out * 1000) as f64/(swap.amount_in as f64);
        }
        usd_sum += usd_sol.usd;
        usd_sol_vec.push(usd_sol);
    }
    // filte usd

    if usd_sum > 100000 {
        let sol_price = calculate_sol_price(usd_sol_vec);
        if sol_price!=0.0{
            save_solana_block_sol_usd(block_number,sol_price,changes);
        }
    }

}

fn calculate_sol_price(usd_sol_vec:Vec<SwapUsdSol>) -> f64 {
    let mut price: f64 = 0.0;
    let mut sum_usd = 0;
    let mut sum_sol = 0;
    for (_i_,swap_usd_sol) in usd_sol_vec.iter().enumerate(){
        sum_usd += swap_usd_sol.usd;
        sum_sol += swap_usd_sol.sol;
    }
    if sum_sol!=0 {
        price = (sum_usd as f64)/(sum_sol as f64);
    }
    price
}

fn save_solana_block_sol_usd(
    block_number : u64,
    sol_price : f64,
    changes: &mut DatabaseChanges
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert(
        "id".to_string(),
        format!(
            "{}",block_number),
    );
    changes
        .push_change_composite("solana_block_sol_usd", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("price", (None,(sol_price * 1000000.0 )as u64))
        .change("price_text", (None,sol_price.to_string()));
}

fn push_transfer(
    changes: &mut DatabaseChanges,
    block_time: String,
    signature: String,
    transaction_index: String,
    event_data: TransferEvent,

    counter: usize,
    block_number: u64,
) {
    let pre_balance = match &event_data.funding_account_balance {
        Some(account_balance) => account_balance.pre_balance, // ✅ 直接使用引用
        None => AccountBalance::default().pre_balance,
    };
    let post_balance = match &event_data.funding_account_balance {
        Some(account_balance) => account_balance.post_balance,
        None => AccountBalance::default().post_balance,
    };
    let recipient_account_balance_pre_balance = match &event_data.recipient_account_balance {
        Some(pre_balance) => pre_balance.pre_balance,
        None => AccountBalance::default().pre_balance,
    };
    let recipient_account_balance_post_balance = match &event_data.recipient_account_balance {
        Some(balance) => balance.post_balance,
        None => AccountBalance::default().post_balance,
    };
    let mut composite_key = HashMap::new();
    composite_key.insert(
        "id".to_string(),
        format!(
            "{}_{}_{}_{}_{}_{}",
            signature,
            counter,
            transaction_index,
            block_number,
            event_data.funding_account,
            event_data.recipient_account
        ),
    );

    changes
        .push_change_composite(
            "solana_raydium_transfer",
            composite_key,
            1,
            Operation::Create,
        )
        .change("signature", (None, signature))
        .change("transaction_index", (None, transaction_index))
        .change("block_time", (None, block_time))
        .change("block_number", (None, block_number))
        .change("funding_account", (None, event_data.funding_account))
        .change("recipient_account", (None, event_data.recipient_account))
        .change("lamports", (None, event_data.lamports))
        .change("funding_account_balance_pre_balance", (None, pre_balance))
        .change("funding_account_balance_post_balance", (None, post_balance))
        .change(
            "recipient_account_balance_pre_balance",
            (None, recipient_account_balance_pre_balance),
        )
        .change(
            "recipient_account_balance_post_balance",
            (None, recipient_account_balance_post_balance),
        );
}
fn push_swap(
    changes: &mut DatabaseChanges,
    block_time: String,
    signature: String,
    transaction_index: String,
    event_data: SwapEvent,
    counter: usize,
    block_number: u64,
) {
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert(
        "id".to_string(),
        format!(
            "{}_{}_{}_{}_{}",
            signature, counter, transaction_index, block_number, event_data.user
        ),
    );

    changes
        .push_change_composite("solana_raydium_swap", composite_key, 1, Operation::Create)
        .change("signature", (None, signature))
        .change("transaction_index", (None, transaction_index))
        .change("block_time", (None, block_time))
        .change("block_number", (None, block_number))
        .change("amm", (None, event_data.amm))
        .change("user_swap", (None, event_data.user))
        .change("mint_in", (None, event_data.mint_in))
        .change("mint_out", (None, event_data.mint_out))
        .change("amount_in", (None, event_data.amount_in))
        .change("amount_out", (None, event_data.amount_out))
        .change("direction", (None, event_data.direction))
        .change("pool_pc_amount", (0, event_data.pool_pc_amount))
        .change("pool_coin_amount", (0, event_data.pool_coin_amount))
        .change("pc_mint", (None, event_data.pc_mint))
        .change("coin_mint", (None, event_data.coin_mint))
        .change("user_pre_balance_out", (0, event_data.user_pre_balance_out))
        .change("user_pre_balance_in", (0, event_data.user_pre_balance_in));
}

fn push_initalize(
    changes: &mut DatabaseChanges,
    block_time: String,
    signature: String,
    transaction_index: String,
    event_data: InitializeEvent,
    counter: usize,
    block_number: u64,
) {
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert(
        "id".to_string(),
        format!(
            "{}_{}_{}_{}_{}",
            signature, counter, transaction_index, block_number, event_data.user
        ),
    );
    changes
        .push_change_composite(
            "solana_raydium_initialize",
            composite_key,
            1,
            Operation::Create,
        )
        .change("signature", (None, signature))
        .change("transaction_index", (None, transaction_index))
        .change("block_time", (None, block_time))
        .change("block_number", (None, block_number))
        .change("amm", (None, event_data.amm))
        .change("initialize_user", (None, event_data.user))
        .change("pc_init_amount", (None, event_data.pc_init_amount))
        .change("coin_init_amount", (None, event_data.coin_init_amount))
        .change("lp_init_amount", (None, event_data.lp_init_amount))
        .change("pc_mint", (None, event_data.pc_mint))
        .change("coin_mint", (None, event_data.coin_mint))
        .change("lp_mint", (None, event_data.lp_mint))
        .change("nonce", (None, event_data.nonce))
        .change("market", (None, event_data.market.unwrap_or_default()))
        .change(
            "user_pc_pre_balance",
            (None, event_data.user_pc_pre_balance.unwrap_or_default()),
        )
        .change(
            "user_coin_pre_balance",
            (None, event_data.user_coin_pre_balance.unwrap_or_default()),
        );
}

fn push_deposit(
    changes: &mut DatabaseChanges,
    block_time: String,
    signature: String,
    transaction_index: String,
    event_data: DepositEvent,
    counter: usize,
    block_number: u64,
) {
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert(
        "id".to_string(),
        format!(
            "{}_{}_{}_{}_{}",
            signature, counter, transaction_index, block_number, event_data.user
        ),
    );
    changes
        .push_change_composite(
            "solana_raydium_deposite",
            composite_key,
            1,
            Operation::Create,
        )
        .change("signature", (None, signature))
        .change("transaction_index", (None, transaction_index))
        .change("block_time", (None, block_time))
        .change("block_number", (None, block_number))
        .change("amm", (None, event_data.amm))
        .change("deposite_user", (None, event_data.user))
        .change("pc_amount", (None, event_data.pc_amount))
        .change("coin_amount", (None, event_data.coin_amount))
        .change("lp_amount", (None, event_data.lp_amount))
        .change("pc_mint", (None, event_data.pc_mint))
        .change("coin_mint", (None, event_data.coin_mint))
        .change("lp_mint", (None, event_data.lp_mint))
        .change(
            "pool_pc_amount",
            (None, event_data.pool_pc_amount.unwrap_or_default()),
        )
        .change(
            "pool_coin_amount",
            (None, event_data.pool_coin_amount.unwrap_or_default()),
        )
        .change(
            "pool_lp_amount",
            (None, event_data.pool_lp_amount.unwrap_or_default()),
        )
        .change(
            "user_pc_pre_balance",
            (None, event_data.user_pc_pre_balance.unwrap_or_default()),
        )
        .change(
            "user_coin_pre_balance",
            (None, event_data.user_coin_pre_balance.unwrap_or_default()),
        );
}

fn push_withdraw(
    changes: &mut DatabaseChanges,
    block_time: String,
    signature: String,
    transaction_index: String,
    event_data: WithdrawEvent,
    counter: usize,
    block_number: u64,
) {
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert(
        "id".to_string(),
        format!(
            "{}_{}_{}_{}_{}",
            signature, counter, transaction_index, block_number, event_data.user
        ),
    );
    changes
        .push_change_composite(
            "solana_raydium_withdraw",
            composite_key,
            1,
            Operation::Create,
        )
        .change("signature", (None, signature))
        .change("transaction_index", (None, transaction_index))
        .change("block_time", (None, block_time))
        .change("block_number", (None, block_number))
        .change("amm", (None, event_data.amm))
        .change("withdraw_user", (None, event_data.user))
        .change("pc_amount", (None, event_data.pc_amount))
        .change("coin_amount", (None, event_data.coin_amount))
        .change("lp_amount", (None, event_data.lp_amount))
        .change("pc_mint", (None, event_data.pc_mint))
        .change("coin_mint", (None, event_data.coin_mint))
        .change("lp_mint", (None, event_data.lp_mint))
        .change(
            "pool_pc_amount",
            (None, event_data.pool_pc_amount.unwrap_or_default()),
        )
        .change(
            "pool_coin_amount",
            (None, event_data.pool_coin_amount.unwrap_or_default()),
        )
        .change(
            "pool_lp_amount",
            (None, event_data.pool_lp_amount.unwrap_or_default()),
        )
        .change(
            "user_pc_pre_balance",
            (None, event_data.user_pc_pre_balance.unwrap_or_default()),
        )
        .change(
            "user_coin_pre_balance",
            (None, event_data.user_coin_pre_balance.unwrap_or_default()),
        );
}

fn push_withdraw_pnl(
    changes: &mut DatabaseChanges,
    block_time: String,
    signature: String,
    transaction_index: String,
    event_data: WithdrawPnlEvent,
    counter: usize,
    block_number: u64,
) {
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert(
        "id".to_string(),
        format!(
            "{}_{}_{}_{}_{}",
            signature, counter, transaction_index, block_number, event_data.user
        ),
    );
    changes
        .push_change_composite(
            "solana_raydium_withdraw_pnl",
            composite_key,
            1,
            Operation::Create,
        )
        .change("signature", (None, signature))
        .change("transaction_index", (None, transaction_index))
        .change("block_time", (None, block_time))
        .change("block_number", (None, block_number))
        .change("amm", (None, event_data.amm))
        .change("withdraw_pnl_user", (None, event_data.user))
        .change(
            "pc_amount",
            (None, event_data.pc_amount.unwrap_or_default()),
        )
        .change(
            "coin_amount",
            (None, event_data.coin_amount.unwrap_or_default()),
        )
        .change("pc_mint", (None, event_data.pc_mint.unwrap_or_default()))
        .change(
            "coin_mint",
            (None, event_data.coin_mint.unwrap_or_default()),
        );
}

fn push_transfer_with_seed(
    changes: &mut DatabaseChanges,
    block_time: String,
    signature: String,
    transaction_index: String,
    event_data: TransferWithSeedEvent,
    counter: usize,
    block_number: u64,
) {
    let mut composite_key: HashMap<String, String> = HashMap::new();
    let funding_account_pre_balance = match &event_data.funding_account_balance {
        Some(account_balance) => account_balance.pre_balance, // ✅ 直接使用引用
        None => AccountBalance::default().pre_balance,
    };
    let funding_account_post_balance = match &event_data.funding_account_balance {
        Some(account_balance) => account_balance.post_balance,
        None => AccountBalance::default().post_balance,
    };
    let recipient_account_pre_balance = match &event_data.recipient_account_balance {
        Some(pre_balance) => pre_balance.pre_balance,
        None => AccountBalance::default().pre_balance,
    };
    let recipient_account_post_balance = match &event_data.recipient_account_balance {
        Some(balance) => balance.post_balance,
        None => AccountBalance::default().post_balance,
    };

    composite_key.insert(
        "id".to_string(),
        format!(
            "{}_{}_{}_{}_{}",
            signature, counter, transaction_index, block_number, event_data.funding_account
        ),
    );
    changes
        .push_change_composite(
            "solana_raydium_transfer_with_seed",
            composite_key,
            1,
            Operation::Create,
        )
        .change("signature", (None, signature))
        .change("transaction_index", (None, transaction_index))
        .change("block_time", (None, block_time))
        .change("block_number", (None, block_number))
        .change("funding_account", (None, event_data.funding_account))
        .change("base_account", (None, event_data.base_account))
        .change("recipient_account", (None, event_data.recipient_account))
        .change("lamports", (None, event_data.lamports))
        .change("from_seed", (None, event_data.from_seed))
        .change("from_owner", (None, event_data.from_owner))
        .change(
            "funding_account_pre_balance",
            (None, funding_account_pre_balance),
        )
        .change(
            "funding_account_post_balance",
            (None, funding_account_post_balance),
        )
        .change(
            "recipient_account_pre_balance",
            (None, recipient_account_pre_balance),
        )
        .change(
            "recipient_account_post_balance",
            (None, recipient_account_post_balance),
        );
}

fn push_transfer_pump_fun_swap(
    changes: &mut DatabaseChanges,
    block_time: String,
    signature: String,
    transaction_index: String,
    event_data: PumpfunSwapEvent,
    counter: usize,
    block_number: u64,
) {
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert(
        "id".to_string(),
        format!(
            "{}_{}_{}_{}_{}",
            signature, counter, transaction_index, block_number, event_data.user
        ),
    );
    changes
        .push_change_composite(
            "solana_raydium_pump_fun_swap",
            composite_key,
            1,
            Operation::Create,
        )
        .change("signature", (None, signature))
        .change("transaction_index", (None, transaction_index))
        .change("block_time", (None, block_time))
        .change("block_number", (None, block_number))
        .change("pump_fun_swap_user", (None, event_data.user))
        .change("mint", (None, event_data.mint))
        .change("bonding_curve", (None, event_data.bonding_curve))
        .change(
            "sol_amount",
            (None, event_data.sol_amount.unwrap_or_default()),
        )
        .change("token_amount", (None, event_data.token_amount))
        .change("direction", (None, event_data.direction))
        .change(
            "virtual_sol_reserves",
            (None, event_data.virtual_sol_reserves.unwrap_or_default()),
        )
        .change(
            "virtual_token_reserves",
            (None, event_data.virtual_token_reserves.unwrap_or_default()),
        )
        .change(
            "real_sol_reserves",
            (None, event_data.real_sol_reserves.unwrap_or_default()),
        )
        .change(
            "real_token_reserves",
            (None, event_data.real_token_reserves.unwrap_or_default()),
        )
        .change(
            "user_token_pre_balance",
            (None, event_data.user_token_pre_balance.unwrap_or_default()),
        );
}

fn push_transfer_pump_fun_withdraw(
    changes: &mut DatabaseChanges,
    block_time: String,
    signature: String,
    transaction_index: String,
    event_data: PumpfunWithdrawEvent,
    counter: usize,
    block_number: u64,
) {
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert(
        "id".to_string(),
        format!(
            "{}_{}_{}_{}",
            signature, counter, transaction_index, block_number
        ),
    );
    changes
        .push_change_composite(
            "solana_raydium_pump_fun_withdraw",
            composite_key,
            1,
            Operation::Create,
        )
        .change("signature", (None, signature))
        .change("transaction_index", (None, transaction_index))
        .change("block_time", (None, block_time))
        .change("block_number", (None, block_number))
        .change("mint", (None, event_data.mint));
}

fn push_transfer_pump_fun_create(
    changes: &mut DatabaseChanges,
    block_time: String,
    signature: String,
    transaction_index: String,
    event_data: PumpfunCreateEvent,
    counter: usize,
    block_number: u64,
) {
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert(
        "id".to_string(),
        format!(
            "{}_{}_{}_{}_{}",
            signature, counter, transaction_index, block_number, event_data.user
        ),
    );
    changes
        .push_change_composite(
            "solana_raydium_pump_fun_create",
            composite_key,
            1,
            Operation::Create,
        )
        .change("signature", (None, signature))
        .change("transaction_index", (None, transaction_index))
        .change("block_time", (None, block_time))
        .change("block_number", (None, block_number))
        .change("fun_name", (None, event_data.name))
        .change("symbol", (None, event_data.symbol))
        .change("uri", (None, event_data.uri))
        .change("mint", (None, event_data.mint))
        .change("bonding_curve", (None, event_data.bonding_curve))
        .change(
            "associated_bonding_curve",
            (None, event_data.associated_bonding_curve),
        )
        .change("metadata", (None, event_data.metadata));
}

pub fn parse_block(block: &Block) -> Vec<RaydiumAmmTransactionEvents> {
    let mut block_events: Vec<RaydiumAmmTransactionEvents> = Vec::new();
  
    let timestamp = block.block_time.as_ref().unwrap().timestamp;
    for (i, transaction) in block.transactions.iter().enumerate() {
        if let Ok(events) = parse_transaction(transaction) {
            if !events.is_empty() {
                block_events.push(RaydiumAmmTransactionEvents {
                    signature: utils::transaction::get_signature(&transaction),
                    events,
                    block_time: timestamp.to_string(),
                    transaction_index: i.to_string(),
                });
            }
        }
    }
    block_events
}

pub fn parse_transaction(
    transaction: &ConfirmedTransaction,
) -> Result<Vec<RaydiumAmmEvent>, Error> {
    if let Some(_) = transaction.meta.as_ref().unwrap().err {
        return Ok(Vec::new());
    }

    let mut events: Vec<RaydiumAmmEvent> = Vec::new();

    let mut context: TransactionContext<'_> = get_context(transaction)?;
    let instructions: Vec<std::rc::Rc<StructuredInstruction<'_>>> = get_structured_instructions(transaction)?;

    // 检查是否存在 DEX PROGRAM
    let contains_dex_program = instructions.flattened().iter().any(|instruction| {
        instruction.program_id() == RAYDIUM_AMM_PROGRAM_ID
            || instruction.program_id() == JUPITER_AGG_PROGRAM_ID
            || instruction.program_id() == PUMPFUN_PROGRAM_ID
    });

    for instruction in instructions.flattened().iter() {
        context.update_balance(&instruction.instruction);

        if instruction.program_id() == SYSTEM_PROGRAM_ID && !contains_dex_program {
            // 解析系统程序指令，并过滤掉返回值为 None 的情况
            match parse_system_program_instruction(instruction, &context) {
                Ok(Some(event)) => {
                    // 仅在返回有效事件时添加到结果中
                    events.push(RaydiumAmmEvent { event: Some(event) });
                }
                Ok(None) => {}
                Err(e) => {
                    return Err(anyhow!(
                        "Failed to parse SOL transfer {} with error: {}",
                        context.signature,
                        e
                    ))
                }
            }
        }

        // PUMPFUN
        if instruction.program_id() == PUMPFUN_PROGRAM_ID {
            match parse_pumpfun_instruction(&instruction, &context) {
                Ok(Some(event)) => events.push(RaydiumAmmEvent { event: Some(event) }),
                Ok(None) => (),
                Err(e) => {
                    return Err(anyhow!(
                        "Failed to parse Pumpfun transaction {} with error: {}",
                        context.signature,
                        e
                    ))
                }
            }
        }

        if instruction.program_id() == RAYDIUM_AMM_PROGRAM_ID {
            match parse_instruction(&instruction, &context) {
                Ok(Some(event)) => events.push(RaydiumAmmEvent { event: Some(event) }),
                Ok(None) => (),
                Err(error) => substreams::log::println(format!(
                    "Failed to process instruction of transaction {}: {}",
                    &context.signature, error
                )),
            }
        }
    }
    Ok(events)
}

pub fn parse_system_program_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<Option<Event>, Error> {
    if instruction.program_id() != SYSTEM_PROGRAM_ID {
        return Err(anyhow!("Not a System Program instruction."));
    }
    let unpacked: SystemInstruction = SystemInstruction::unpack(&instruction.data())?;
    match unpacked {
        SystemInstruction::CreateAccount(_create_account) => Ok(None),
        SystemInstruction::Assign(_assign) => Ok(None),
        SystemInstruction::Transfer(transfer) => {
            if transfer.lamports < SOL_MINIMUM_LAMPORTS {
                return Ok(None);
            }
            _parse_transfer_instruction(instruction, context, &transfer)
                .map(|x| Some(Event::Transfer(x)))
        }
        SystemInstruction::CreateAccountWithSeed(_create_account_with_seed) => Ok(None),
        SystemInstruction::AdvanceNonceAccount => Ok(None),
        SystemInstruction::WithdrawNonceAccount(_lamports) => Ok(None),
        SystemInstruction::InitializeNonceAccount(_pubkey) => Ok(None),
        SystemInstruction::AuthorizeNonceAccount(_pubkey) => Ok(None),
        SystemInstruction::Allocate(_allocate) => Ok(None),
        SystemInstruction::AllocateWithSeed(_allocate_with_seed) => Ok(None),
        SystemInstruction::AssignWithSeed(_assign_with_seed) => Ok(None),
        SystemInstruction::TransferWithSeed(transfer_with_seed) => {
            _parse_transfer_with_seed_instruction(instruction, context, transfer_with_seed)
                .map(|x| Some(Event::TransferWithSeed(x)))
        }
        SystemInstruction::UpgradeNonceAccount => Ok(None),
    }
    .context("Failed to parse System instruction")
}

pub fn parse_pumpfun_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<Option<Event>, Error> {
    if instruction.program_id() != PUMPFUN_PROGRAM_ID {
        return Err(anyhow!("Not a Pumpfun instruction."));
    }

    let unpacked = PumpfunInstruction::unpack(instruction.data()).map_err(|x| anyhow!(x))?;
    match unpacked {
        PumpfunInstruction::Initialize => Ok(None),
        PumpfunInstruction::SetParams(_) => Ok(None),
        PumpfunInstruction::Create(create) => Ok(Some(Event::PumpfunCreate(
            _parse_pumpfun_create_instruction(instruction, context, create)?,
        ))),
        PumpfunInstruction::Buy(buy) => Ok(Some(Event::PumpfunSwap(
            _parse_pumpfun_buy_instruction(instruction, context, buy)?,
        ))),
        PumpfunInstruction::Sell(sell) => Ok(Some(Event::PumpfunSwap(
            _parse_pumpfun_sell_instruction(instruction, context, sell)?,
        ))),
        PumpfunInstruction::Withdraw => Ok(Some(Event::PumpfunWithdraw(
            _parse_pumpfun_withdraw_instruction(instruction, context)?,
        ))),
        _ => Ok(None),
    }
}

pub fn is_spl_token_address(context: &TransactionContext, address: &str) -> bool {
    let program_id_str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

    let account_pk = Pubkey::from_string(&address);
    let account_ref = PubkeyRef {
        0: &account_pk.0.to_vec(),
    };
    if let Some(token_account) = context.get_token_account(&account_ref) {
        let owner = token_account.owner.to_string();
        // 使用 owner 进行处理
        return owner == program_id_str;
    }

    return false;
}

pub fn parse_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<Option<Event>, String> {
    if instruction.program_id() != RAYDIUM_AMM_PROGRAM_ID {
        return Err("Instruction does not originate from Raydium AMM Program.".into());
    }
    let unpacked = AmmInstruction::unpack(&instruction.data())?;
    match unpacked {
        AmmInstruction::SwapBaseIn(_) | AmmInstruction::SwapBaseOut(_) => {
            let event = _parse_swap_instruction(instruction, context)?;
            Ok(Some(Event::Swap(event)))
        }
        AmmInstruction::Initialize2(initialize) => {
            let event = _parse_initialize_instruction(instruction, context, initialize.nonce)?;
            Ok(Some(Event::Initialize(event)))
        }
        AmmInstruction::Deposit(_deposit) => {
            let event = _parse_deposit_instruction(instruction, context)?;
            Ok(Some(Event::Deposit(event)))
        }
        AmmInstruction::Withdraw(_withdraw) => {
            let event = _parse_withdraw_instruction(instruction, context)?;
            Ok(Some(Event::Withdraw(event)))
        }
        AmmInstruction::WithdrawPnl => {
            let event = _parse_withdraw_pnl_instruction(instruction, context)?;
            Ok(Some(Event::WithdrawPnl(event)))
        }
        _ => Ok(None),
    }
}

fn _parse_swap_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<SwapEvent, String> {
    let amm = instruction.accounts()[1].to_string();
    let user = instruction.accounts().last().unwrap().to_string();

    let instructions_len = instruction.inner_instructions().len();
    let transfer_in = spl_token_substream::parse_transfer_instruction(
        &instruction.inner_instructions()[instructions_len - 2],
        context,
    )?;
    let transfer_out = spl_token_substream::parse_transfer_instruction(
        &instruction.inner_instructions()[instructions_len - 1],
        context,
    )?;

    let amount_in = transfer_in.amount;
    let amount_out = transfer_out.amount;
    let mint_in = transfer_in.source.as_ref().unwrap().mint.clone();
    let mint_out = transfer_out.source.as_ref().unwrap().mint.clone();

    let user_pre_balance_in = transfer_in.source.unwrap().pre_balance;
    let user_pre_balance_out = transfer_out.destination.unwrap().pre_balance;

    let delta = if instruction.accounts().len() == 17 {
        0
    } else {
        1
    };
    let coin_mint = context
        .get_token_account(&instruction.accounts()[4 + delta])
        .unwrap()
        .mint
        .to_string();
    let pc_mint = context
        .get_token_account(&instruction.accounts()[5 + delta])
        .unwrap()
        .mint
        .to_string();

    let direction = (if mint_out == coin_mint { "coin" } else { "pc" }).to_string();

    let (pool_coin_amount, pool_pc_amount) = match parse_raydium_log(instruction) {
        Ok(RayLog::SwapBaseIn(swap_base_in)) => {
            (Some(swap_base_in.pool_coin), Some(swap_base_in.pool_pc))
        }
        Ok(RayLog::SwapBaseOut(swap_base_out)) => {
            (Some(swap_base_out.pool_coin), Some(swap_base_out.pool_pc))
        }
        _ => (None, None),
    };

    Ok(SwapEvent {
        amm,
        user,
        mint_in,
        mint_out,
        amount_in,
        amount_out,
        direction,
        pool_coin_amount,
        pool_pc_amount,
        coin_mint,
        pc_mint,
        user_pre_balance_in,
        user_pre_balance_out,
    })
}

fn _parse_initialize_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    context: &TransactionContext,
    nonce: u8,
) -> Result<InitializeEvent, String> {
    let amm = instruction.accounts()[4].to_string();
    let user = instruction.accounts()[17].to_string();

    let instructions_len = instruction.inner_instructions().len();
    let coin_transfer = spl_token_substream::parse_transfer_instruction(
        &instruction.inner_instructions()[instructions_len - 3],
        context,
    )?;
    let pc_transfer = spl_token_substream::parse_transfer_instruction(
        &instruction.inner_instructions()[instructions_len - 2],
        context,
    )?;
    let lp_mint_to = spl_token_substream::parse_mint_to_instruction(
        &instruction.inner_instructions()[instructions_len - 1],
        context,
    )?;

    let pc_init_amount = pc_transfer.amount;
    let coin_init_amount = coin_transfer.amount;
    let lp_init_amount = lp_mint_to.amount;
    let pc_mint = pc_transfer.source.as_ref().unwrap().mint.clone();
    let coin_mint = coin_transfer.source.as_ref().unwrap().mint.clone();
    let lp_mint = lp_mint_to.mint;

    let user_pc_pre_balance = pc_transfer.source.unwrap().pre_balance;
    let user_coin_pre_balance = coin_transfer.source.unwrap().pre_balance;

    let market = match parse_raydium_log(instruction) {
        Ok(RayLog::Init(init)) => Some(Pubkey(init.market).to_string()),
        _ => None,
    };

    Ok(InitializeEvent {
        amm,
        user,
        pc_init_amount,
        coin_init_amount,
        lp_init_amount,
        pc_mint,
        coin_mint,
        lp_mint,
        nonce: nonce as u32,
        market,
        user_pc_pre_balance,
        user_coin_pre_balance,
    })
}

fn _parse_deposit_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<DepositEvent, String> {
    let amm = instruction.accounts()[1].to_string();
    let user = instruction.accounts()[12].to_string();

    let instructions_len = instruction.inner_instructions().len();
    let pc_transfer = spl_token_substream::parse_transfer_instruction(
        &instruction.inner_instructions()[instructions_len - 2],
        context,
    )?;
    let coin_transfer = spl_token_substream::parse_transfer_instruction(
        &instruction.inner_instructions()[instructions_len - 3],
        context,
    )?;
    let lp_mint_to = spl_token_substream::parse_mint_to_instruction(
        &instruction.inner_instructions()[instructions_len - 1],
        context,
    )?;

    let pc_amount = pc_transfer.amount;
    let coin_amount = coin_transfer.amount;
    let lp_amount = lp_mint_to.amount;
    let pc_mint = pc_transfer.source.as_ref().unwrap().mint.clone();
    let coin_mint = coin_transfer.source.as_ref().unwrap().mint.clone();
    let lp_mint = lp_mint_to.mint;

    let user_pc_pre_balance = pc_transfer.source.unwrap().pre_balance;
    let user_coin_pre_balance = coin_transfer.source.unwrap().pre_balance;

    let (pool_pc_amount, pool_coin_amount, pool_lp_amount) = match parse_raydium_log(instruction) {
        Ok(RayLog::Deposit(deposit)) => (
            Some(deposit.pool_pc),
            Some(deposit.pool_coin),
            Some(deposit.pool_lp),
        ),
        _ => (None, None, None),
    };

    Ok(DepositEvent {
        amm,
        user,
        pc_amount,
        coin_amount,
        lp_amount,
        pc_mint,
        coin_mint,
        lp_mint,
        pool_pc_amount,
        pool_coin_amount,
        pool_lp_amount,
        user_pc_pre_balance,
        user_coin_pre_balance,
    })
}

fn _parse_withdraw_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<WithdrawEvent, String> {
    let amm = instruction.accounts()[1].to_string();
    let user = instruction.accounts()[16].to_string();

    let instructions_len = instruction.inner_instructions().len();
    let pc_transfer = spl_token_substream::parse_transfer_instruction(
        &instruction.inner_instructions()[instructions_len - 2],
        context,
    )?;
    let coin_transfer = spl_token_substream::parse_transfer_instruction(
        &instruction.inner_instructions()[instructions_len - 3],
        context,
    )?;
    let lp_burn = spl_token_substream::parse_burn_instruction(
        &instruction.inner_instructions()[instructions_len - 1],
        context,
    )?;

    let pc_amount = pc_transfer.amount;
    let coin_amount = coin_transfer.amount;
    let lp_amount = lp_burn.amount;
    let pc_mint = pc_transfer.source.unwrap().mint;
    let coin_mint = coin_transfer.source.unwrap().mint;
    let lp_mint = lp_burn.source.unwrap().mint;

    let user_pc_pre_balance = pc_transfer.destination.unwrap().pre_balance;
    let user_coin_pre_balance = coin_transfer.destination.unwrap().pre_balance;

    let (pool_pc_amount, pool_coin_amount, pool_lp_amount) = match parse_raydium_log(instruction) {
        Ok(RayLog::Withdraw(withdraw)) => (
            Some(withdraw.pool_pc),
            Some(withdraw.pool_coin),
            Some(withdraw.pool_lp),
        ),
        _ => (None, None, None),
    };

    Ok(WithdrawEvent {
        amm,
        user,
        pc_amount,
        coin_amount,
        lp_amount,
        pc_mint,
        coin_mint,
        lp_mint,
        pool_pc_amount,
        pool_coin_amount,
        pool_lp_amount,
        user_pc_pre_balance,
        user_coin_pre_balance,
    })
}

fn _parse_withdraw_pnl_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<WithdrawPnlEvent, String> {
    let amm = instruction.accounts()[1].to_string();
    let user = instruction.accounts()[9].to_string();

    let instructions_len = instruction.inner_instructions().len();
    if instructions_len == 2 || instructions_len == 3 {
        let pc_transfer = spl_token_substream::parse_transfer_instruction(
            &instruction.inner_instructions()[instructions_len - 1],
            context,
        )?;
        let coin_transfer = spl_token_substream::parse_transfer_instruction(
            &instruction.inner_instructions()[instructions_len - 2],
            context,
        )?;

        let pc_amount = Some(pc_transfer.amount);
        let coin_amount = Some(coin_transfer.amount);
        let pc_mint = Some(pc_transfer.source.unwrap().mint);
        let coin_mint = Some(coin_transfer.source.unwrap().mint);

        return Ok(WithdrawPnlEvent {
            amm,
            user,
            pc_amount,
            coin_amount,
            pc_mint,
            coin_mint,
        });
    } else {
        return Ok(WithdrawPnlEvent {
            amm,
            user,
            pc_amount: None,
            coin_amount: None,
            pc_mint: None,
            coin_mint: None,
        });
    }
}

fn _parse_transfer_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
    transfer: &system_program::Transfer,
) -> Result<TransferEvent, Error> {
    let funding_account = instruction.accounts()[0].to_string();
    let recipient_account = instruction.accounts()[1].to_string();
    let lamports = transfer.lamports;
    let funding_account_balance = context
        .account_balances
        .get(instruction.instruction.accounts()[0] as usize)
        .map(|x| x.clone().into());
    let recipient_account_balance = context
        .account_balances
        .get(instruction.instruction.accounts()[1] as usize)
        .map(|x| x.clone().into());

    Ok(TransferEvent {
        funding_account,
        recipient_account,
        lamports,
        funding_account_balance,
        recipient_account_balance,
    })
}

fn _parse_transfer_with_seed_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
    transfer_with_seed: system_program::TransferWithSeed,
) -> Result<TransferWithSeedEvent, Error> {
    let funding_account = instruction.accounts()[0].to_string();
    let base_account = instruction.accounts()[1].to_string();
    let recipient_account = instruction.accounts()[2].to_string();
    let from_owner = transfer_with_seed.from_owner.to_string();
    let from_seed = transfer_with_seed.from_seed.0.clone();
    let lamports = transfer_with_seed.lamports;
    let funding_account_balance = context
        .account_balances
        .get(instruction.instruction.accounts()[0] as usize)
        .map(|x| x.clone().into());
    let recipient_account_balance = context
        .account_balances
        .get(instruction.instruction.accounts()[1] as usize)
        .map(|x| x.clone().into());

    Ok(TransferWithSeedEvent {
        funding_account,
        base_account,
        recipient_account,
        from_owner,
        from_seed,
        lamports,
        funding_account_balance,
        recipient_account_balance,
    })
}

pub fn _parse_pumpfun_create_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
    create: pumpfun::instruction::CreateInstruction,
) -> Result<PumpfunCreateEvent, Error> {
    let user = instruction.accounts()[7].to_string();
    let name = create.name;
    let symbol = create.symbol;
    let uri = create.uri;
    let mint = instruction.accounts()[0].to_string();
    let bonding_curve = instruction.accounts()[2].to_string();
    let associated_bonding_curve = instruction.accounts()[2].to_string();
    let metadata = instruction.accounts()[6].to_string();

    Ok(PumpfunCreateEvent {
        user,
        name,
        symbol,
        uri,
        mint,
        bonding_curve,
        associated_bonding_curve,
        metadata,
    })
}

pub fn _parse_pumpfun_buy_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    context: &TransactionContext,
    buy: pumpfun::instruction::BuyInstruction,
) -> Result<PumpfunSwapEvent, Error> {
    let mint = instruction.accounts()[2].to_string();
    let bonding_curve = instruction.accounts()[3].to_string();
    let user = instruction.accounts()[6].to_string();
    let token_amount = buy.amount;

    let system_transfer_instruction = instruction
        .inner_instructions()
        .iter()
        .find(|x| x.program_id() == SYSTEM_PROGRAM_ID)
        .unwrap()
        .clone();
    let system_transfer = system_program_substream::parse_transfer_instruction(
        system_transfer_instruction.as_ref(),
        context,
    )?;
    let sol_amount = Some(system_transfer.lamports);

    let token_transfer_instruction = instruction
        .inner_instructions()
        .iter()
        .find(|x| x.program_id() == TOKEN_PROGRAM_ID)
        .unwrap()
        .clone();
    let token_transfer = spl_token_substream::parse_transfer_instruction(
        token_transfer_instruction.as_ref(),
        context,
    )
    .map_err(|e| anyhow!(e))?;
    let user_token_pre_balance = token_transfer.destination.unwrap().pre_balance;

    let trade = match parse_pumpfun_log(instruction) {
        Ok(PumpfunLog::Trade(trade)) => Some(trade),
        _ => None,
    };
    let virtual_sol_reserves = trade.as_ref().map(|x| x.virtual_sol_reserves);
    let virtual_token_reserves = trade.as_ref().map(|x| x.virtual_token_reserves);
    let real_sol_reserves = trade.as_ref().map(|x| x.real_sol_reserves);
    let real_token_reserves = trade.as_ref().map(|x| x.real_token_reserves);

    let direction = "token".to_string();

    Ok(PumpfunSwapEvent {
        user,
        mint,
        bonding_curve,
        sol_amount,
        token_amount,
        direction,
        virtual_sol_reserves,
        virtual_token_reserves,
        real_sol_reserves,
        real_token_reserves,
        user_token_pre_balance,
    })
}

pub fn _parse_pumpfun_sell_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
    sell: pumpfun::instruction::SellInstruction,
) -> Result<PumpfunSwapEvent, Error> {
    let mint = instruction.accounts()[2].to_string();
    let user = instruction.accounts()[6].to_string();
    let bonding_curve = instruction.accounts()[3].to_string();
    let token_amount = sell.amount;

    let trade = match parse_pumpfun_log(instruction) {
        Ok(PumpfunLog::Trade(trade)) => Some(trade),
        _ => None,
    };
    let sol_amount = trade.as_ref().map(|x| x.sol_amount);
    let virtual_sol_reserves = trade.as_ref().map(|x| x.virtual_sol_reserves);
    let virtual_token_reserves = trade.as_ref().map(|x| x.virtual_token_reserves);
    let real_sol_reserves = trade.as_ref().map(|x| x.real_sol_reserves);
    let real_token_reserves = trade.as_ref().map(|x| x.real_token_reserves);

    let direction = "sol".to_string();

    let token_transfer_instruction = instruction
        .inner_instructions()
        .iter()
        .find(|x| x.program_id() == TOKEN_PROGRAM_ID)
        .unwrap()
        .clone();
    let token_transfer = spl_token_substream::parse_transfer_instruction(
        token_transfer_instruction.as_ref(),
        context,
    )
    .map_err(|e| anyhow!(e))?;
    let user_token_pre_balance = token_transfer.source.unwrap().pre_balance;

    Ok(PumpfunSwapEvent {
        user,
        mint,
        bonding_curve,
        token_amount,
        sol_amount,
        direction,
        virtual_sol_reserves,
        virtual_token_reserves,
        real_sol_reserves,
        real_token_reserves,
        user_token_pre_balance,
    })
}

pub fn _parse_pumpfun_withdraw_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
) -> Result<PumpfunWithdrawEvent, Error> {
    let mint = instruction.accounts()[2].to_string();

    Ok(PumpfunWithdrawEvent { mint })
}

fn parse_raydium_log(instruction: &StructuredInstruction) -> Result<RayLog, Error> {
    let re = regex::Regex::new(r"ray_log: (.+)").unwrap();
    let log_message = instruction
        .logs()
        .as_ref()
        .context("Failed to parse logs due to truncation")?
        .iter()
        .rev()
        .find_map(|log| {
            if let Log::Program(program_log) = log {
                Some(program_log.message().unwrap())
            } else {
                None
            }
        });
    match log_message {
        Some(message) => match re.captures(message.as_str()) {
            Some(captures) => Ok(decode_ray_log(&captures[1])),
            None => return Err(anyhow!("Failed to capture log message")),
        },
        None => return Err(anyhow!("Log message not found")),
    }
}

fn parse_pumpfun_log(instruction: &StructuredInstruction) -> Result<PumpfunLog, Error> {
    let data = instruction
        .logs()
        .as_ref()
        .context("Failed to parse logs due to truncation")?
        .iter()
        .find_map(|log| match log {
            Log::Data(data_log) => data_log.data().ok(),
            _ => None,
        })
        .ok_or(anyhow!("Couldn't find data log."))?;
    PumpfunLog::unpack(data.as_slice()).map_err(|x| anyhow!(x))
}

impl From<utils::account::AccountBalance> for AccountBalance {
    fn from(value: utils::account::AccountBalance) -> Self {
        Self {
            pre_balance: value.pre_balance,
            post_balance: value.post_balance,
        }
    }
}
