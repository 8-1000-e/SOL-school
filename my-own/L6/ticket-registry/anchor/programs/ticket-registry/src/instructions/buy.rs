use anchor_lang::prelude::*;
use crate::state::*;
use anchor_lang::system_program;
use crate::errors::EventError;

pub fn _buy(ctx: Context<Buy>) -> Result<()>
{
    require!(ctx.accounts.event.start_time > Clock::get()?.unix_timestamp, EventError::TimeError);
    require!(ctx.accounts.event.ticket_available >= 1, EventError::NoMoreTicket);

    ctx.accounts.event.ticket_available -= 1;

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer{
            from: ctx.accounts.ticket_payer.to_account_info(),
            to: ctx.accounts.event.to_account_info()
        }
    );

    system_program::transfer(cpi_context, ctx.accounts.event.ticket_cost)?;

    ctx.accounts.ticket.owner = ctx.accounts.ticket_payer.key();
    ctx.accounts.ticket.event = ctx.accounts.event.key();
    ctx.accounts.ticket.price = ctx.accounts.event.ticket_cost;

    Ok(())
}

#[derive(Accounts)]
pub struct Buy<'info>
{
    #[account(mut)]
    pub ticket_payer: Signer<'info>,

    #[account(mut)] 
    pub event: Account<'info, Event>,

    #[account(
        init,
        payer = ticket_payer,
        space = 8 + Ticket::INIT_SPACE,
        seeds = [b"ticket", event.key().as_ref(), ticket_payer.key().as_ref()],
        bump,
    )]
    pub ticket: Account<'info, Ticket>,
    pub system_program: Program<'info, System>
}