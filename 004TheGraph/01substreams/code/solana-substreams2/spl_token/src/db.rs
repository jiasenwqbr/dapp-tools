use std::collections::HashMap;

use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};

use crate::pb::spl_token::{ApproveEvent, BurnEvent, CloseAccountEvent, FreezeAccountEvent, InitializeAccountEvent, InitializeImmutableOwnerEvent, InitializeMintEvent, InitializeMultisigEvent, MintToEvent, RevokeEvent, SetAuthorityEvent, SplTokenTransactionEvents, SyncNativeEvent, ThawAccountEvent, TransferEvent};

pub fn transform_block_meta_to_database_changes(
    changes: &mut DatabaseChanges,
    spl_transactions: Vec<SplTokenTransactionEvents>, 
    block_number: u64,
    block_time: i64,
    ){
        for (spl_index,spl_token_transaction_event) in spl_transactions.iter().enumerate(){
            let events = &spl_token_transaction_event.events;
            let signature = &spl_token_transaction_event.signature;
            
            for (spl_token_event_index,spl_token_event) in events.iter().enumerate(){
                if let Some(event) = &spl_token_event.event {
                    match event {
                        crate::pb::spl_token::spl_token_event::Event::Transfer(transfer_event) => handle_transfer_event(
                            block_number,
                            block_time,
                            signature,
                            spl_index,
                            spl_token_event_index,
                            transfer_event,
                            changes,
                        ),
                        crate::pb::spl_token::spl_token_event::Event::InitializeMint(initialize_mint_event) => handle_initialize_mint_event(
                            block_number,
                            block_time,
                            signature,
                            spl_index,
                            spl_token_event_index,
                            initialize_mint_event,
                            changes,
                        ),
                        crate::pb::spl_token::spl_token_event::Event::InitializeImmutableOwner(initialize_immutable_owner_event) => handle_initialize_immutable_owner_event(
                            block_number,
                            block_time,
                            signature,
                            spl_index,
                            spl_token_event_index,
                            initialize_immutable_owner_event,
                            changes,
                        ),
                        crate::pb::spl_token::spl_token_event::Event::InitializeAccount(initialize_account_event) => handle_initialize_account_event(
                            block_number,
                            block_time,
                            signature,
                            spl_index,
                            spl_token_event_index,
                            initialize_account_event,
                            changes,
                        ),
                        crate::pb::spl_token::spl_token_event::Event::InitializeMultisig(initialize_multisig_event) => handle_initialize_multisig_event(
                            block_number,
                            block_time,
                            signature,
                            spl_index,
                            spl_token_event_index,
                            initialize_multisig_event,
                            changes,
                        ),
                        crate::pb::spl_token::spl_token_event::Event::Approve(approve_event) => handle_approve_event(
                            block_number,
                            block_time,
                            signature,
                            spl_index,
                            spl_token_event_index,
                            approve_event,
                            changes,
                        ),
                        crate::pb::spl_token::spl_token_event::Event::MintTo(mint_to_event) => handle_mint_to_event(
                            block_number,
                            block_time,
                            signature,
                            spl_index,
                            spl_token_event_index,
                            mint_to_event,
                            changes,
                        ),
                        crate::pb::spl_token::spl_token_event::Event::Revoke(revoke_event) => handle_revoke_event(
                            block_number,
                            block_time,
                            signature,
                            spl_index,
                            spl_token_event_index,
                            revoke_event,
                            changes,
                        ),
                        crate::pb::spl_token::spl_token_event::Event::SetAuthority(set_authority_event) => handle_set_authority_event(
                            block_number,
                            block_time,
                            signature,
                            spl_index,
                            spl_token_event_index,
                            set_authority_event,
                            changes,
                        ),
                        crate::pb::spl_token::spl_token_event::Event::Burn(burn_event) => handle_burn_event(
                            block_number,
                            block_time,
                            signature,
                            spl_index,
                            spl_token_event_index,
                            burn_event,
                            changes,
                        ),
                        crate::pb::spl_token::spl_token_event::Event::CloseAccount(close_account_event) => handle_close_account_event(
                            block_number,
                            block_time,
                            signature,
                            spl_index,
                            spl_token_event_index,
                            close_account_event,
                            changes,
                        ),
                        crate::pb::spl_token::spl_token_event::Event::FreezeAccount(freeze_account_event) => handle_freeze_account_event(
                            block_number,
                            block_time,
                            signature,
                            spl_index,
                            spl_token_event_index,
                            freeze_account_event,
                            changes,
                        ),
                        crate::pb::spl_token::spl_token_event::Event::ThawAccount(thaw_account_event) => handle_thaw_account_event(
                            block_number,
                            block_time,
                            signature,
                            spl_index,
                            spl_token_event_index,
                            thaw_account_event,
                            changes,
                        ),
                        crate::pb::spl_token::spl_token_event::Event::SyncNative(sync_native_event) => handle_sync_native_event(
                            block_number,
                            block_time,
                            signature,
                            spl_index,
                            spl_token_event_index,
                            sync_native_event,
                            changes,
                        ),
                    }
                }
            }
        }
}

