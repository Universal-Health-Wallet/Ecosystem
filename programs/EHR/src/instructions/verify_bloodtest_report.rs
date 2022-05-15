use anchor_lang::prelude::*;
use crate::state::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::utils::*;

#[derive(Accounts)]
pub struct VerifyBloodtestReport <'info>{
    #[account(
        mut,
        constraint = bloodtest_report.patient_deposit_token_account == *patient_deposit_token_account.to_account_info().key,
        constraint = bloodtest_report.patient == *patient.key,
        seeds = [b"bloodtest-report", bloodtest_report.patient.as_ref()], 
        bump = bloodtest_report.bloodtest_report_bump
    )]
    pub bloodtest_report: Account<'info, BloodtestReport>,
    #[account(mut)]
    pub vault_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub vault_authority: AccountInfo<'info>,
    #[account(mut)]
    pub patient_profile: Account<'info, PatientProfile>,
    #[account(mut)]
    pub patient_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub technician_receive_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dao_token_account: Account<'info, TokenAccount>,
    pub patient: Signer<'info>,
    pub token_program: Program<'info,Token>
}

impl<'info> VerifyBloodtestReport<'info> {
    fn into_transfer_to_technician_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self.technician_receive_token_account.to_account_info().clone(),
            authority: self.vault_authority.clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
    fn into_transfer_to_dao_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self.dao_token_account.to_account_info().clone(),
            authority: self.vault_authority.clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handler(ctx: Context<VerifyBloodtestReport>) -> Result<()>{
    let bloodtest_report = &mut ctx.accounts.bloodtest_report;
    const BLOODTEST_PDA_SEED: &[u8] = b"bloodtest-report";
    bloodtest_report.patient_verified = true;
    let dao_share = calculate_share(bloodtest_report.technician_bloodtest_fee, bloodtest_report.uhw_dao_share)?;
    let technicain_share = calculate_share(bloodtest_report.technician_bloodtest_fee, 100- bloodtest_report.uhw_dao_share)?;
    let (_vault_authority, vault_authority_bump) = Pubkey::find_program_address(&[BLOODTEST_PDA_SEED], ctx.program_id);
    let authority_seeds = &[&BLOODTEST_PDA_SEED[..], &[vault_authority_bump]];
    token::transfer(
        ctx.accounts
            .into_transfer_to_technician_context()
            .with_signer(&[&authority_seeds[..]]),
            technicain_share,
    )?;
    token::transfer(
        ctx.accounts
            .into_transfer_to_dao_context()
            .with_signer(&[&authority_seeds[..]]),
            dao_share,
    )?;
    Ok(())
}
