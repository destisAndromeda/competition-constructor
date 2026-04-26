use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Participant {
    /// Address of participant wallet
    pub participant: Pubkey,

    /// Total count earned points
    pub points: u64,

    /// Current status of participant
    pub status: ParticipantStatus,

    /// Bump for Participant account PDA seeds
    pub bump: u8,
}

#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    InitSpace,
    Copy,
    Clone,
    PartialEq,
    Eq,
)]
pub enum ParticipantStatus {
    Active          { timestamp: i64 },
    Disqualified    { timestamp: i64 },
    Winner          { timestamp: i64 },
}