use anchor_lang::prelude::*;
#[repr(C)]
#[account]
pub struct PatientEhr{
    pub patient: Pubkey,
    pub name: String,
    pub sex: String,
    pub dob: String, 
    pub start_date: i64,
    pub height: u32,
    pub weight: u32,
    pub blood_pressure: u32,
    pub blood_sugar: u32,    
    pub bump: u8,
    _reserved: [u8; 7],
}
const DISCRIMINATOR_LENGTH: usize = 8;
const MAX_TIMESTAMP_SIZE: usize = 8; //start_date,dob
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_NAME_LENGTH: usize = 128;
const MAX_SEX_LENGTH: usize = 16;
const MAX_DOB_LENGTH: usize = 16;
const MAX_PUBKEY_LENGTH: usize = 32;
const MAX_SIZE_IU32: usize = 4;
const BUMP_LENGTH: usize = 1;
const RESERVED_SIZE: usize = 7;

impl PatientEhr{
    pub const LEN: usize = DISCRIMINATOR_LENGTH
                        + MAX_PUBKEY_LENGTH
                        + STRING_LENGTH_PREFIX + MAX_NAME_LENGTH
                        + STRING_LENGTH_PREFIX + MAX_SEX_LENGTH
                        + STRING_LENGTH_PREFIX + MAX_DOB_LENGTH
                        + MAX_TIMESTAMP_SIZE
                        + 4* MAX_SIZE_IU32
                        + BUMP_LENGTH
                        + RESERVED_SIZE;
}