use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct InitTechnicianProfile <'info>{
    #[account(init,
        payer = technician,
        space = 8 + TechnicianProfile::LEN,
        seeds = [b"technician-profile", technician.key().as_ref()],
        bump
    )]
    pub technician_profile: Account<'info, TechnicianProfile>,
    #[account(mut)]
    pub technician: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitTechnicianProfile>, technician_name: String, technician_sex: String, technician_dob: String, technician_experience_months: u32, technician_bloodtest_fee: u64, technician_licence:bool, technician_bloodtest_expiry_time: i64 ) -> Result<()>{
    let technician_profile = &mut ctx.accounts.technician_profile;
    let technician = &ctx.accounts.technician;
    let clock: Clock = Clock::get().unwrap();
    technician_profile.technician = technician.key();
    technician_profile.technician_name = technician_name;
    technician_profile.technician_sex = technician_sex;
    technician_profile.technician_dob = technician_dob;
    technician_profile.technician_experience_months = technician_experience_months;
    technician_profile.technician_bloodtest_fee = technician_bloodtest_fee;
    technician_profile.technician_bloodtest_expiry_time = technician_bloodtest_expiry_time;
    technician_profile.sign_up_date = clock.unix_timestamp;
    technician_profile.technician_licence = technician_licence;
    technician_profile.technician_profile_bump = *ctx.bumps.get("technical_profile").unwrap();

    Ok(())
}