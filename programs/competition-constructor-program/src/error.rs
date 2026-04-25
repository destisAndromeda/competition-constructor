use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized")]
    Unauthorized,                  // 6000
    
    #[msg("Inactive")]
    Inactive,                      // 6001
    
    #[msg("Depercated Address")]
    DeprecatedAddress,             // 6002
    
    #[msg("Invalid Amount")]
    InvalidAmount,                 // 6003
    
    #[msg("Overflow")]
    Overflow,                      // 6004
    
    #[msg("Winner Is Not Determine")]
    WinnerIsNotDetermine,          // 6005
    
    #[msg("Invalid Account")]
    InvalidAccount,                // 6006

    #[msg("Same Accounts")]
    SameAccounts,                  // 6007
}