use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct InitPatientProfile <'info>{
    #[account(init, 
        payer = patient, 
        space = 8 + PatientProfile::LEN, 
        seeds = [b"patient-profile".as_ref(), patient.key().as_ref()], 
        bump
    )]
    pub patient_profile: Account<'info, PatientProfile>,
    #[account(mut)]
    pub patient: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitPatientProfile>, name: String, sex: String, dob: String) -> Result<()>{
    let patient_profile = &mut ctx.accounts.patient_profile;
    let clock: Clock = Clock::get().unwrap();
    if name.as_bytes().len() > 128 {
        return Err(ErrorCode::NameTooLong.into())
    }
    if sex.as_bytes().len() > 16 {
        return Err(ErrorCode::SexTooLong.into())
    }
    if dob.as_bytes().len() > 16 {
        return Err(ErrorCode::DOBTooLong.into())
    }
    
    patient_profile.patient = *ctx.accounts.patient.key;
    patient_profile.name = name;  
    patient_profile.sex = sex;  
    patient_profile.dob = dob;  
    patient_profile.signup_date = clock.unix_timestamp;
    patient_profile.patient_profile_bump = *ctx.bumps.get("patient_profile").unwrap();
    Ok(())

}