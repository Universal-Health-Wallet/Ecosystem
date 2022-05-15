use anchor_lang::prelude::*;
use instructions::*;
pub mod utils;

declare_id!("3ajwFh1gw3din2vueAXQshpQRJ1VjD3DrWx72B7Ctm87");
pub mod instructions;
pub mod state;
pub mod error;

#[program]
pub mod ehr {
    use super::*;

    pub fn init_bloodtest_booking(ctx: Context<InitBloodtestBooking>) -> Result<()>{
        instructions::init_bloodtest_booking::handler(ctx)
    }
    pub fn init_doctor_profile(ctx: Context<InitDoctorProfile>,  doctor_name: String, doctor_sex: String, doctor_dob: String, doctor_experience_months: u32, doctor_gc_fee: u64, doctor_licence: bool, doctor_gc_expiry_time: i64) -> Result<()>{
        instructions::init_doctor_profile::handler(ctx, doctor_name, doctor_sex, doctor_dob, doctor_experience_months, doctor_gc_fee, doctor_licence, doctor_gc_expiry_time)
    }
    pub fn init_gc_booking(ctx: Context<InitGcBooking>, patient_comments: String) -> Result<()>{
        instructions::init_gc_booking::handler(ctx, patient_comments)
    }
    pub fn init_patient_profile(ctx: Context<InitPatientProfile>, name: String, sex: String, dob: String) -> Result<()> {
        instructions::init_patient_profile::handler(ctx, name, sex, dob)
     }
    pub fn init_technician_profile(ctx: Context<InitTechnicianProfile>, technician_name: String, technician_sex: String, technician_dob: String, technician_experience_months: u32, technician_bloodtest_fee: u64, technician_licence: bool, technician_bloodtest_expiry_time: i64) -> Result<()>{
        instructions::init_technician_profile::handler(ctx, technician_name, technician_sex, technician_dob, technician_experience_months, technician_bloodtest_fee, technician_licence, technician_bloodtest_expiry_time)
    }
    pub fn update_bloodtest_report(ctx: Context<UpdateBloodtestReport>,  white_blood_cells: u128, red_blood_cells: u128, blood_platelets: u128, technician_comments: String, bloodtest_report_ipfs_hash: String) -> Result<()>{
        instructions::update_bloodtest_report::handler(ctx, white_blood_cells, red_blood_cells, blood_platelets, technician_comments, bloodtest_report_ipfs_hash)
    }
    pub fn update_gc_report(ctx: Context<UpdateGcReport>, blood_pressure: u32, blood_sugar: u32, temperature: u32, doctor_comments: String, gc_report_ipfs_hash: String) -> Result<()>{
        instructions::update_gc_report::handler(ctx, blood_pressure, blood_sugar, temperature, doctor_comments, gc_report_ipfs_hash)
    }
    pub fn verify_bloodtest_report(ctx: Context<VerifyBloodtestReport>) -> Result<()>{
        instructions::verify_bloodtest_report::handler(ctx)
    }
    pub fn verify_gc_report(ctx: Context<VerifyGcReport>) -> Result<()>{
        instructions::verify_gc_report::handler(ctx)
    }
}
