#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod bootcamptemp {
    use super::*;

  pub fn close(_ctx: Context<CloseBootcamptemp>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.bootcamptemp.count = ctx.accounts.bootcamptemp.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.bootcamptemp.count = ctx.accounts.bootcamptemp.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeBootcamptemp>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.bootcamptemp.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeBootcamptemp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + Bootcamptemp::INIT_SPACE,
  payer = payer
  )]
  pub bootcamptemp: Account<'info, Bootcamptemp>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseBootcamptemp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub bootcamptemp: Account<'info, Bootcamptemp>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub bootcamptemp: Account<'info, Bootcamptemp>,
}

#[account]
#[derive(InitSpace)]
pub struct Bootcamptemp {
  count: u8,
}
