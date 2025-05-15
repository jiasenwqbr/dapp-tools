use substreams_database_change::pb::database::DatabaseChanges;

use crate::pb::spl_token::{SplTokenTransactionEvents, TransferEvent};

pub fn transform_block_meta_to_database_changes(
    changes: &mut DatabaseChanges,
    spl_transactions: Vec<SplTokenTransactionEvents>, 
    block_number: u64,){
        for (spl_index,spl_token_transaction_event) in spl_transactions.iter().enumerate(){
            let events = &spl_token_transaction_event.events;
            let signature = &spl_token_transaction_event.signature;
            for (spl_token_event_index,spl_token_event) in events.iter().enumerate(){
                if let Some(event) = &spl_token_event.event {
                    match event {
                        crate::pb::spl_token::spl_token_event::Event::Transfer(transfer_event) => {
                            handle_transfer_event(
                                block_number,
                                signature,
                                spl_index,
                                spl_token_event_index,
                                transfer_event,
                                changes
                            );
                        },
                        crate::pb::spl_token::spl_token_event::Event::InitializeMint(initialize_mint_event) => todo!(),
                        crate::pb::spl_token::spl_token_event::Event::InitializeImmutableOwner(initialize_immutable_owner_event) => todo!(),
                        crate::pb::spl_token::spl_token_event::Event::InitializeAccount(initialize_account_event) => todo!(),
                        crate::pb::spl_token::spl_token_event::Event::InitializeMultisig(initialize_multisig_event) => todo!(),
                        crate::pb::spl_token::spl_token_event::Event::Approve(approve_event) => todo!(),
                        crate::pb::spl_token::spl_token_event::Event::MintTo(mint_to_event) => todo!(),
                        crate::pb::spl_token::spl_token_event::Event::Revoke(revoke_event) => todo!(),
                        crate::pb::spl_token::spl_token_event::Event::SetAuthority(set_authority_event) => todo!(),
                        crate::pb::spl_token::spl_token_event::Event::Burn(burn_event) => todo!(),
                        crate::pb::spl_token::spl_token_event::Event::CloseAccount(close_account_event) => todo!(),
                        crate::pb::spl_token::spl_token_event::Event::FreezeAccount(freeze_account_event) => todo!(),
                        crate::pb::spl_token::spl_token_event::Event::ThawAccount(thaw_account_event) => todo!(),
                        crate::pb::spl_token::spl_token_event::Event::SyncNative(sync_native_event) => todo!(),
                    }
                }
            }
        }
}

fn handle_transfer_event(
    block_number:u64,
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
    let destination_post_balance = &destination.post_balance.unwrap();
    let destination_pre_balance = &destination.pre_balance.unwrap();
    let source = &transfer_event.source.clone().unwrap();
    let source_address = &source.address;
    let source_mint = &source.mint;
    let source_owner = &source.owner;
    let source_post_balance = &source.post_balance.unwrap();
    let source_pre_balance = &source.pre_balance.unwrap();

    save_transfer_event();
}

fn save_transfer_event(){}