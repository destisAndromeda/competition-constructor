use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Participant {
    /// Address of participant wallet
    pub participant: Pubkey,

    /// Total count earned points
    pub points: u64,

    /// Bump for Participant account PDA seeds
    pub bump: u8,
}
