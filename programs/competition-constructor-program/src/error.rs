use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized")]
    Unauthorized,          // 6000
    #[msg("Deprecated")]
    Deprecated,            // 6001
}