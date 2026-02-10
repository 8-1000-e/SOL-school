use anchor_lang::prelude::*;
use puppet::cpi::accounts::SetData;
use puppet::program::Puppet;
use puppet::{self, Data};

declare_id!("D1ngLJ1DdNvYFLfQTQoPozU5tps3Y8EkepB5oP84BEfF");

#[program]
pub mod puppet_master {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> 
    {
        Ok(())
    }

    pub fn pull_strings(ctx: Context<PullString>, value: u64) -> Result<()> 
    {
        let cpi_program = ctx.accounts.puppet_program.to_account_info();
        let cpi_account = SetData{
            data: ctx.accounts.data.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_account);
        puppet::cpi::set_data(cpi_ctx, value)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct PullString<'info>
{
    #[account(mut)]
    pub data: Account<'info, Data>,

    pub puppet_program: Program<'info, Puppet>
}