pub fn transform_block_meta_to_spl_database_changes(
    changes: &mut DatabaseChanges,
    spl_transactions: Vec<SplTokenTransactionEvents>, 
    block_number: u64,
    block_time: i64,
    ){
        for (spl_index,spl_token_transaction_event) in spl_transactions.iter().enumerate(){
            let events = &spl_token_transaction_event.events;
            let signature = &spl_token_transaction_event.signature;
            for (spl_token_event_index,spl_token_event) in events.iter().enumerate(){
                if let Some(event) = &spl_token_event.event {
                    match event {
                        crate::pb::spl_token::spl_token_event::Event::InitializeMint(initialize_mint_event) => handle_initialize_mint_event(
                            block_number,
                            block_time,
                            signature,
                            spl_index,
                            spl_token_event_index,
                            initialize_mint_event,
                            changes,
                        ),
                        // crate::pb::spl_token::spl_token_event::Event::InitializeAccount(initialize_account_event) => handle_initialize_account_event(
                        //     block_number,
                        //     block_time,
                        //     signature,
                        //     spl_index,
                        //     spl_token_event_index,
                        //     initialize_account_event,
                        //     changes,
                        // ),
                        _ => {},
                    }
                }
            }

        }
    }

fn handle_transfer_event(
    block_number:u64,
    block_time:i64,
    signature:&String,
    spl_index: usize,
    spl_token_event_index:usize,
    transfer_event: &TransferEvent,
    changes: &mut DatabaseChanges
){
    let amount = transfer_event.amount;
    let authority = &transfer_event.authority;
    let destination = &transfer_event.destination.clone().unwrap();
    let destination_address = &destination.address;
    let destination_mint = &destination.mint;
    let destination_owner = &destination.owner;
    let destination_post_balance = &destination.post_balance.unwrap_or(0);
    let destination_pre_balance = &destination.pre_balance.unwrap_or(0);
    let source: &crate::pb::spl_token::TokenAccount = &transfer_event.source.clone().unwrap();
    let source_address = &source.address;
    let source_mint = &source.mint;
    let source_owner = &source.owner;
    let source_post_balance = &source.post_balance.unwrap_or(0);
    let source_pre_balance = &source.pre_balance.unwrap_or(0);

    let id  = format!("{block_number}_{signature}_{spl_index}_{spl_token_event_index}");
    
    save_transfer_event(
        id,
        block_number,
        block_time,
        signature,
        spl_index,
        spl_token_event_index,
        amount,
        authority,
        destination_address,
        destination_mint,
        destination_owner,
        destination_pre_balance,
        destination_post_balance,
        source_address,
        source_mint,
        source_owner,
        source_pre_balance,
        source_post_balance,
        changes
    );
}

