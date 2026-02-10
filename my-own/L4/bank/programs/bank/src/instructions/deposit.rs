use anchor_lang::prelude::*;
use crate::BankAccount;


pub fn _deposit (ctx: Context<Deposit>, amount: u64) -> Result<()>
{
    let txn = anchor_lang::solana_program::system_instruction::transfer(
    &ctx.accounts.payer.key(),    // source
    &ctx.accounts.bank_account.key(),    // destination
    amount,
    );

    anchor_lang::solana_program::program::invoke(
        &txn,
        &[
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.bank_account.to_account_info(),
        ],
    )?;

    ctx.accounts.bank_account.balance += amount;
    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info>
{
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub bank_account: Account<'info, BankAccount>,
    pub system_program: Program<'info, System>
}