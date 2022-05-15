use anchor_lang::prelude::*;
#[repr(C)]
#[account]
pub struct DoctorProfile{
    pub doctor: Pubkey,
    pub doctor_name: String,
    pub doctor_sex: String,
    pub doctor_dob: String,
    pub doctor_experience_months: u32, 
    pub doctor_gc_fee: u64,
    pub doctor_gc_expiry_time: i64,
    pub sign_up_date: i64,
    pub doctor_licence: bool,
    pub doctor_profile_bump: u8,
    _reserved: [u8; 6],
}

const MAX_PUBKEY_LENGTH: usize = 32;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_DOCTOR_NAME_LENGTH: usize = 128;
const MAX_DOCTOR_SEX_LENGTH: usize = 16;
const MAX_DOCTOR_DOB_LENGTH: usize = 16;
const MAX_SIZE_IU32: usize = 4;
const MAX_SIZE_IU64: usize = 8;
const MAX_BOOLSIZE: usize = 1;
const BUMP_LENGTH: usize = 1;
const DOCTORPROFILE_RESERVED_SIZE: usize = 6;

impl DoctorProfile{
    pub const LEN: usize = MAX_PUBKEY_LENGTH
                        + STRING_LENGTH_PREFIX + MAX_DOCTOR_NAME_LENGTH
                        + STRING_LENGTH_PREFIX + MAX_DOCTOR_SEX_LENGTH
                        + STRING_LENGTH_PREFIX + MAX_DOCTOR_DOB_LENGTH
                        + MAX_SIZE_IU32
                        + 3 * MAX_SIZE_IU64
                        + MAX_BOOLSIZE
                        + BUMP_LENGTH
                        + DOCTORPROFILE_RESERVED_SIZE;
}