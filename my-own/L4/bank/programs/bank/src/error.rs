use anchor_lang::prelude::*;

#[error_code]
pub enum BankError
{
   #[msg("No the Good owner!")]
   OwnerMismatch,
   #[msg("Insufficient Funds!")]
   InsufficientFunds,
}