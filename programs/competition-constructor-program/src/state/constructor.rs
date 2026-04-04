use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Constructor {
    /// Key for competitions PDA seeds
    pub creator_key: Pubkey,

    /// Last competition index
    pub competition_index: u64,

    /// Fee for competitions create in lamports
    pub transaction_fee: u64,

    /// Bump for Constructor account PDA seeds
    pub bump: u8,
}