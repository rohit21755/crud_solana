#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod crud {
    use super::*;

  pub fn close(_ctx: Context<CloseCrud>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.crud.count = ctx.accounts.crud.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.crud.count = ctx.accounts.crud.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeCrud>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.crud.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeCrud<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + Crud::INIT_SPACE,
  payer = payer
  )]
  pub crud: Account<'info, Crud>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseCrud<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub crud: Account<'info, Crud>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub crud: Account<'info, Crud>,
}

#[account]
#[derive(InitSpace)]
pub struct Crud {
  count: u8,
}
