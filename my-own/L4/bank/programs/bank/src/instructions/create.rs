use anchor_lang::prelude::*;
use crate::{BankAccount, BANK_ACCOUNT_SEED};

pub fn _create(ctx: Context<Create>, name: String) -> Result<()>
{
    ctx.accounts.bank_account.name = name;
    ctx.accounts.bank_account.balance = 0;
    ctx.accounts.bank_account.owner = *ctx.accounts.payer.key;
    Ok(())
}

#[derive(Accounts)]
pub struct Create<'info>
{
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + BankAccount::INIT_SPACE,
        seeds = [BANK_ACCOUNT_SEED.as_bytes(), payer.key().as_ref()],
        bump
    )]
    pub bank_account: Account<'info, BankAccount>,
    pub system_program: Program<'info, System>
}