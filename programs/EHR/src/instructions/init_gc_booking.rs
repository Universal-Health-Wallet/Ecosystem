use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct InitGcBooking <'info>{
    #[account(init, 
        payer = patient, 
        space = 8  + GeneralConsultancy::LEN,
        seeds = [b"general-consultancy".as_ref(), patient_profile.patient.as_ref()], 
        bump
    )]
    pub general_consultancy: Account<'info, GeneralConsultancy>,
    #[account(mut)]
    pub patient_profile: Account<'info, PatientProfile>,
    #[account(mut)]
    pub doctor_profile: Account<'info, DoctorProfile>,
    /// CHECK: wallet can be any account and is not written to or read
    #[account(mut)]
    pub uhw_dao_wallet: UncheckedAccount<'info>,
    #[account(mut)]
    pub patient: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitGcBooking>, patient_comments: String) -> Result<()>{
    let general_consultancy = &mut ctx.accounts.general_consultancy;
    let doctor_profile = &ctx.accounts.doctor_profile;
    let patient = &ctx.accounts.patient;
    let clock: Clock = Clock::get().unwrap();  
    
    if patient_comments.as_bytes().len() > 512 {
        return Err(ErrorCode::PatientCommentsLong.into())
    }
    general_consultancy.patient = patient.key();
    general_consultancy.doctor = doctor_profile.key();
    general_consultancy.doctor_gc_fee = doctor_profile.doctor_gc_fee;
    general_consultancy.booking_time = clock.unix_timestamp;
    general_consultancy.general_consultancy_expiry_time = doctor_profile.doctor_gc_expiry_time;
    general_consultancy.patient_comments = patient_comments;
    general_consultancy.patient_verified = false;
    
    general_consultancy.general_consultancy_bump = *ctx.bumps.get("general_consultancy").unwrap();
    Ok(())
    
}
