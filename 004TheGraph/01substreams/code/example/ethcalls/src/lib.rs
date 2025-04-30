mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const USDTCONTRACT_TRACKED_CONTRACT: [u8; 20] = hex!("dac17f958d2ee523a2206206994597c13d831ec7");

fn map_usdtcontract_events(blk: &eth::Block, events: &mut contract::Events) {
    events.usdtcontract_added_black_lists.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDTCONTRACT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdtcontract_contract::events::AddedBlackList::match_and_decode(log) {
                        return Some(contract::UsdtcontractAddedBlackList {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_user: event.u_user,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdtcontract_approvals.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDTCONTRACT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdtcontract_contract::events::Approval::match_and_decode(log) {
                        return Some(contract::UsdtcontractApproval {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            owner: event.owner,
                            spender: event.spender,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdtcontract_deprecates.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDTCONTRACT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdtcontract_contract::events::Deprecate::match_and_decode(log) {
                        return Some(contract::UsdtcontractDeprecate {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_address: event.new_address,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdtcontract_destroyed_black_funds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDTCONTRACT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdtcontract_contract::events::DestroyedBlackFunds::match_and_decode(log) {
                        return Some(contract::UsdtcontractDestroyedBlackFunds {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_balance: event.u_balance.to_string(),
                            u_black_listed_user: event.u_black_listed_user,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdtcontract_issues.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDTCONTRACT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdtcontract_contract::events::Issue::match_and_decode(log) {
                        return Some(contract::UsdtcontractIssue {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdtcontract_params.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDTCONTRACT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdtcontract_contract::events::Params::match_and_decode(log) {
                        return Some(contract::UsdtcontractParams {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            fee_basis_points: event.fee_basis_points.to_string(),
                            max_fee: event.max_fee.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdtcontract_pauses.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDTCONTRACT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdtcontract_contract::events::Pause::match_and_decode(log) {
                        return Some(contract::UsdtcontractPause {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdtcontract_redeems.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDTCONTRACT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdtcontract_contract::events::Redeem::match_and_decode(log) {
                        return Some(contract::UsdtcontractRedeem {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdtcontract_removed_black_lists.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDTCONTRACT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdtcontract_contract::events::RemovedBlackList::match_and_decode(log) {
                        return Some(contract::UsdtcontractRemovedBlackList {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_user: event.u_user,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdtcontract_transfers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDTCONTRACT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdtcontract_contract::events::Transfer::match_and_decode(log) {
                        return Some(contract::UsdtcontractTransfer {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdtcontract_unpauses.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDTCONTRACT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdtcontract_contract::events::Unpause::match_and_decode(log) {
                        return Some(contract::UsdtcontractUnpause {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                        });
                    }

                    None
                })
        })
        .collect());
}
fn map_usdtcontract_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.usdtcontract_call_add_black_lists.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDTCONTRACT_TRACKED_CONTRACT && abi::usdtcontract_contract::functions::AddBlackList::match_call(call))
                .filter_map(|call| {
                    match abi::usdtcontract_contract::functions::AddBlackList::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdtcontractAddBlackListCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_evil_user: decoded_call.u_evil_user,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdtcontract_call_approves.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDTCONTRACT_TRACKED_CONTRACT && abi::usdtcontract_contract::functions::Approve::match_call(call))
                .filter_map(|call| {
                    match abi::usdtcontract_contract::functions::Approve::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdtcontractApproveCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_spender: decoded_call.u_spender,
                                u_value: decoded_call.u_value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdtcontract_call_deprecates.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDTCONTRACT_TRACKED_CONTRACT && abi::usdtcontract_contract::functions::Deprecate::match_call(call))
                .filter_map(|call| {
                    match abi::usdtcontract_contract::functions::Deprecate::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdtcontractDeprecateCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_upgraded_address: decoded_call.u_upgraded_address,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdtcontract_call_destroy_black_funds.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDTCONTRACT_TRACKED_CONTRACT && abi::usdtcontract_contract::functions::DestroyBlackFunds::match_call(call))
                .filter_map(|call| {
                    match abi::usdtcontract_contract::functions::DestroyBlackFunds::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdtcontractDestroyBlackFundsCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_black_listed_user: decoded_call.u_black_listed_user,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdtcontract_call_issues.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDTCONTRACT_TRACKED_CONTRACT && abi::usdtcontract_contract::functions::Issue::match_call(call))
                .filter_map(|call| {
                    match abi::usdtcontract_contract::functions::Issue::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdtcontractIssueCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdtcontract_call_pauses.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDTCONTRACT_TRACKED_CONTRACT && abi::usdtcontract_contract::functions::Pause::match_call(call))
                .filter_map(|call| {
                    match abi::usdtcontract_contract::functions::Pause::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdtcontractPauseCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdtcontract_call_redeems.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDTCONTRACT_TRACKED_CONTRACT && abi::usdtcontract_contract::functions::Redeem::match_call(call))
                .filter_map(|call| {
                    match abi::usdtcontract_contract::functions::Redeem::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdtcontractRedeemCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdtcontract_call_remove_black_lists.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDTCONTRACT_TRACKED_CONTRACT && abi::usdtcontract_contract::functions::RemoveBlackList::match_call(call))
                .filter_map(|call| {
                    match abi::usdtcontract_contract::functions::RemoveBlackList::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdtcontractRemoveBlackListCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_cleared_user: decoded_call.u_cleared_user,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdtcontract_call_set_params.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDTCONTRACT_TRACKED_CONTRACT && abi::usdtcontract_contract::functions::SetParams::match_call(call))
                .filter_map(|call| {
                    match abi::usdtcontract_contract::functions::SetParams::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdtcontractSetParamsCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_basis_points: decoded_call.new_basis_points.to_string(),
                                new_max_fee: decoded_call.new_max_fee.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdtcontract_call_transfers.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDTCONTRACT_TRACKED_CONTRACT && abi::usdtcontract_contract::functions::Transfer::match_call(call))
                .filter_map(|call| {
                    match abi::usdtcontract_contract::functions::Transfer::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdtcontractTransferCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_to: decoded_call.u_to,
                                u_value: decoded_call.u_value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdtcontract_call_transfer_froms.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDTCONTRACT_TRACKED_CONTRACT && abi::usdtcontract_contract::functions::TransferFrom::match_call(call))
                .filter_map(|call| {
                    match abi::usdtcontract_contract::functions::TransferFrom::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdtcontractTransferFromCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_from: decoded_call.u_from,
                                u_to: decoded_call.u_to,
                                u_value: decoded_call.u_value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdtcontract_call_transfer_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDTCONTRACT_TRACKED_CONTRACT && abi::usdtcontract_contract::functions::TransferOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::usdtcontract_contract::functions::TransferOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdtcontractTransferOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_owner: decoded_call.new_owner,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdtcontract_call_unpauses.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDTCONTRACT_TRACKED_CONTRACT && abi::usdtcontract_contract::functions::Unpause::match_call(call))
                .filter_map(|call| {
                    match abi::usdtcontract_contract::functions::Unpause::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdtcontractUnpauseCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}

#[substreams::handlers::map]
fn map_events_calls(
    events: contract::Events,
    calls: contract::Calls,
) -> Result<contract::EventsCalls, substreams::errors::Error> {
    Ok(contract::EventsCalls {
        events: Some(events),
        calls: Some(calls),
    })
}
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_usdtcontract_events(&blk, &mut events);
    Ok(events)
}
#[substreams::handlers::map]
fn map_calls(blk: eth::Block) -> Result<contract::Calls, substreams::errors::Error> {
let mut calls = contract::Calls::default();
    map_usdtcontract_calls(&blk, &mut calls);
    Ok(calls)
}

