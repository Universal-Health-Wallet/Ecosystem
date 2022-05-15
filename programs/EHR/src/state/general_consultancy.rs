use anchor_lang::prelude::*;
#[repr(C)]
#[account]
pub struct GeneralConsultancy{
    pub patient: Pubkey,
    pub doctor: Pubkey,
    pub patient_deposit_token_account: Pubkey,
    pub doctor_gc_fee: u64,
    pub booking_time: i64,
    pub general_consultancy_expiry_time: i64,
    pub uhw_dao_share: u64,
    pub patient_comments: String,
    pub blood_pressure: u32,
    pub blood_sugar: u32,
    pub temperature: u32,
    pub doctor_comments: String,
    pub gc_report_ipfs_hash: String,
    pub patient_verified: bool,
    pub general_consultancy_bump: u8,
    _reserved: [u8; 6],
}
const MAX_PUBKEY_LENGTH: usize = 32;
const MAX_SIZE_IU64: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_PAT_COMMENTS_LENGTH: usize = 512;
const MAX_SIZE_IU32: usize = 4;
const MAX_DOC_COMMENTS_LENGTH: usize = 512;
const MAX_IPFS_HASH: usize = 128;
const MAX_BOOLSIZE: usize = 1;
const BUMP_LENGTH: usize = 1;
const GENERALCONSULTANCY_RESERVE_SIZE: usize = 6;

impl GeneralConsultancy{
    pub const LEN: usize = 3 * MAX_PUBKEY_LENGTH
                    + 4 * MAX_SIZE_IU64
                    + STRING_LENGTH_PREFIX + MAX_PAT_COMMENTS_LENGTH
                    + 3 * MAX_SIZE_IU32
                    + STRING_LENGTH_PREFIX + MAX_DOC_COMMENTS_LENGTH
                    + STRING_LENGTH_PREFIX + MAX_IPFS_HASH
                    + MAX_BOOLSIZE
                    + BUMP_LENGTH
                    + GENERALCONSULTANCY_RESERVE_SIZE;
}
    