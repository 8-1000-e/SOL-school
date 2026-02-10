use anchor_lang::prelude::*;

pub const MAX_NAME_LEN: usize = 15;
pub const MAX_DESCRIPTION_LEN: usize = 150;

#[account]
#[derive(InitSpace)]
pub struct Event
{
    #[max_len(15)]
    pub name: String,
    #[max_len(150)]
    pub description: String,
    pub ticket_cost: u64,
    pub ticket_available: u64,
    pub owner: Pubkey,
    pub start_time: i64,
}

#[account]
#[derive(InitSpace)]
pub struct Ticket
{
    pub owner: Pubkey,
    pub event: Pubkey,
    pub price: u64
}