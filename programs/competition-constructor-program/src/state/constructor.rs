use anchor_lang::prelude::*;

use crate::error::*;

#[account]
#[derive(InitSpace)]
pub struct Constructor {
    /// Authority that can update constructor
    pub authority: Pubkey,

    /// Key for competitions PDA seeds
    pub creator_key: Pubkey,

    /// Last competition index
    pub competition_index: u64,

    /// Fee for competitions create in lamports
    pub transaction_fee: u64,

    /// Bump for Constructor account PDA seeds
    pub bump: u8,
}

impl Constructor {
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
            self.creator_key,
            self.authority,
            CustomError::InvalidAccount,
        );

        Ok(())
    }
}