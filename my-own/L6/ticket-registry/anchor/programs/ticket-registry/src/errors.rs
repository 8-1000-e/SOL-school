use anchor_lang::prelude::*;

#[error_code]
pub enum EventError
{
    #[msg("Name must be under 16 char long!")]
    NameTooLong,
    #[msg("Description must be under 151 char long!")]
    DescTooLong,
    #[msg("Time Error!")]
    TimeError,
    #[msg("No enough ticket available!")]
    NotEnoughTickets,
    #[msg("No more ticket!")]
    NoMoreTicket,
}