use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod instructions;

pub use instructions::*;

declare_id!("7occibxbsT7JbhBAC3BwXhzXhuR31GiKpuHNoqv7hm3X");

#[program]
pub mod ticket_registry {
    use super::*;

    pub fn initialize(ctx: Context<Initialise>, name: String, desc: String, t_cost: u64, ticket_available: u64, start_t: i64) -> Result<()>
    {
        _initialize(ctx, name, desc, t_cost, ticket_available, start_t)
    }

    pub fn buy(ctx: Context<Buy>) -> Result<()>
    {
        _buy(ctx)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()>
    {
        _withdraw(ctx, amount)
    }
}
