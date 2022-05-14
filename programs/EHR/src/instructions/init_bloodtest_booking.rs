use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct InitBloodtestBooking <'info>{
    #[account(init,
        payer = patient,
        space = 8 + BloodtestReport::LEN,
        seeds = [b"bloodtest-report", patient_profile.patient.as_ref()],
        bump
    )]
    pub bloodtest_report: Account<'info, BloodtestReport>,
    #[account(mut)]
    pub patient_profile: Account<'info, PatientProfile>,
    #[account(mut)]
    pub technician_profile: Account<'info, TechnicianProfile>,
    /// CHECK: wallet can be any account and is not written to or read
    #[account(mut)]
    pub uhw_dao_wallet: UncheckedAccount<'info>,
    #[account(mut)]
    pub patient: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn handler(ctx: Context<InitBloodtestBooking>) -> Result<()>
{
    let bloodtest_report = &mut ctx.accounts.bloodtest_report;
    let technician_profile = &ctx.accounts.technician_profile;
    let patient = &ctx.accounts.patient;
    let clock: Clock = Clock::get().unwrap();
    bloodtest_report.patient = patient.key();
    bloodtest_report.technician = technician_profile.key();
    bloodtest_report.technician_bloodtest_fee = technician_profile.technician_bloodtest_fee;
    bloodtest_report.patient_booking_time = clock.unix_timestamp;
    bloodtest_report.technician_bloodtest_expiry_time = technician_profile.technician_bloodtest_expiry_time;
    bloodtest_report.patient_verified = false;
    bloodtest_report.bloodtest_report_bump = *ctx.bumps.get("bloodtest_report").unwrap();
    Ok(())
}