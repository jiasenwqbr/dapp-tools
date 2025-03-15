use regex;
use anyhow::{anyhow, Error, Context};

use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

pub mod raydium_amm;
use raydium_amm::instruction::AmmInstruction;
use raydium_amm::constants::RAYDIUM_AMM_PROGRAM_ID;
use raydium_amm::constants::JUPITER_AGG_PROGRAM_ID;
use raydium_amm::constants::SOL_MINIMUM_LAMPORTS;
use raydium_amm::log::{decode_ray_log, RayLog};

use substreams_solana_utils::system_program;
use substreams_solana_utils as utils;
use substreams_solana_utils::pubkey::PubkeyRef;
use utils::instruction::{get_structured_instructions, StructuredInstruction, StructuredInstructions};
use utils::transaction::{get_context, TransactionContext};
use utils::pubkey::Pubkey;
use utils::log::Log;
use utils::system_program::{SystemInstruction, SYSTEM_PROGRAM_ID};
use substreams_solana_utils::spl_token::TOKEN_PROGRAM_ID;

use spl_token_substream;

pub mod pumpfun;
use pumpfun::instruction::PumpfunInstruction;
use pumpfun::constants::PUMPFUN_PROGRAM_ID;
use pumpfun::log::PumpfunLog;

use system_program_substream;

pub mod pb;
use pb::raydium_amm::*;
use pb::raydium_amm::raydium_amm_event::Event;

#[substreams::handlers::map]
fn raydium_amm_events(block: Block) -> Result<RaydiumAmmBlockEvents, Error> {
    let transactions = parse_block(&block);
    Ok(RaydiumAmmBlockEvents { transactions })
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

pub fn parse_transaction(transaction: &ConfirmedTransaction) -> Result<Vec<RaydiumAmmEvent>, Error> {
    if let Some(_) = transaction.meta.as_ref().unwrap().err {
        return Ok(Vec::new());
    }

    let mut events: Vec<RaydiumAmmEvent> = Vec::new();

    let mut context = get_context(transaction)?;
    let instructions = get_structured_instructions(transaction)?;

    // 检查是否存在 DEX PROGRAM
    let contains_dex_program = instructions.flattened().iter().any(|instruction| 
        instruction.program_id() == RAYDIUM_AMM_PROGRAM_ID 
        || instruction.program_id() == JUPITER_AGG_PROGRAM_ID 
        || instruction.program_id() == PUMPFUN_PROGRAM_ID);
    
    for instruction in instructions.flattened().iter() {
        context.update_balance(&instruction.instruction);

        if instruction.program_id() == SYSTEM_PROGRAM_ID && !contains_dex_program {
            // 解析系统程序指令，并过滤掉返回值为 None 的情况
            match parse_system_program_instruction(instruction, &context) {
                Ok(Some(event)) => {
                    // 仅在返回有效事件时添加到结果中
                    events.push(RaydiumAmmEvent { event: Some(event) });
                },
                Ok(None) => {},
                Err(e) => return Err(anyhow!("Failed to parse SOL transfer {} with error: {}", context.signature, e))
            }
        }

        // PUMPFUN 
        if instruction.program_id() == PUMPFUN_PROGRAM_ID {
            match parse_pumpfun_instruction(&instruction, &context) {
                Ok(Some(event)) => {
                    events.push(RaydiumAmmEvent {
                        event: Some(event),
                    })
                }
                Ok(None) => (),
                Err(e) => return Err(anyhow!("Failed to parse Pumpfun transaction {} with error: {}", context.signature, e))
            }
        }
        
        if instruction.program_id() == RAYDIUM_AMM_PROGRAM_ID {
            match parse_instruction(&instruction, &context) {
                Ok(Some(event)) => {
                    events.push(RaydiumAmmEvent {
                        event: Some(event),
                    })
                }
                Ok(None) => (),
                Err(error) => substreams::log::println(format!("Failed to process instruction of transaction {}: {}", &context.signature, error))
            }
        }

    }
    Ok(events)
}

pub fn parse_system_program_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    context: &TransactionContext
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
            _parse_transfer_instruction(instruction, context, &transfer).map(|x| Some(Event::Transfer(x)))
        },
        SystemInstruction::CreateAccountWithSeed(_create_account_with_seed) => Ok(None),
        SystemInstruction::AdvanceNonceAccount => Ok(None),
        SystemInstruction::WithdrawNonceAccount(_lamports) => Ok(None),
        SystemInstruction::InitializeNonceAccount(_pubkey) => Ok(None),
        SystemInstruction::AuthorizeNonceAccount(_pubkey) => Ok(None),
        SystemInstruction::Allocate(_allocate) => Ok(None),
        SystemInstruction::AllocateWithSeed(_allocate_with_seed) => Ok(None),
        SystemInstruction::AssignWithSeed(_assign_with_seed) => Ok(None),
        SystemInstruction::TransferWithSeed(transfer_with_seed) => {
            _parse_transfer_with_seed_instruction(instruction, context, transfer_with_seed).map(|x| Some(Event::TransferWithSeed(x)))
        },
        SystemInstruction::UpgradeNonceAccount => Ok(None)
    }.context("Failed to parse System instruction")
}

