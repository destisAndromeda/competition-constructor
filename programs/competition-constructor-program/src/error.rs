use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized")]
    Unauthorized,          // 6000
    #[msg("Deprecated Address")]
    DeprecatedAddress,     // 6001
    #[msg("Invalid Amount")]
    InvalidAmount,         // 6002
}