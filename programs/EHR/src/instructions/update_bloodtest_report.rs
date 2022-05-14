use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateBloodtestReport <'info>{
    #[account(
        mut,
        has_one = technician, 
        seeds = [b"bloodtest-report", bloodtest_report.patient.as_ref()], 
        bump = bloodtest_report.bloodtest_report_bump
    )]
    pub bloodtest_report: Account<'info, BloodtestReport>,
    #[account(mut)]
    pub technician_profile: Account<'info, TechnicianProfile>,
    pub technician: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateBloodtestReport>, white_blood_cells: u128, red_blood_cells: u128, blood_platelets: u128, technician_comments: String ) -> Result<()>{
    let bloodtest_report = &mut ctx.accounts.bloodtest_report;
        bloodtest_report.red_blood_cells = red_blood_cells;
        bloodtest_report.white_blood_cells = white_blood_cells;
        bloodtest_report.blood_platelets = blood_platelets;
        bloodtest_report.technician_comments = technician_comments;
        Ok(())
    
}
