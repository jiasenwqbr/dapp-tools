use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    if system_program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId);
    }

    if accounts.len() < 4 {
        msg!("This instruction requires 4 accounts:");
        msg!("  payer, account_to_create, account_to_change, system_program");
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    // Accounts passed in a vector must be in the expected order.
    let accounts_iter = &mut accounts.iter();
    let _payer = next_account_info(accounts_iter)?;
    let account_to_create = next_account_info(accounts_iter)?;
    let account_to_change = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // You can make sure an account has NOT been initialized.
    msg!("New account: {}", account_to_create.key);
    if account_to_create.lamports() != 0 {
        msg!("The program expected the account to create to not yet be initialized.");
        return Err(ProgramError::AccountAlreadyInitialized);
    };

    // If we want to modify an account's data, it must be owned by our program.
    if account_to_change.owner != program_id {
        msg!("Account to change does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId);
    };

    // You can also check pubkeys against constants.
    if system_program.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    };

    Ok(())
}
