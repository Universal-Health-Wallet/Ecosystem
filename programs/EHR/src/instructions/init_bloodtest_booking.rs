use anchor_lang::prelude::*;
use crate::state::*;
use anchor_spl::token::{self, SetAuthority, Mint, Token, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;
//use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct InitBloodtestBooking <'info>{
    #[account(init,
        payer = patient,
        space = 8 + BloodtestReport::LEN,
        seeds = [b"bloodtest-report", patient.key().as_ref()],
        bump
    )]
    pub bloodtest_report: Account<'info, BloodtestReport>,
    #[account(mut)]
    pub patient_profile: Account<'info, PatientProfile>,
    #[account(mut)]
    pub technician_profile: Account<'info, TechnicianProfile>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        seeds = [b"token-seed".as_ref()],
        bump,
        payer = patient,
        token::mint = mint,
        token::authority = patient,
    )]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(mut,
        constraint = patient_deposit_token_account.amount >= bloodtest_report.technician_bloodtest_fee
    )]
    pub patient_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub patient: Signer<'info>,
    pub system_program: Program<'info, System>, 
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}
impl<'info> InitBloodtestBooking<'info> {
    fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self
                .patient_deposit_token_account
                .to_account_info()
                .clone(),
            to: self.vault_account.to_account_info().clone(),
            authority: self.patient.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.vault_account.to_account_info().clone(),
            current_authority: self.patient.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
}
pub fn handler(ctx: Context<InitBloodtestBooking>) -> Result<()> {
    const BLOODTEST_PDA_SEED: &[u8] = b"bloodtest-report";
    const UHW_DAO_SHARE_BASISPOINTS: u16 = 50;
    let bloodtest_report = &mut ctx.accounts.bloodtest_report;
    let technician_profile = &ctx.accounts.technician_profile;
    let patient = &ctx.accounts.patient;
    let clock: Clock = Clock::get().unwrap();
    
    bloodtest_report.patient = patient.key();
    bloodtest_report.technician = technician_profile.key();
    bloodtest_report.technician_bloodtest_fee = technician_profile.technician_bloodtest_fee;
    bloodtest_report.patient_deposit_token_account = *ctx.accounts.patient_deposit_token_account.to_account_info().key;
    bloodtest_report.patient_booking_time = clock.unix_timestamp;
    bloodtest_report.technician_bloodtest_expiry_time = technician_profile.technician_bloodtest_expiry_time;
    bloodtest_report.patient_verified = false;
    bloodtest_report.bloodtest_report_bump = *ctx.bumps.get("bloodtest_report").unwrap();
    bloodtest_report.uhw_dao_share = UHW_DAO_SHARE_BASISPOINTS;
    let (vault_authority, _vault_authority_bump) =
            Pubkey::find_program_address(&[BLOODTEST_PDA_SEED], ctx.program_id);
        token::set_authority(
            ctx.accounts.into_set_authority_context(),
            AuthorityType::AccountOwner,
            Some(vault_authority),
        )?;

        token::transfer(
            ctx.accounts.into_transfer_to_pda_context(),
            ctx.accounts.bloodtest_report.technician_bloodtest_fee,
        )?;
    Ok(())
}