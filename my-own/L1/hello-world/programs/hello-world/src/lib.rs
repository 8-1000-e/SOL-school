use anchor_lang::prelude::*;

declare_id!("BXPnxoVh4tAPBT1rzMCegrgJvQhdZzeaSx4XZfHVJP8Z");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> 
    {
        ctx.accounts.hw_acc.greetings = "Hello World !".to_string();
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
        space = 8 + HelloWorldAccount::INIT_SPACE,

    )]
    pub hw_acc: Account<'info, HelloWorldAccount>,
    pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct HelloWorldAccount
{
    #[max_len(20)]
    pub greetings: String
}