use anchor_lang::prelude::*;
#[repr(C)]
#[account]
pub struct PatientProfile{
    pub patient: Pubkey,
    pub name: String,
    pub sex: String,
    pub dob: String, 
    pub signup_date: i64,
    pub height: u32,
    pub weight: u32,
    pub patient_profile_bump: u8,
    _reserved: [u8; 7],
}

const MAX_SIZE_IU64: usize = 8; //start_date,dob
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_NAME_LENGTH: usize = 128;
const MAX_SEX_LENGTH: usize = 16;
const MAX_DOB_LENGTH: usize = 16;
const MAX_PUBKEY_LENGTH: usize = 32;
const MAX_SIZE_IU32: usize = 4;
const BUMP_LENGTH: usize = 1;
const PATIENTPROFILE_RESERVED_SIZE: usize = 7;

impl PatientProfile{
    pub const LEN: usize = MAX_PUBKEY_LENGTH
                        + STRING_LENGTH_PREFIX + MAX_NAME_LENGTH
                        + STRING_LENGTH_PREFIX + MAX_SEX_LENGTH
                        + STRING_LENGTH_PREFIX + MAX_DOB_LENGTH
                        + MAX_SIZE_IU64
                        + 2* MAX_SIZE_IU32
                        + BUMP_LENGTH
                        + PATIENTPROFILE_RESERVED_SIZE;
}