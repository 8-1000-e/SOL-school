use anchor_lang::prelude::*;
use crate::{error::BankError, BankAccount};

pub fn _withdraw (ctx: Context<Withdraw>, amount: u64) -> Result<()>
{
    require!(ctx.accounts.bank_account.owner == ctx.accounts.payer.key(), BankError::OwnerMismatch);

    let rent = Rent::get()?.minimum_balance(ctx.accounts.bank_account.to_account_info().data_len());
    if **ctx.accounts.bank_account.to_account_info().lamports.borrow() - rent < amount {
        return Err(BankError::InsufficientFunds.into());
    }

    **ctx.accounts.bank_account.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.payer.to_account_info().try_borrow_mut_lamports()? += amount;
    ctx.accounts.bank_account.balance -= amount;
    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info>
{
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub bank_account: Account<'info, BankAccount>,
}