fn save_transfer_event(
    id: String,
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index: usize,
    amount:u64,
    authority:&String,
    destination_address: &String,
    destination_mint: &String,
    destination_owner: &String,
    destination_pre_balance: &u64,
    destination_post_balance: &u64,
    source_address:&String,
    source_mint:&String,
    source_owner:&String,
    source_pre_balance: &u64,
    source_post_balance: &u64,
    changes: &mut DatabaseChanges){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);

    changes.push_change_composite("solana_substream_spl_token", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("block_time", (None,block_time))
        .change("signature", (None,signature))
        .change("spl_index", (None, spl_index as i64)) // usize to i64
        .change("spl_token_event_index", (None, spl_token_event_index as i64))
        .change("amount", (None, amount as i64)) // u64 to i64 if you store as number
        .change("authority", (None, authority))
        .change("destination_address", (None, destination_address))
        .change("destination_mint", (None, destination_mint))
        .change("destination_owner", (None, destination_owner))
        .change("destination_pre_balance", (None, *destination_pre_balance as i64))
        .change("destination_post_balance", (None, *destination_post_balance as i64))
        .change("source_address", (None, source_address))
        .change("source_mint", (None, source_mint))
        .change("source_owner", (None, source_owner))
        .change("source_pre_balance", (None, *source_pre_balance as i64))
        .change("source_post_balance", (None, *source_post_balance as i64));
}

fn handle_initialize_mint_event(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    initialize_mint_event: &InitializeMintEvent,
    changes: &mut DatabaseChanges,
){
    let decimals = initialize_mint_event.decimals;
    let binding = String::new();
    let freeze_authority = match &initialize_mint_event.freeze_authority {
        Some(freeze) => freeze,
        None => &binding,
    };
   
    let mint = &initialize_mint_event.mint;
    let mint_authority = &initialize_mint_event.mint_authority;

    let id = format!("{block_number}_{signature}_{spl_index}_{spl_token_event_index}");

    save_initialize_mint_event(
        block_number,
        block_time,
        signature,
        spl_index,
        spl_token_event_index,
        decimals,
        freeze_authority,
        mint,
        mint_authority,
        id,
        changes,
    );
}

fn save_initialize_mint_event(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    decimals: u32,
    freeze_authority: &String,
    mint: &String,
    mint_authority: &String,
    id: String,
    changes: &mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);
    changes.push_change_composite("solana_substream_spl_token_initialize_mint", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("block_time", (None, block_time))
        .change("signature", (None, signature.to_string()))
        .change("spl_index", (None, spl_index as i64))
        .change("spl_token_event_index", (None, spl_token_event_index as i64))
        .change("decimals", (None, decimals as i32))
        .change("freeze_authority", (None, freeze_authority.to_string()))
        .change("mint", (None, mint.to_string()))
        .change("mint_authority", (None, mint_authority.to_string()));
}

fn handle_initialize_immutable_owner_event(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    initialize_immutable_owner_event: &InitializeImmutableOwnerEvent,
    changes: &mut DatabaseChanges,
){
    match &initialize_immutable_owner_event.account {
        Some(token_account) => {
            let address = &token_account.address;
            let mint = &token_account.mint;
            let owner = &token_account.owner;
            let post_balance = match token_account.post_balance{
                Some(val) => val,
                None => 0,
            };
            let pre_balance = match token_account.pre_balance{
                Some(val) => val,
                None => 0,
            };
            let id = format!("{block_number}_{signature}_{spl_index}_{spl_token_event_index}");
            save_initialize_immutable_owner(
                block_number,
                block_time,
                signature,
                spl_index,
                spl_token_event_index,
                address,
                mint,
                owner,
                post_balance,
                pre_balance,
                id,
                changes,
            );
        },
        None => {},
    };
}


