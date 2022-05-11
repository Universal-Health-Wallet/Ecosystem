use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;


#[derive(Accounts)]
pub struct InitPatientEhr <'info>{
    #[account(mut)]
    pub patient: Signer<'info>,
    #[account(
        init, 
        payer = patient, 
        space = PatientEhr::LEN, 
        seeds = [b"patient-ehr".as_ref(), patient.key().as_ref()], 
        bump
    )]
    pub patient_ehr_account: Account<'info, PatientEhr>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitPatientEhr>, name: String, sex: String, dob: String) -> Result<()>{
    let patient_ehr_account = &mut ctx.accounts.patient_ehr_account;
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
    patient_ehr_account.patient = *ctx.accounts.patient.key;
    patient_ehr_account.name = name;  
    patient_ehr_account.sex = sex;  
    patient_ehr_account.dob = dob;  
    patient_ehr_account.start_date = clock.unix_timestamp;
    patient_ehr_account.bump = *ctx.bumps.get("patient_ehr_account").unwrap();
    Ok(())

}