use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct InitDoctorProfile <'info>{
    #[account(init, 
        payer = doctor, 
        space = 8 + DoctorProfile::LEN, 
        seeds = [b"doctor-profile".as_ref(), doctor.key().as_ref()], 
        bump
    )]
    pub doctor_profile: Account<'info, DoctorProfile>,
    #[account(mut)]
    pub doctor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitDoctorProfile>, doctor_name: String, doctor_sex: String, doctor_dob: String, doctor_experience_months: u32, doctor_gc_fee: u128, doctor_licence:bool, doctor_gc_expiry_time: i64 ) -> Result<()>{
    let doctor_profile = &mut ctx.accounts.doctor_profile;
    let doctor = &ctx.accounts.doctor;
    let clock: Clock = Clock::get().unwrap();
    if doctor_name.as_bytes().len() > 128 {
        return Err(ErrorCode::NameTooLong.into())
    }
    if doctor_sex.as_bytes().len() > 16 {
        return Err(ErrorCode::SexTooLong.into())
    }
    if doctor_dob.as_bytes().len() > 16 {
        return Err(ErrorCode::DOBTooLong.into())
    }
    doctor_profile.doctor = doctor.key();
    doctor_profile.doctor_name = doctor_name;  
    doctor_profile.doctor_sex = doctor_sex;  
    doctor_profile.doctor_dob = doctor_dob;  
    doctor_profile.doctor_experience_months = doctor_experience_months;
    doctor_profile.doctor_gc_fee = doctor_gc_fee;
    doctor_profile.doctor_gc_expiry_time = doctor_gc_expiry_time;
    doctor_profile.sign_up_date = clock.unix_timestamp;
    doctor_profile.doctor_licence = doctor_licence;
    doctor_profile.doctor_profile_bump = *ctx.bumps.get("doctor_profile").unwrap();
    Ok(())

}