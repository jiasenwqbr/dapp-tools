use std::collections::HashMap;

use substreams_database_change::pb::database::{self, table_change::Operation, DatabaseChanges};

use crate::pb::pumpfun::{InitializeEvent, PumpfunCreateEvent, PumpfunSwapEvent, PumpfunTransactionEvents, PumpfunWithdrawEvent, SetParamsEvent};
pub fn transform_block_meta_to_database_changes(
    changes: &mut DatabaseChanges,
    block_events: Vec<PumpfunTransactionEvents>,
    block_number: u64,){
    for (pump_index,pubfun) in block_events.iter().enumerate(){
        let signature = &pubfun.signature;
       for(event_index,event) in pubfun.events.iter().enumerate(){
            match &event.event {
                Some(pumpfun_event) => {
                    match pumpfun_event {
                        crate::pb::pumpfun::pumpfun_event::Event::Initialize(initialize_event) => handle_initialize_event(
                            block_number,
                            signature,
                            pump_index,
                            event_index,
                            initialize_event,
                            changes,
                        ),
                        crate::pb::pumpfun::pumpfun_event::Event::SetParams(set_params_event) => handle_set_params_event(
                            block_number,
                            signature,
                            pump_index,
                            event_index,
                            set_params_event,
                            changes,
                        ),
                        crate::pb::pumpfun::pumpfun_event::Event::PumpfunSwap(pumpfun_swap_event) => handle_pumpfun_swap_event(
                            block_number,
                            signature,
                            pump_index,
                            event_index,
                            pumpfun_swap_event,
                            changes,
                        ),
                        crate::pb::pumpfun::pumpfun_event::Event::PumpfunWithdraw(pumpfun_withdraw_event) => handle_pumpfun_withdraw_event(
                            block_number,
                            signature,
                            pump_index,
                            event_index,
                            pumpfun_withdraw_event,
                            changes,
                        ),
                        crate::pb::pumpfun::pumpfun_event::Event::PumpfunCreate(pumpfun_create_event) => handle_pumpfun_create_event(
                            block_number,
                            signature,
                            pump_index,
                            event_index,
                            pumpfun_create_event,
                            changes,
                        ),
                    }
                },
                None => {},
            }
       }
    }

}


fn handle_initialize_event(
    block_number:u64,
    signature:&String,
    pump_index:usize,
    event_index:usize,
    initialize_event:&InitializeEvent,
    changes: &mut DatabaseChanges,
){
    let user = &initialize_event.user;
    let id = format!("{block_number}_{signature}_{pump_index}_{event_index}");
    save_initialize_user(
        block_number,
        signature,
        pump_index,
        event_index,
        user,
        id,
        changes,
    );
}

fn save_initialize_user(
    block_number:u64,
    signature:&String,
    pump_index:usize,
    event_index:usize,
    user: &String,
    id: String,
    changes: &mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);

    changes.push_change_composite("solana_substream_pumpfun_initialize_user", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("signature", (None, signature.clone()))
        .change("pump_index", (None, pump_index as i64))
        .change("event_index", (None, event_index as i64))
        .change("inital_user", (None, user.clone()));
}

fn handle_set_params_event(
    block_number:u64,
    signature:&String,
    pump_index:usize,
    event_index:usize,
    set_params_event:&SetParamsEvent,
    changes:&mut DatabaseChanges,
){
    let fee_basis_points = set_params_event.fee_basis_points;
    let fee_recipient = &set_params_event.fee_recipient;
    let initial_real_token_reserves = set_params_event.initial_real_token_reserves;
    let initial_virtual_sol_reserves = set_params_event.initial_virtual_sol_reserves;
    let initial_virtual_token_reserves = set_params_event.initial_virtual_token_reserves;
    let token_total_supply = set_params_event.token_total_supply;
    let user = &set_params_event.user;
    let id = format!("{block_number}_{signature}_{pump_index}_{event_index}");
    save_set_params(
        block_number,
        signature,
        pump_index,
        event_index,
        user,
        id,
        fee_basis_points,
        fee_recipient,
        initial_real_token_reserves,
        initial_virtual_sol_reserves,
        initial_virtual_token_reserves,
        token_total_supply,
        changes,
    );
}


fn save_set_params(
    block_number:u64,
    signature:&String,
    pump_index:usize,
    event_index:usize,
    user: &String,
    id: String,
    fee_basis_points:u64,
    fee_recipient:&String,
    initial_real_token_reserves:u64,
    initial_virtual_sol_reserves:u64,
    initial_virtual_token_reserves:u64,
    token_total_supply:u64,
    changes:&mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);

    changes.push_change_composite("solana_substream_pumpfun_set_params", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("signature", (None, signature.clone()))
        .change("pump_index", (None, pump_index as i64))
        .change("event_index", (None, event_index as i64))
        .change("set_user", (None, user.clone()))
        .change("fee_basis_points", (None, fee_basis_points as i64))
        .change("fee_recipient", (None, fee_recipient.clone()))
        .change("initial_real_token_reserves", (None, initial_real_token_reserves as i64))
        .change("initial_virtual_sol_reserves", (None, initial_virtual_sol_reserves as i64))
        .change("initial_virtual_token_reserves", (None, initial_virtual_token_reserves as i64))
        .change("token_total_supply", (None, token_total_supply as i64));
}