pub fn parse_pumpfun_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext
) -> Result<Option<Event>, Error> {
    if instruction.program_id() != PUMPFUN_PROGRAM_ID {
        return Err(anyhow!("Not a Pumpfun instruction."));
    }
    
    let unpacked = PumpfunInstruction::unpack(instruction.data()).map_err(|x| anyhow!(x))?;
    match unpacked {
        PumpfunInstruction::Initialize => Ok(None),
        PumpfunInstruction::SetParams(_) => Ok(None),
        PumpfunInstruction::Create(create) => {
            Ok(Some(Event::PumpfunCreate(_parse_pumpfun_create_instruction(instruction, context, create)?)))
        },
        PumpfunInstruction::Buy(buy) => {
            Ok(Some(Event::PumpfunSwap(_parse_pumpfun_buy_instruction(instruction, context, buy)?)))
        }
        PumpfunInstruction::Sell(sell) => {
            Ok(Some(Event::PumpfunSwap(_parse_pumpfun_sell_instruction(instruction, context, sell)?)))
        }
        PumpfunInstruction::Withdraw => {
            Ok(Some(Event::PumpfunWithdraw(_parse_pumpfun_withdraw_instruction(instruction, context)?)))
        }
        _ => Ok(None),
    }
}

pub fn is_spl_token_address(context: &TransactionContext, address: &str) -> bool {
    let program_id_str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

    let account_pk = Pubkey::from_string(&address);
    let account_ref = PubkeyRef { 0: &account_pk.0.to_vec() };
    if let Some(token_account) = context.get_token_account(&account_ref) {
        let owner = token_account.owner.to_string();
        // 使用 owner 进行处理
        return owner == program_id_str;
    } 
    
    return false;
}