fn save_initialize_immutable_owner(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    address: &String,
    mint: &String,
    owner: &String,
    post_balance: u64,
    pre_balance: u64,
    id: String,
    changes: &mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);
    changes.push_change_composite("solana_substream_spl_token_initialize_immutable_owner", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("block_time", (None, block_time))
        .change("signature", (None, signature))
        .change("spl_index", (None, spl_index as i64))
        .change("spl_token_event_index", (None, spl_token_event_index as i64))
        .change("address", (None, address))
        .change("mint", (None, mint))
        .change("owner", (None, owner))
        .change("pre_balance", (None, pre_balance as i64))
        .change("post_balance", (None, post_balance as i64));
}


fn handle_initialize_account_event(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    initialize_account_event:&InitializeAccountEvent,
    changes: &mut DatabaseChanges,
){
    match &initialize_account_event.account {
        Some(token_account) => {
            let address = &token_account.address;
            let mint = &token_account.mint;
            let owner = &token_account.owner;
            let post_balance = match token_account.post_balance{
                Some(val) => val,
                None => 0,
            };
            let pre_balance = match token_account.pre_balance{
                Some(val) => val,
                None => 0,
            };
            let id = format!("{block_number}_{signature}_{spl_index}_{spl_token_event_index}");
            save_initialize_account_event(
                block_number,
                block_time,
                signature,
                spl_index,
                spl_token_event_index,
                address,
                mint,
                owner,
                post_balance,
                pre_balance,
                id,
                changes,
            );
        },
        None => {},
    };
}

fn save_initialize_account_event(
    block_number: u64,
    block_time:i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    address: &String,
    mint: &String,
    owner: &String,
    post_balance: u64,
    pre_balance: u64,
    id: String,
    changes: &mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);
    changes.push_change_composite("solana_substream_spl_token_initialize_account", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("block_time", (None,block_time))
        .change("signature", (None, signature))
        .change("spl_index", (None, spl_index as i64))
        .change("spl_token_event_index", (None, spl_token_event_index as i64))
        .change("address", (None, address))
        .change("mint", (None, mint))
        .change("owner", (None, owner))
        .change("pre_balance", (None, pre_balance as i64))
        .change("post_balance", (None, post_balance as i64));
}


fn handle_initialize_multisig_event(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    initialize_multisig_event:&InitializeMultisigEvent,
    changes: &mut DatabaseChanges,
){
    let multisig = &initialize_multisig_event.multisig;
    let m = &initialize_multisig_event.m;
    let signers = &initialize_multisig_event.signers;
    for (signer_index,signer) in signers.iter().enumerate(){
        let id =  format!("{block_number}_{signature}_{spl_index}_{spl_token_event_index}_{signer_index}");
        save_initialize_multisig(
            block_number,
            block_time,
            signature,
            spl_index,
            spl_token_event_index,
            multisig,
            m,
            signer_index,
            signer,
            id,
            changes,
        );
    }
}

fn save_initialize_multisig(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    multisig: &String,
    m: &u32,
    signer_index: usize,
    signer: &String,
    id: String,
    changes: &mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);
    changes.push_change_composite("solana_substream_spl_token_initialize_multisig", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("block_time", (None,block_time))
        .change("signature", (None, signature.clone()))
        .change("spl_index", (None, spl_index as i64)) // 转换为i64以兼容更多数据库
        .change("spl_token_event_index", (None, spl_token_event_index as i64))
        .change("multisig", (None, multisig.clone()))
        .change("m", (None, *m as i64)) // u32转换为i64
        .change("signer_index", (None, signer_index as i64))
        .change("signer", (None, signer.clone()));

}