fn handle_pumpfun_swap_event(
    block_number:u64,
    signature:&String,
    pump_index:usize,
    event_index:usize,
    pumpfun_swap_event:&PumpfunSwapEvent,
    changes:&mut DatabaseChanges,
){
    let bonding_curve = &pumpfun_swap_event.bonding_curve;
    let direction = &pumpfun_swap_event.direction;
    let mint = &pumpfun_swap_event.mint;
    let zero = 0;
    let real_sol_reserves=  pumpfun_swap_event.real_sol_reserves.unwrap_or(0);
    let real_token_reserves = pumpfun_swap_event.real_token_reserves.unwrap_or(0);
    let sol_amount = pumpfun_swap_event.sol_amount.unwrap_or(0);
    let token_amount = pumpfun_swap_event.token_amount;
    let swap_user = &pumpfun_swap_event.user;
    let user_token_pre_balance = pumpfun_swap_event.user_token_pre_balance.unwrap_or(0);
    let virtual_sol_reserves = pumpfun_swap_event.virtual_sol_reserves.unwrap_or(0);
    let id = format!("{block_number}_{signature}_{pump_index}_{event_index}");
    save_pumpfun_swap(
        block_number,
        signature,
        pump_index,
        event_index,
        bonding_curve,
        direction,
        mint,
        zero,
        real_sol_reserves,
        real_token_reserves,
        sol_amount,
        token_amount,
        swap_user,
        user_token_pre_balance,
        virtual_sol_reserves,
        id,
        changes,
    );

}
fn save_pumpfun_swap(
    block_number:u64,
    signature:&String,
    pump_index:usize,
    event_index:usize,
    bonding_curve:&String,
    direction:&String,
    mint:&String,
    zero:i32,
    real_sol_reserves:u64,
    real_token_reserves:u64,
    sol_amount:u64,
    token_amount:u64,
    swap_user:&String,
    user_token_pre_balance:u64,
    virtual_sol_reserves:u64,
    id:String,
    changes:&mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);

    changes.push_change_composite("solana_substream_pumpfun_swap", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("signature", (None, signature.clone()))
        .change("pump_index", (None, pump_index as i64))
        .change("event_index", (None, event_index as i64))
        .change("bonding_curve", (None, bonding_curve.clone()))
        .change("direction", (None, direction.clone()))
        .change("mint", (None, mint.clone()))
        .change("zero", (None, zero))
        .change("real_sol_reserves", (None, real_sol_reserves as i64))
        .change("real_token_reserves", (None, real_token_reserves as i64))
        .change("sol_amount", (None, sol_amount as i64))
        .change("token_amount", (None, token_amount as i64))
        .change("swap_user", (None, swap_user.clone()))
        .change("user_token_pre_balance", (None, user_token_pre_balance as i64))
        .change("virtual_sol_reserves", (None, virtual_sol_reserves as i64));
}


fn handle_pumpfun_withdraw_event(
    block_number:u64,
    signature:&String,
    pump_index:usize,
    event_index:usize,
    pumpfun_withdraw_event:&PumpfunWithdrawEvent,
    changes:&mut DatabaseChanges,
){
    let mint = &pumpfun_withdraw_event.mint;
    let id = format!("{block_number}_{signature}_{pump_index}_{event_index}");
    save_pumpfun_withdraw(
        block_number,
        signature,
        pump_index,
        event_index,
        id,
        mint,
        changes,
    );
}

fn save_pumpfun_withdraw(
    block_number:u64,
    signature:&String,
    pump_index:usize,
    event_index:usize,
    id: String,
    mint: &String,
    changes: &mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);

    changes.push_change_composite("solana_substream_pumpfun_withdraw", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("signature", (None, signature.clone()))
        .change("pump_index", (None, pump_index as i64))
        .change("event_index", (None, event_index as i64))
        .change("mint", (None,mint));
}

fn handle_pumpfun_create_event(
    block_number:u64,
    signature:&String,
    pump_index:usize,
    event_index:usize,
    pumpfun_create_event:&PumpfunCreateEvent,
    changes:&mut DatabaseChanges,
){
    let associated_bonding_curve = &pumpfun_create_event.associated_bonding_curve;
    let bonding_curve = &pumpfun_create_event.bonding_curve;
    let metadata = &pumpfun_create_event.metadata;
    let mint = &pumpfun_create_event.mint;
    let token_name = &pumpfun_create_event.name;
    let symbol = &pumpfun_create_event.symbol;
    let uri = &pumpfun_create_event.uri;
    let carete_user = &pumpfun_create_event.user;
    let id = format!("{block_number}_{signature}_{pump_index}_{event_index}");
    save_create(
        block_number,
        signature,
        pump_index,
        event_index,
        id,
        associated_bonding_curve,
        bonding_curve,
        metadata,
        mint,
        token_name,
        symbol,
        uri,
        carete_user,
        changes,
    );
}

fn save_create(
    block_number:u64,
    signature:&String,
    pump_index:usize,
    event_index:usize,
    id:String,
    associated_bonding_curve:&String,
    bonding_curve:&String,
    metadata:&String,
    mint:&String,
    token_name:&String,
    symbol:&String,
    uri:&String,
    carete_user:&String,
    changes:&mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);

    changes.push_change_composite("solana_substream_pumpfun_create", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("signature", (None, signature.clone()))
        .change("pump_index", (None, pump_index as i64))
        .change("event_index", (None, event_index as i64))
        .change("associated_bonding_curve", (None, associated_bonding_curve.clone()))
        .change("bonding_curve", (None, bonding_curve.clone()))
        .change("metadata", (None, metadata.clone()))
        .change("mint", (None, mint.clone()))
        .change("token_name", (None, token_name.clone()))
        .change("symbol", (None, symbol.clone()))
        .change("uri", (None, uri.clone()))
        .change("carete_user", (None, carete_user.clone()));
}