pub fn parse_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    context: &TransactionContext
) -> Result<Option<Event>, String> {
    if instruction.program_id() != RAYDIUM_AMM_PROGRAM_ID {
        return Err("Instruction does not originate from Raydium AMM Program.".into());
    }
    let unpacked = AmmInstruction::unpack(&instruction.data())?;
    match unpacked {
        AmmInstruction::SwapBaseIn(_) |
        AmmInstruction::SwapBaseOut(_) => {
            let event = _parse_swap_instruction(instruction, context)?;
            Ok(Some(Event::Swap(event)))
        },
        AmmInstruction::Initialize2(initialize) => {
            let event = _parse_initialize_instruction(instruction, context, initialize.nonce)?;
            Ok(Some(Event::Initialize(event)))
        },
        AmmInstruction::Deposit(_deposit) => {
            let event = _parse_deposit_instruction(instruction, context)?;
            Ok(Some(Event::Deposit(event)))
        },
        AmmInstruction::Withdraw(_withdraw) => {
            let event = _parse_withdraw_instruction(instruction, context)?;
            Ok(Some(Event::Withdraw(event)))
        },
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
    let transfer_in = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 2], context)?;
    let transfer_out = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 1], context)?;

    let amount_in = transfer_in.amount;
    let amount_out = transfer_out.amount;
    let mint_in = transfer_in.source.as_ref().unwrap().mint.clone();
    let mint_out = transfer_out.source.as_ref().unwrap().mint.clone();

    let user_pre_balance_in = transfer_in.source.unwrap().pre_balance;
    let user_pre_balance_out = transfer_out.destination.unwrap().pre_balance;

    let delta = if instruction.accounts().len() == 17 { 0 } else { 1 };
    let coin_mint = context.get_token_account(&instruction.accounts()[4 + delta]).unwrap().mint.to_string();
    let pc_mint = context.get_token_account(&instruction.accounts()[5 + delta]).unwrap().mint.to_string();

    let direction = (if mint_out == coin_mint { "coin" } else { "pc" }).to_string();

    let (pool_coin_amount, pool_pc_amount) = match parse_raydium_log(instruction) {
        Ok(RayLog::SwapBaseIn(swap_base_in)) => {
            (Some(swap_base_in.pool_coin), Some(swap_base_in.pool_pc))
        },
        Ok(RayLog::SwapBaseOut(swap_base_out)) => {
            (Some(swap_base_out.pool_coin), Some(swap_base_out.pool_pc))
        },
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
    let coin_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 3], context)?;
    let pc_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 2], context)?;
    let lp_mint_to = spl_token_substream::parse_mint_to_instruction(&instruction.inner_instructions()[instructions_len - 1], context)?;

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
    context: &TransactionContext
) -> Result<DepositEvent, String> {
    let amm = instruction.accounts()[1].to_string();
    let user = instruction.accounts()[12].to_string();

    let instructions_len = instruction.inner_instructions().len();
    let pc_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 2], context)?;
    let coin_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 3], context)?;
    let lp_mint_to = spl_token_substream::parse_mint_to_instruction(&instruction.inner_instructions()[instructions_len - 1], context)?;

    let pc_amount = pc_transfer.amount;
    let coin_amount = coin_transfer.amount;
    let lp_amount = lp_mint_to.amount;
    let pc_mint = pc_transfer.source.as_ref().unwrap().mint.clone();
    let coin_mint = coin_transfer.source.as_ref().unwrap().mint.clone();
    let lp_mint = lp_mint_to.mint;

    let user_pc_pre_balance = pc_transfer.source.unwrap().pre_balance;
    let user_coin_pre_balance = coin_transfer.source.unwrap().pre_balance;

    let (pool_pc_amount, pool_coin_amount, pool_lp_amount) = match parse_raydium_log(instruction) {
        Ok(RayLog::Deposit(deposit)) => {
            (Some(deposit.pool_pc), Some(deposit.pool_coin), Some(deposit.pool_lp))
        },
        _ => (None, None, None)
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
    let pc_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 2], context)?;
    let coin_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 3], context)?;
    let lp_burn = spl_token_substream::parse_burn_instruction(&instruction.inner_instructions()[instructions_len - 1], context)?;

    let pc_amount = pc_transfer.amount;
    let coin_amount = coin_transfer.amount;
    let lp_amount = lp_burn.amount;
    let pc_mint = pc_transfer.source.unwrap().mint;
    let coin_mint = coin_transfer.source.unwrap().mint;
    let lp_mint = lp_burn.source.unwrap().mint;

    let user_pc_pre_balance = pc_transfer.destination.unwrap().pre_balance;
    let user_coin_pre_balance = coin_transfer.destination.unwrap().pre_balance;

    let (pool_pc_amount, pool_coin_amount, pool_lp_amount) = match parse_raydium_log(instruction) {
        Ok(RayLog::Withdraw(withdraw)) => {
            (Some(withdraw.pool_pc), Some(withdraw.pool_coin), Some(withdraw.pool_lp))
        },
        _ => (None, None, None)
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
        let pc_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 1], context)?;
        let coin_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 2], context)?;

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
            coin_mint
        });
    } else {
        return Ok(WithdrawPnlEvent {
            amm,
            user,
            pc_amount: None,
            coin_amount: None,
            pc_mint: None,
            coin_mint: None,
        })
    }
}

