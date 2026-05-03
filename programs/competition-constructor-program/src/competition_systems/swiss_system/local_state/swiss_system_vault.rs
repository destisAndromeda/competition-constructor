use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    /// Winner address in the leaderboard
    pub winner: Option<Pubkey>,

    /// If None - SOL, else token mint
    pub asset: Option<Pubkey>,

    /// Place that winner gets
    pub place: u64,

    /// Bump for Vault PDA seeds
    pub bump: u8,
}