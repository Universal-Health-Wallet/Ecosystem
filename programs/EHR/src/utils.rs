use anchor_lang::prelude::*;
use crate::error::ErrorCode;
pub fn calculate_share(
    total_amount: u64,
    shares: u64,
) -> Result<u64> {
    Ok(total_amount
        .checked_mul(shares)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(100)
        .ok_or(ErrorCode::MathOverflow)?)
    }