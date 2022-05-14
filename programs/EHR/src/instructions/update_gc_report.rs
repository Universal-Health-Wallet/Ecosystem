use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateGcReport <'info>{
    #[account(
        mut, 
        has_one = doctor,
        seeds = [b"general-consultancy".as_ref(), general_consultancy.patient.as_ref()], 
        bump = general_consultancy.general_consultancy_bump
    )]
    pub general_consultancy: Account<'info, GeneralConsultancy>,
    pub doctor_profile: Account<'info, DoctorProfile>,
    pub doctor: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateGcReport>, blood_pressure: u32, blood_sugar: u32, temperature: u32, doctor_comments: String, gc_report_ipfs_hash: String ) -> Result<()>{
    let general_consultancy = &mut ctx.accounts.general_consultancy;
    general_consultancy.blood_pressure = blood_pressure;
    general_consultancy.blood_sugar = blood_sugar;
    general_consultancy.temperature = temperature;
    general_consultancy.doctor_comments = doctor_comments;
    general_consultancy.gc_report_ipfs_hash = gc_report_ipfs_hash;
    Ok(())
}
