use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("41PgN2DHRDQKhvGsYupHtBkr53V1kqDxwCLdGYzbjSdt");

#[program]
pub mod bank {
    use super::*;

    pub fn create(ctx: Context<Create>, name: String) -> Result<()> {
        _create(ctx, name)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        _deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        _withdraw(ctx, amount)
    }
}
