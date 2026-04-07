use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SwissSystem {
    /// Owner of event
    pub organizer: Pubkey,

    /// Current stage of the competition
    pub stage: Option<Stage>,
    
    /// Metadata for stage determine
    pub stage_info: StageInfo,

    /// Current status of competition
    pub activated: bool,
    
    /// The last index of created vault
    pub vault_index: u64,
    
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
    RegistrationPeriod { timestamp: u64 },
    
    /// The period during which the competition takes place
    CompetitionPeriod  { timestamp: u64 },
    
    /// The period during which the winner can claim the prize
    WithdrawPeriod     { timestamp: u64 },
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
pub struct StageInfo {
    pub registration_period: u64,
    pub competition_period: u64,
    pub withdraw_period: u64,
}