use anchor_lang::prelude::*;
use crate::state::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::utils::*;

#[derive(Accounts)]
pub struct VerifyGcReport <'info>{
    #[account(
        mut, 
        constraint = general_consultancy.patient_deposit_token_account == *patient_deposit_token_account.to_account_info().key,
        constraint = general_consultancy.patient == *patient.key,
        seeds = [b"general-consultancy".as_ref(), general_consultancy.patient.as_ref()], 
        bump = general_consultancy.general_consultancy_bump
    )]
    pub general_consultancy: Account<'info, GeneralConsultancy>,
    #[account(mut)]
    pub vault_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub vault_authority: AccountInfo<'info>,
    pub patient_profile: Account<'info, DoctorProfile>,
    /// CHECK: wallet can be any account and is not written to or read
    #[account(mut)]
    pub patient_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub doctor_receive_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dao_token_account: Account<'info, TokenAccount>,
    pub patient: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
impl<'info> VerifyGcReport<'info> {
    fn into_transfer_to_doctor_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self.doctor_receive_token_account.to_account_info().clone(),
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


pub fn handler(ctx: Context<VerifyGcReport>) -> Result<()>{
    const GC_PDA_SEED: &[u8] = b"general-consultancy";
    let general_consultancy = &mut ctx.accounts.general_consultancy;
    general_consultancy.patient_verified = true;
    let dao_share = calculate_share(general_consultancy.doctor_gc_fee, general_consultancy.uhw_dao_share)?;
    let doctor_share = calculate_share(general_consultancy.doctor_gc_fee, 100- general_consultancy.uhw_dao_share)?;
    
    let (_vault_authority, vault_authority_bump) =
        Pubkey::find_program_address(&[GC_PDA_SEED], ctx.program_id);
    let authority_seeds = &[&GC_PDA_SEED[..], &[vault_authority_bump]];
    token::transfer(
        ctx.accounts
            .into_transfer_to_doctor_context()
            .with_signer(&[&authority_seeds[..]]),
            doctor_share,
    )?;
    token::transfer(
        ctx.accounts
            .into_transfer_to_dao_context()
            .with_signer(&[&authority_seeds[..]]),
            dao_share,
    )?;
    
    Ok(())
}
