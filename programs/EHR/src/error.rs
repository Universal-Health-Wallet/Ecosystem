use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Src Balance < LP Deposit Amount.")]
    NotEnoughBalance,
    #[msg("Name is more than 128 bytes")]
    NameTooLong,
    #[msg("DOB is more than 16 bytes")]
    DOBTooLong,
    #[msg("Sex is more than 16 bytes")]
    SexTooLong,
    #[msg("Patient Comments is more than 512 bytes")]
    PatientCommentsLong,
}