fn handle_approve_event(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    approve_event: &ApproveEvent,
    changes: &mut DatabaseChanges,
){
    let amount = &approve_event.amount;
    let delegate = &approve_event.delegate;
    match &approve_event.source {
        Some(approve) => {
            let approve_address = &approve.address;
            let approve_mint = &approve.mint;
            let approve_owner = &approve.owner;
            let approve_post_balance = &approve.post_balance.unwrap_or(0);
            let approve_pre_balance = &approve.pre_balance.unwrap_or(0);
            let id = format!("{block_number}_{signature}_{spl_index}_{spl_token_event_index}");
            save_approve_event(
                block_number,
                block_time,
                signature,
                spl_index,
                spl_token_event_index,
                amount,
                delegate,
                approve_address,
                approve_mint,
                approve_owner,
                approve_post_balance,
                approve_pre_balance,
                id,
                changes,
            );
        },
        None => {},
    }
}


fn save_approve_event(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    amount:&u64,
    delegate: &String,
    approve_address: &String,
    approve_mint: &String,
    approve_owner: &String,
    approve_post_balance: &u64,
    approve_pre_balance: &u64,
    id: String,
    changes: &mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);
    changes.push_change_composite("solana_substream_spl_token_approve", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("block_time", (None,block_time))
        .change("signature", (None, signature.clone()))
        .change("spl_index", (None, spl_index as i64))
        .change("spl_token_event_index", (None, spl_token_event_index as i64))
        .change("amount", (None, *amount as i64))
        .change("delegate", (None, delegate.clone()))
        .change("approve_address", (None, approve_address.clone()))
        .change("approve_mint", (None, approve_mint.clone()))
        .change("approve_owner", (None, approve_owner.clone()))
        .change("approve_post_balance", (None, *approve_post_balance as i64))
        .change("approve_pre_balance", (None, *approve_pre_balance as i64));
}

fn handle_mint_to_event(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    mint_to_event: &MintToEvent,
    changes: &mut DatabaseChanges,
){
    let amount = mint_to_event.amount;
    let mint = &mint_to_event.mint;
    let mint_authority = &mint_to_event.mint_authority;
    match &mint_to_event.destination {
        Some(destination) => {
            let mint_address = &destination.address;
            let mint_mint = &destination.mint;
            let mint_owner = &destination.owner;
            let approve_post_balance = &destination.post_balance.unwrap_or(0);
            let approve_pre_balance = &destination.pre_balance.unwrap_or(0);
            let id = format!("{block_number}_{signature}_{spl_index}_{spl_token_event_index}");
            save_mint_to(
                block_number,
                block_time,
                signature,
                spl_index,
                spl_token_event_index,
                amount,
                mint,
                mint_authority,
                mint_address,
                mint_mint,
                mint_owner,
                approve_post_balance,
                approve_pre_balance,
                id,
                changes
            );

        },
        None => {},
    }
}

fn save_mint_to(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    amount:u64,
    mint: &String,
    mint_authority: &String,
    mint_address: &String,
    mint_mint: &String,
    mint_owner: &String,
    approve_post_balance: &u64,
    approve_pre_balance: &u64,
    id: String,
    changes: &mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);
    changes.push_change_composite("solana_substream_spl_token_mint_to", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("block_time", (None,block_time))
        .change("signature", (None, signature.clone()))
        .change("spl_index", (None, spl_index as i64))
        .change("spl_token_event_index", (None, spl_token_event_index as i64))
        .change("amount", (None, amount as i64))
        .change("mint", (None, mint.clone()))
        .change("mint_authority", (None, mint_authority.clone()))
        .change("mint_address", (None, mint_address.clone()))
        .change("mint_mint", (None, mint_mint.clone()))
        .change("mint_owner", (None, mint_owner.clone()))
        .change("approve_post_balance", (None, *approve_post_balance as i64))
        .change("approve_pre_balance", (None, *approve_pre_balance as i64));
}

