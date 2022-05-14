use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct VerifyGcReport <'info>{
    #[account(
        mut, 
        has_one = patient,
        seeds = [b"general-consultancy".as_ref(), general_consultancy.patient.as_ref()], 
        bump = general_consultancy.general_consultancy_bump
    )]
    pub general_consultancy: Account<'info, GeneralConsultancy>,
    pub patient_profile: Account<'info, DoctorProfile>,
    pub patient: Signer<'info>,
}

pub fn handler(ctx: Context<VerifyGcReport>) -> Result<()>{
    let general_consultancy = &mut ctx.accounts.general_consultancy;
        general_consultancy.patient_verified = true;
        Ok(())
    
}
