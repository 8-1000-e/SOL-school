use anchor_lang::prelude::*;
use crate::state::*;

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()>
{
    ctx.accounts.event.sub_lamports(amount)?;
    ctx.accounts.owner.add_lamports(amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info>
{
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        has_one = owner
    )]
    pub event: Account<'info, Event>,
}