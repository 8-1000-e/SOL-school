use anchor_lang::prelude::*;
use crate::state::Event;
use crate::errors::EventError;

pub fn _initialize(ctx: Context<Initialise>, name: String, desc: String, t_cost: u64, ticket_available: u64, start_t: i64) -> Result<()>
{
    require!(name.len() <= 15, EventError::NameTooLong);
    require!(desc.len() <= 150, EventError::DescTooLong);
    require!(start_t > Clock::get()?.unix_timestamp, EventError::TimeError);
    require!(ticket_available > 0 , EventError::NotEnoughTickets);

    ctx.accounts.event.name = name;
    ctx.accounts.event.description = desc;
    ctx.accounts.event.ticket_cost = t_cost;
    ctx.accounts.event.ticket_available = ticket_available;
    ctx.accounts.event.start_time = start_t;
    ctx.accounts.event.owner = ctx.accounts.event_organizer.key();
    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialise<'info>
{
    #[account(mut)]
    pub event_organizer: Signer<'info>,
 
    #[account(
        init,
        payer = event_organizer,
        space = 8 + Event::INIT_SPACE,
        seeds = [b"event", name.as_bytes(), event_organizer.key().as_ref()],
        bump,
    )]
    pub event: Account<'info, Event>,
    pub system_program: Program<'info, System>
}