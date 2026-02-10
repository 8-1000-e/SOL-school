use anchor_lang::prelude::*;

declare_id!("9yn4yWPLgqnmJmyvXMzg7x5qPut9QN3fFFrx4aQ9Bxiw");

#[program]
pub mod game {
    use super::*;

    pub fn create_user_stats(ctx: Context<Initialize>, name: String) -> Result<()> 
    {
        ctx.accounts.user_stats.name = name;
        ctx.accounts.user_stats.level = 0;
        ctx.accounts.user_stats.bump = ctx.bumps.user_stats;
        Ok(())
    }

    pub fn change_user_name(ctx: Context<ChangeName>, name: String) -> Result<()> 
    {
        ctx.accounts.user_stats.name = name;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info>
{
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + UserStats::INIT_SPACE,
        seeds = [b"user-stats", payer.key().as_ref()],
        bump,
    )]
    pub user_stats: Account<'info, UserStats>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ChangeName<'info>
{
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user-stats", payer.key().as_ref()],
        bump = user_stats.bump,
    )]
    pub user_stats: Account<'info, UserStats>,
}

#[account]
#[derive(InitSpace)]
pub struct UserStats
{
    pub level: u16,
    #[max_len(20)]
    pub name: String,
    pub bump: u8
}
