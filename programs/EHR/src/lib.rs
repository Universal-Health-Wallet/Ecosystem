use anchor_lang::prelude::*;

use instructions::*;

declare_id!("3ajwFh1gw3din2vueAXQshpQRJ1VjD3DrWx72B7Ctm87");
pub mod instructions;
pub mod state;
pub mod error;

#[program]
pub mod ehr {
    use super::*;

    pub fn init_patient_ehr(ctx: Context<InitPatientEhr>, name: String, sex: String, dob: String) -> Result<()> {
       instructions::init_patient_ehr::handler(ctx, name, sex, dob)
    }
}
