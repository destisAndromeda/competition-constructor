use anchor_lang::prelude::*;

use crate::error::*;

#[account]
#[derive(InitSpace)]
pub struct ProgramConfig {
    /// Owners of Platform
    pub authority: Pubkey,

    /// For Constructor PDA seeds
    pub creator_key: Pubkey,

    /// For fee collections
    pub treasury: Pubkey,

    /// For future fields
    pub _reserved: [u8; 64],

    /// Bump for ProgramConfig account PDA
    pub bump: u8,
}

impl ProgramConfig {
    pub fn invariant(&self) -> Result<()> {
        require_keys_neq!(
            self.authority,
            Pubkey::default(),
            CustomError::InvalidAccount,
        );
        
        require_keys_neq!(
            self.creator_key,
            Pubkey::default(),
            CustomError::InvalidAccount,
        );

        require_keys_neq!(
            self.treasury,
            Pubkey::default(),
            CustomError::InvalidAccount,
        );

        require_keys_neq!(
            self.treasury,
            self.creator_key,
            CustomError::SameAccounts,
        );

        Ok(())
    }
}