fn _parse_transfer_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
    transfer: &system_program::Transfer
) -> Result<TransferEvent, Error> {
    let funding_account = instruction.accounts()[0].to_string();
    let recipient_account = instruction.accounts()[1].to_string();
    let lamports = transfer.lamports;
    let funding_account_balance = context.account_balances.get(instruction.instruction.accounts()[0] as usize).map(|x| x.clone().into());
    let recipient_account_balance = context.account_balances.get(instruction.instruction.accounts()[1] as usize).map(|x| x.clone().into());
    
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
    transfer_with_seed: system_program::TransferWithSeed
) -> Result<TransferWithSeedEvent, Error> {
    let funding_account = instruction.accounts()[0].to_string();
    let base_account = instruction.accounts()[1].to_string();
    let recipient_account = instruction.accounts()[2].to_string();
    let from_owner = transfer_with_seed.from_owner.to_string();
    let from_seed = transfer_with_seed.from_seed.0.clone();
    let lamports = transfer_with_seed.lamports;
    let funding_account_balance = context.account_balances.get(instruction.instruction.accounts()[0] as usize).map(|x| x.clone().into());
    let recipient_account_balance = context.account_balances.get(instruction.instruction.accounts()[1] as usize).map(|x| x.clone().into());

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

    let system_transfer_instruction = instruction.inner_instructions().iter().find(|x| x.program_id() == SYSTEM_PROGRAM_ID).unwrap().clone();
    let system_transfer = system_program_substream::parse_transfer_instruction(system_transfer_instruction.as_ref(), context)?;
    let sol_amount = Some(system_transfer.lamports);

    let token_transfer_instruction = instruction.inner_instructions().iter().find(|x| x.program_id() == TOKEN_PROGRAM_ID).unwrap().clone();
    let token_transfer = spl_token_substream::parse_transfer_instruction(token_transfer_instruction.as_ref(), context).map_err(|e| anyhow!(e))?;
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
        _ => None
    };
    let sol_amount = trade.as_ref().map(|x| x.sol_amount);
    let virtual_sol_reserves = trade.as_ref().map(|x| x.virtual_sol_reserves);
    let virtual_token_reserves = trade.as_ref().map(|x| x.virtual_token_reserves);
    let real_sol_reserves = trade.as_ref().map(|x| x.real_sol_reserves);
    let real_token_reserves = trade.as_ref().map(|x| x.real_token_reserves);

    let direction = "sol".to_string();

    let token_transfer_instruction = instruction.inner_instructions().iter().find(|x| x.program_id() == TOKEN_PROGRAM_ID).unwrap().clone();
    let token_transfer = spl_token_substream::parse_transfer_instruction(token_transfer_instruction.as_ref(), context).map_err(|e| anyhow!(e))?;
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

    Ok(PumpfunWithdrawEvent {
        mint,
    })
}

fn parse_raydium_log(instruction: &StructuredInstruction) -> Result<RayLog, Error> {
    let re = regex::Regex::new(r"ray_log: (.+)").unwrap();
    let log_message = instruction.logs().as_ref().context("Failed to parse logs due to truncation")?.iter().rev().find_map(|log| {
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
    let data = instruction.logs().as_ref().context("Failed to parse logs due to truncation")?.iter().find_map(|log| match log {
        Log::Data(data_log) => data_log.data().ok(),
        _ => None,
    }).ok_or(anyhow!("Couldn't find data log."))?;
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