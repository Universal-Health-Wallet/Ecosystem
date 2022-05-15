use anchor_lang::prelude::*;

#[repr(C)]
#[account]
pub struct BloodtestReport{
    pub patient: Pubkey,
    pub technician: Pubkey,
    pub patient_deposit_token_account: Pubkey,
    pub technician_bloodtest_fee: u64,
    pub patient_booking_time: i64,
    pub technician_bloodtest_expiry_time: i64,
    pub uhw_dao_share: u64,
    pub red_blood_cells: u128,
    pub white_blood_cells: u128,
    pub blood_platelets: u128,
    pub technician_comments: String,
    pub bloodtest_report_ipfs_hash: String,
    pub patient_verified: bool,
    pub bloodtest_report_bump: u8,
    _reserved: [u8; 4],
}

const MAX_PUBKEY_LENGTH: usize = 32;
const MAX_SIZE_IU128: usize = 16;
const MAX_SIZE_IU64: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TECH_COMMENTS_LENGTH: usize = 512;
const MAX_IPFS_HASH: usize = 128;
const MAX_BOOLSIZE: usize = 1;
const BUMP_LENGTH: usize = 1;
const BLOODTESTREPORT_RESERVED_SIZE: usize = 4;

impl BloodtestReport{
    pub const LEN: usize = 3 * MAX_PUBKEY_LENGTH
                        + 4 * MAX_SIZE_IU64
                        + 3 * MAX_SIZE_IU128
                        + STRING_LENGTH_PREFIX + MAX_TECH_COMMENTS_LENGTH
                        + STRING_LENGTH_PREFIX + MAX_IPFS_HASH
                        + MAX_BOOLSIZE
                        + BUMP_LENGTH
                        + BLOODTESTREPORT_RESERVED_SIZE;
}