fn handle_revoke_event(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    revoke_event: &RevokeEvent,
    changes: &mut DatabaseChanges,
){
    match &revoke_event.source {
        Some(revoke) => {
            let revoke_address = &revoke.address;
            let revoke_mint = &revoke.mint;
            let revoke_owner = &revoke.owner;
            let revoke_post_balance = &revoke.post_balance.unwrap_or(0);
            let revoke_pre_balance = &revoke.pre_balance.unwrap_or(0);
            let id = format!("{block_number}_{signature}_{spl_index}_{spl_token_event_index}");
            save_revoke(
                block_number,
                block_time,
                signature,
                spl_index,
                spl_token_event_index,
                revoke_address,
                revoke_mint,
                revoke_owner,
                revoke_post_balance,
                revoke_pre_balance,
                id,
                changes,
            );
        },
        None => {},
    }

}

fn save_revoke(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    revoke_address:&String,
    revoke_mint:&String,
    revoke_owner:&String,
    revoke_post_balance:&u64,
    revoke_pre_balance:&u64,
    id:String,
    changes:&mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);
    changes.push_change_composite("solana_substream_spl_token_revoke", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("block_time", (None,block_time))
        .change("signature", (None, signature.clone()))
        .change("spl_index", (None, spl_index as i64))
        .change("spl_token_event_index", (None, spl_token_event_index as i64))
        .change("revoke_address", (None, revoke_address.clone()))
        .change("revoke_mint", (None, revoke_mint.clone()))
        .change("revoke_owner", (None, revoke_owner.clone()))
        .change("revoke_post_balance", (None, *revoke_post_balance as i64))
        .change("revoke_pre_balance", (None, *revoke_pre_balance as i64));
}

fn handle_set_authority_event(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    set_authority_event: &SetAuthorityEvent,
    changes: &mut DatabaseChanges,
){
    let authority = &set_authority_event.authority;
    let authority_type = set_authority_event.authority_type;
    let mint = &set_authority_event.mint;
    let binding = String::new();
    let new_authority = match &set_authority_event.new_authority{
        Some(val) => val,
        None => &binding,
    };
    let id = format!("{block_number}_{signature}_{spl_index}_{spl_token_event_index}");
    save_set_authority(
        block_number,
        block_time,
        signature,
        spl_index,
        spl_token_event_index,
        authority,
        authority_type,
        mint,
        new_authority,
        id,
        changes,
    );
}

fn save_set_authority(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    authority: &String,
    authority_type:i32,
    mint:&String,
    new_authority:&String,
    id:String,
    changes:&mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);
    changes.push_change_composite("solana_substream_spl_token_set_authority", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("block_time", (None,block_time))
        .change("signature", (None, signature.clone()))
        .change("spl_index", (None, spl_index as i64))
        .change("spl_token_event_index", (None, spl_token_event_index as i64))
        .change("authority", (None, authority.clone()))
        .change("authority_type", (None, authority_type))
        .change("mint", (None, mint.clone()))
        .change("new_authority", (None, new_authority.clone()));
}

fn handle_burn_event(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    burn_event: &BurnEvent,
    changes: &mut DatabaseChanges,
){
    let amount = burn_event.amount;
    let authority = &burn_event.authority;
    match &burn_event.source {
        Some(burn) => {
            let burn_address = &burn.address;
            let burn_mint = &burn.mint;
            let burn_owner = &burn.owner;
            let burn_post_balance = &burn.post_balance.unwrap_or(0);
            let burn_pre_balance = &burn.pre_balance.unwrap_or(0);
            let id = format!("{block_number}_{signature}_{spl_index}_{spl_token_event_index}");
            save_burn(
                block_number,
                block_time,
                signature,
                spl_index,
                spl_token_event_index,
                amount,
                authority,
                burn_address,
                burn_mint,
                burn_owner,
                burn_post_balance,
                burn_pre_balance,
                id,
                changes,
            );
        },
        None => {},
    }
}

