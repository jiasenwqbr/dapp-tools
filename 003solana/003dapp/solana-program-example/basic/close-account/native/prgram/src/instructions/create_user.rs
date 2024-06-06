use borsh::BorshSerialize;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke_signed;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;
use solana_program::{account_info::next_account_info, account_info::AccountInfo};

use crate::state::user::User;
pub fn create_user(program_id: &Pubkey, accouts: &[AccountInfo], data: User) -> ProgramResult {
    let account_iter = &mut accouts.iter();
    let target_account = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;

    let account_span = (data.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    let (_, bump) = Pubkey::find_program_address(&[User::SEED_PREFIX.as_bytes()], program_id);
    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            target_account.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            payer.clone(),
            target_account.clone(),
            system_program.clone(),
        ],
        &[&[User::SEED_PREFIX.as_bytes(), payer.key.as_ref(), &[bump]]],
    )?;

    data.serialize(&mut &mut target_account.data.borrow_mut()[..])?;

    Ok(())
}
