use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct VerifyBloodtestReport <'info>{
    #[account(
        mut,
        has_one = patient, 
        seeds = [b"bloodtest-report", bloodtest_report.patient.as_ref()], 
        bump = bloodtest_report.bloodtest_report_bump
    )]
    pub bloodtest_report: Account<'info, BloodtestReport>,
    #[account(mut)]
    pub patient_profile: Account<'info, PatientProfile>,
    pub patient: Signer<'info>,
}


pub fn handler(ctx: Context<VerifyBloodtestReport>) -> Result<()>{
    let bloodtest_report = &mut ctx.accounts.bloodtest_report;
        bloodtest_report.patient_verified = true;
        Ok(())
    
}