fn save_burn(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    amount: u64,
    authority:&String,
    burn_address:&String,
    burn_mint:&String,
    burn_owner:&String,
    burn_post_balance:&u64,
    burn_pre_balance:&u64,
    id:String,
    changes:&mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);
    changes.push_change_composite("solana_substream_spl_token_burn", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("block_time", (None,block_time))
        .change("signature", (None, signature.clone()))
        .change("spl_index", (None, spl_index as i64))
        .change("spl_token_event_index", (None, spl_token_event_index as i64))
        .change("amount", (None, amount as i64))
        .change("authority", (None, authority.clone()))
        .change("burn_address", (None, burn_address.clone()))
        .change("burn_mint", (None, burn_mint.clone()))
        .change("burn_owner", (None, burn_owner.clone()))
        .change("burn_post_balance", (None, *burn_post_balance as i64))
        .change("burn_pre_balance", (None, *burn_pre_balance as i64));
}

fn handle_close_account_event(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    close_account_event:&CloseAccountEvent,
    changes:&mut DatabaseChanges,
){
    let destination = &close_account_event.destination;
    match &close_account_event.source{
        Some(source) => {
            let source_address = &source.address;
            let source_mint = &source.mint;
            let source_owner = &source.owner;
            let source_post_balance = &source.post_balance.unwrap_or(0);
            let source_pre_balance = &source.pre_balance.unwrap_or(0);
            let id = format!("{block_number}_{signature}_{spl_index}_{spl_token_event_index}");
            save_close_account(
                block_number,
                block_time,
                signature,
                spl_index,
                spl_token_event_index,
                destination,
                source_address,
                source_mint,
                source_owner,
                source_post_balance,
                source_pre_balance,
                id,
                changes,
            );
        },
        None => {},
    };
}

fn save_close_account(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    destination:&String,
    source_address:&String,
    source_mint:&String,
    source_owner:&String,
    source_post_balance:&u64,
    source_pre_balance:&u64,
    id:String,
    changes:&mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);
    changes.push_change_composite("solana_substream_spl_token_close_account", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("block_time", (None,block_time))
        .change("signature", (None, signature.clone()))
        .change("spl_index", (None, spl_index as i64))
        .change("spl_token_event_index", (None, spl_token_event_index as i64))
        .change("destination", (None, destination.clone()))
        .change("source_address", (None, source_address.clone()))
        .change("source_mint", (None, source_mint.clone()))
        .change("source_owner", (None, source_owner.clone()))
        .change("source_post_balance", (None, *source_post_balance as i64))
        .change("source_pre_balance", (None, *source_pre_balance as i64));
}


fn handle_freeze_account_event(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    freeze_account_event:&FreezeAccountEvent,
    changes:&mut DatabaseChanges,
){
    let freeze_authority = &freeze_account_event.freeze_authority;
    match &freeze_account_event.source {
        Some(freeze) => {
            let freeze_address = &freeze.address;
            let freeze_mint = &freeze.mint;
            let freeze_owner = &freeze.owner;
            let freeze_post_balance = &freeze.post_balance.unwrap_or(0);
            let freeze_pre_balance = &freeze.pre_balance.unwrap_or(0);
            let id = format!("{block_number}_{signature}_{spl_index}_{spl_token_event_index}");
            save_freeze_account(
                block_number,
                block_time,
                signature,
                spl_index,
                spl_token_event_index,
                freeze_authority,
                freeze_address,
                freeze_mint,
                freeze_owner,
                freeze_post_balance,
                freeze_pre_balance,
                id,
                changes,
            );
        },
        None => {},
    }
}

fn save_freeze_account(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    freeze_authority:&String,
    freeze_address:&String,
    freeze_mint:&String,
    freeze_owner:&String,
    freeze_post_balance:&u64,
    freeze_pre_balance:&u64,
    id:String,
    changes:&mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);
    changes.push_change_composite("solana_substream_spl_token_freeze_account", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("block_time", (None,block_time))
        .change("signature", (None, signature.clone()))
        .change("spl_index", (None, spl_index as i64))
        .change("spl_token_event_index", (None, spl_token_event_index as i64))
        .change("freeze_authority", (None, freeze_authority.clone()))
        .change("freeze_address", (None, freeze_address.clone()))
        .change("freeze_mint", (None, freeze_mint.clone()))
        .change("freeze_owner", (None, freeze_owner.clone()))
        .change("freeze_post_balance", (None, *freeze_post_balance as i64))
        .change("freeze_pre_balance", (None, *freeze_pre_balance as i64));
}

