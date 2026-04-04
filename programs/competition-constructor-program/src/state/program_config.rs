use anchor_lang::prelude::*;

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