use anchor_lang::prelude::*;

declare_id!("8QJnsFS5EDRFtERd2tRVvdLD2h1Ze3iXRPdmkMEqP7T2");

#[program]
pub mod puppet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> 
    {
        ctx.accounts.data.value = 0;
        Ok(())
    }

    pub fn set_data(ctx: Context<SetData>, new_value: u64) -> Result<(u64)> 
    {
        ctx.accounts.data.value = new_value;
        Ok(new_value)
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
        space = 8 + Data::INIT_SPACE,
    )]
    pub data: Account<'info, Data>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct SetData<'info>
{
    #[account(mut)]
    pub data: Account<'info, Data>,
}

#[account]
#[derive(InitSpace)]
pub struct Data
{
    pub value: u64
}
