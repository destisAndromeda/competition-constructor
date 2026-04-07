use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SwissSystem {
    /// Owner of event
    pub organizer: Pubkey,
    /// Current stage of the competition
    pub stage: Stage,
    /// Current status of competition
    pub activated: bool,
    /// The last index of created vault
    pub valut_index: u64,
    /// The last index of created participant
    pub participant_index: u64,
    /// Bump for SwissSystem account PDA seeds
    pub bump: u8,
}

#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    InitSpace,
)]
pub enum Stage {
    /// The period during which participants can register their accounts
    registration_period: { timestamp: u64 },
    /// The period during which the competition takes place
    competition_period:  { timestamp: u64 },
    /// The period during which the winner can claim the prize
    withdraw_period:     { timestamp: u64 },
}
