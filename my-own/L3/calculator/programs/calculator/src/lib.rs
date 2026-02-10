use anchor_lang::prelude::*;

declare_id!("8xmDPqrTYdaNHwftm2F7B1g8P5TbdEEWQP1hNbpLfw92");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> 
    {

        ctx.accounts.calc_acc.greetings = "[INITED]".to_string();
        ctx.accounts.calc_acc.result = 0;

        Ok(())
    }

    pub fn add(ctx: Context<Add>, num1: i64, num2: i64) -> Result<()>
    {
        ctx.accounts.calc_acc.result = num1 + num2;
        Ok(())
    }

    pub fn sub(ctx: Context<Sub>, num1: i64, num2: i64) -> Result<()>
    {
        ctx.accounts.calc_acc.result = num1 - num2;
        Ok(())
    }

    pub fn multiply(ctx: Context<Multiply>, num1: i64, num2: i64) -> Result<()>
    {
        ctx.accounts.calc_acc.result = num1 * num2;
        Ok(())
    }

    pub fn divide(ctx: Context<Divide>, num1: i64, num2: i64) -> Result<()>
    {
        require!(num2 != 0, ErrorMsg::DivideByZero);
        ctx.accounts.calc_acc.result = num1 / num2;
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
        space = 8 + Calculator::INIT_SPACE
    )]
    pub calc_acc: Account<'info, Calculator>,

    pub system_program: Program<'info, System>
}


#[account]
#[derive(InitSpace)]
pub struct Calculator
{
    #[max_len(50)]
    pub greetings: String,
    pub result: i64
}

#[derive(Accounts)]
pub struct Add<'info>
{
    #[account(mut)]
    pub calc_acc: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Sub<'info>
{
    #[account(mut)]
    pub calc_acc: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Multiply<'info>
{
    #[account(mut)]
    pub calc_acc: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Divide<'info>
{
    #[account(mut)]
    pub calc_acc: Account<'info, Calculator>,
}

#[error_code]
pub enum ErrorMsg {
    #[msg("Can't divide by 0")]
    DivideByZero
}