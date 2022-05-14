use anchor_lang::prelude::*;

#[repr(C)]
#[account]
pub struct BookingWallet{
    pub patient: Pubkey,
    pub technician: Pubkey,
    pub technician_bloodtest_fee: u64,
    pub booking_wallet_bump: u8,
    _reserved: [u8; 7],
}

const MAX_PUBKEY_LENGTH: usize = 32;
const MAX_SIZE_IU64: usize = 8;
const BUMP_LENGTH: usize = 1;
const BOOKINGWALLET_RESERVED_SIZE: usize = 7;

impl BookingWallet{
    pub const LEN: usize = 2 * MAX_PUBKEY_LENGTH
                        + MAX_SIZE_IU64
                        + BUMP_LENGTH
                        + BOOKINGWALLET_RESERVED_SIZE;
}