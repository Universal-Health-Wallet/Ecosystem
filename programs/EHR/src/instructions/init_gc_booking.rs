use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;
use anchor_spl::token::{self, SetAuthority, Mint, Token, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;


#[derive(Accounts)]
#[instruction(initializer_amount: u64)]
pub struct InitGcBooking <'info>{
    #[account(init, 
        payer = patient, 
        space = 8  + GeneralConsultancy::LEN,
        seeds = [b"general-consultancy".as_ref(), patient_profile.patient.as_ref()], 
        bump
    )]
    pub general_consultancy: Account<'info, GeneralConsultancy>,
    #[account(mut)]
    pub patient_profile: Account<'info, PatientProfile>,
    #[account(mut)]
    pub doctor_profile: Account<'info, DoctorProfile>,
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
        constraint = patient_deposit_token_account.amount >= doctor_profile.doctor_gc_fee
    )]
    pub patient_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub patient: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}

impl<'info> InitGcBooking<'info> {
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


pub fn handler(ctx: Context<InitGcBooking>, patient_comments: String) -> Result<()>{
    const GC_PDA_SEED: &[u8] = b"general-consultancy";
    const UHW_DAO_SHARE_BASISPOINTS: u64 = 5;
    let general_consultancy = &mut ctx.accounts.general_consultancy;
    let doctor_profile = &ctx.accounts.doctor_profile;
    let patient = &ctx.accounts.patient;
    let clock: Clock = Clock::get().unwrap();  
    
    if patient_comments.as_bytes().len() > 512 {
        return Err(ErrorCode::PatientCommentsLong.into())
    }
    general_consultancy.patient = patient.key();
    general_consultancy.doctor = doctor_profile.key();
    general_consultancy.doctor_gc_fee = doctor_profile.doctor_gc_fee;
    general_consultancy.booking_time = clock.unix_timestamp;
    general_consultancy.general_consultancy_expiry_time = doctor_profile.doctor_gc_expiry_time;
    general_consultancy.patient_comments = patient_comments;
    general_consultancy.uhw_dao_share = UHW_DAO_SHARE_BASISPOINTS;
    general_consultancy.patient_verified = false;
    general_consultancy.general_consultancy_bump = *ctx.bumps.get("general_consultancy").unwrap();
    let (vault_authority, _vault_authority_bump) =
            Pubkey::find_program_address(&[GC_PDA_SEED], ctx.program_id);
        token::set_authority(
            ctx.accounts.into_set_authority_context(),
            AuthorityType::AccountOwner,
            Some(vault_authority),
        )?;

        token::transfer(
            ctx.accounts.into_transfer_to_pda_context(),
            ctx.accounts.general_consultancy.doctor_gc_fee,
        )?; 
    Ok(())
}
