

pub fn calculate_secondary_shares_for_creator(
    total_amount: u64,
    seller_fee_basis_points: u64,
    shares: u64,
) -> Result<u64> {
    Ok((total_amount
        .checked_mul(seller_fee_basis_points)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(10000)
        .ok_or(ErrorCode::MathOverflow)?)
    .checked_mul(shares)
    .ok_or(ErrorCode::MathOverflow)?
    .checked_div(100)
    .ok_or(ErrorCode::MathOverflow)?)
}