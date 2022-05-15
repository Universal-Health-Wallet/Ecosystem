use anchor_lang::prelude::*;
use crate::state::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};


#[derive(Accounts)]
pub struct VerifyBloodtestReport <'info>{
    #[account(
        mut,
        constraint = bloodtest_report.patient_deposit_token_account == *pda_patient_token_account.to_account_info().key,
        constraint = bloodtest_report.patient == *patient_main_account.key,
        seeds = [b"bloodtest-report", bloodtest_report.patient.as_ref()], 
        bump = bloodtest_report.bloodtest_report_bump
    )]
    pub bloodtest_report: Account<'info, BloodtestReport>,
    #[account(mut)]
    pub patient_profile: Account<'info, PatientProfile>,
    /// CHECK: wallet can be any account and is not written to or read
    #[account(mut)]
    pub patient_main_account: AccountInfo<'info>,
    #[account(mut)]
    pub pda_patient_token_account: Account<'info, TokenAccount>,
    pub doctor_receive_token_account: Account<'info, TokenAccount>,
    /// CHECK: wallet can be any account and is not written to or read
    pub pda_account: AccountInfo<'info>,
    pub patient: Signer<'info>,
    pub token_program: Program<'info,Token>
}

impl<'info> VerifyBloodtestReport<'info> {
    fn into_transfer_to_doctor_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.pda_patient_token_account.to_account_info().clone(),
            to: self.doctor_receive_token_account.to_account_info().clone(),
            authority: self.pda_account.clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handler(ctx: Context<VerifyBloodtestReport>) -> Result<()>{
    let bloodtest_report = &mut ctx.accounts.bloodtest_report;
    const BLOODTEST_PDA_SEED: &[u8] = b"bloodtest-report";
    bloodtest_report.patient_verified = true;
    let (_vault_authority, vault_authority_bump) =
            Pubkey::find_program_address(&[BLOODTEST_PDA_SEED], ctx.program_id);
        let authority_seeds = &[&BLOODTEST_PDA_SEED[..], &[vault_authority_bump]];
        token::transfer(
            ctx.accounts
                .into_transfer_to_doctor_context()
                .with_signer(&[&authority_seeds[..]]),
            ctx.accounts.bloodtest_report.technician_bloodtest_fee,
        )?;

Ok(())
}
