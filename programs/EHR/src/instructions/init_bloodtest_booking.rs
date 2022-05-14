use anchor_lang::prelude::*;
use crate::state::*;
use anchor_spl::token::{self,SetAuthority, Token, TokenAccount};
use spl_token::instruction::AuthorityType;
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
    #[account(mut,
        constraint = patient_deposit_token_account.amount >= bloodtest_report.technician_bloodtest_fee
    )]
    pub patient_deposit_token_account: Account<'info, TokenAccount>,
    /// CHECK: wallet can be any account and is not written to or read
    #[account(mut)]
    pub uhw_dao_wallet: UncheckedAccount<'info>,
    #[account(mut)]
    pub patient: Signer<'info>,
    pub system_program: Program<'info, System>, 
    pub token_program: Program<'info, Token>,
}
impl<'info> From<&mut InitBloodtestBooking<'info>>
    for CpiContext<'_, '_, '_, 'info, SetAuthority<'info>>
{
    fn from(accounts: &mut InitBloodtestBooking<'info>) -> Self {
        let cpi_accounts = SetAuthority {
            account_or_mint: accounts
                .patient_deposit_token_account
                .to_account_info()
                .clone(),
            current_authority: accounts.patient.to_account_info().clone(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
pub fn handler(ctx: Context<InitBloodtestBooking>) -> Result<()>
{
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
    let (pda, _bump_seed) = Pubkey::find_program_address(&[BLOODTEST_PDA_SEED], ctx.program_id);
    token::set_authority(ctx.accounts.into(), AuthorityType::AccountOwner, Some(pda))?;
    Ok(())
}