fn handle_thaw_account_event(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    thaw_account_event:&ThawAccountEvent,
    changes:&mut DatabaseChanges,
){
    let freeze_authority = &thaw_account_event.freeze_authority;
    match &thaw_account_event.source {
        Some(source) => {
            let source_address = &source.address;
            let source_mint = &source.mint;
            let source_owner = &source.owner;
            let source_post_balance = &source.post_balance.unwrap_or(0);
            let source_pre_balance = &source.pre_balance.unwrap_or(0);
            let id = format!("{block_number}_{signature}_{spl_index}_{spl_token_event_index}");
            save_thaw_account(
                block_number,
                block_time,
                signature,
                spl_index,
                spl_token_event_index,
                freeze_authority,
                source_address,
                source_mint,
                source_owner,
                source_post_balance,
                source_pre_balance,
                id,
                changes,
            );
        },
        None => {},
    }
}

fn save_thaw_account(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    freeze_authority: &String,
    source_address:&String,
    source_mint:&String,
    source_owner:&String,
    source_post_balance:&u64,
    source_pre_balance:&u64,
    id:String,
    changes:&mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);
    changes.push_change_composite("solana_substream_spl_token_thaw_account", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("block_time", (None,block_time))
        .change("signature", (None, signature.clone()))
        .change("spl_index", (None, spl_index as i64))
        .change("spl_token_event_index", (None, spl_token_event_index as i64))
        .change("freeze_authority", (None, freeze_authority.clone()))
        .change("source_address", (None, source_address.clone()))
        .change("source_mint", (None, source_mint.clone()))
        .change("source_owner", (None, source_owner.clone()))
        .change("source_post_balance", (None, *source_post_balance as i64))
        .change("source_pre_balance", (None, *source_pre_balance as i64));
}

fn handle_sync_native_event(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    sync_native_event:&SyncNativeEvent,
    changes:&mut DatabaseChanges,
){
   match &sync_native_event.account {
    Some(account) => {
        let address = &account.address;
        let mint = &account.mint;
        let owner = &account.owner;
        let post_balance = &account.post_balance.unwrap_or(0);
        let pre_balance = &account.pre_balance.unwrap_or(0);
        let id = format!("{block_number}_{signature}_{spl_index}_{spl_token_event_index}");
        save_sync_native(
            block_number,
            block_time,
            signature,
            spl_index,
            spl_token_event_index,
            address,
            mint,
            owner,
            post_balance,
            pre_balance,
            id,
            changes,
        );
    },
    None => {},
   };
}

fn save_sync_native(
    block_number: u64,
    block_time: i64,
    signature: &String,
    spl_index: usize,
    spl_token_event_index:usize,
    address:&String,
    mint:&String,
    owner:&String,
    post_balance:&u64,
    pre_balance:&u64,
    id:String,
    changes:&mut DatabaseChanges,
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert( "id".to_string(), id,);
    changes.push_change_composite("solana_substream_spl_token_sync_native", composite_key, 1, Operation::Create)
        .change("block_number", (None, block_number))
        .change("block_time", (None,block_time))
        .change("signature", (None, signature.clone()))
        .change("spl_index", (None, spl_index as i64))
        .change("spl_token_event_index", (None, spl_token_event_index as i64))
        .change("address", (None, address.clone()))
        .change("mint", (None, mint.clone()))
        .change("owner", (None, owner.clone()))
        .change("post_balance", (None, *post_balance as i64))
        .change("pre_balance", (None, *pre_balance as i64));
}