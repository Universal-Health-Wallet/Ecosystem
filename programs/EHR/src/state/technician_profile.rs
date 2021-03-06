use anchor_lang::prelude::*;

#[repr(C)]
#[account]
pub struct TechnicianProfile{
    pub technician: Pubkey,
    pub technician_name: String,
    pub technician_sex: String,
    pub technician_dob: String,
    pub technician_experience_months: u32, 
    pub technician_bloodtest_fee: u64,
    pub technician_bloodtest_expiry_time: i64,
    pub sign_up_date: i64,
    pub technician_licence: bool,
    pub technician_profile_bump: u8,
    _reserved: [u8; 6],
}

const MAX_PUBKEY_LENGTH: usize = 32;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TECHNICIAN_NAME_LENGTH: usize = 128;
const MAX_TECHNICIAN_SEX_LENGTH: usize = 16;
const MAX_TECHNICIAN_DOB_LENGTH: usize = 16;
const MAX_BOOLSIZE: usize = 1;
const MAX_SIZE_IU32: usize = 4;
const MAX_SIZE_IU64: usize = 8; //sign_up_date
const BUMP_LENGTH: usize = 1;
const TECHNICIANPROFILE_RESERVED_SIZE: usize = 6;

impl TechnicianProfile{
    pub const LEN: usize = MAX_PUBKEY_LENGTH
                        + STRING_LENGTH_PREFIX + MAX_TECHNICIAN_NAME_LENGTH
                        + STRING_LENGTH_PREFIX + MAX_TECHNICIAN_SEX_LENGTH
                        + STRING_LENGTH_PREFIX + MAX_TECHNICIAN_DOB_LENGTH
                        + MAX_SIZE_IU32
                        + 3* MAX_SIZE_IU64
                        + MAX_BOOLSIZE
                        + BUMP_LENGTH
                        + TECHNICIANPROFILE_RESERVED_SIZE;
}