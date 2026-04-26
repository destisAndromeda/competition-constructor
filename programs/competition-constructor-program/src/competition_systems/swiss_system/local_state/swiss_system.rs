use anchor_lang::prelude::*;

// use state::error::*;

#[account]
#[derive(InitSpace)]
pub struct SwissSystem {
    /// Owner of event
    pub organizer: Pubkey,

    /// Creator key for accounts PDA
    pub creator_key: Pubkey,    

    /// Current stage of the competition
    pub stage: Option<Stage>,
    
    /// Metadata for stage determine
    pub stage_info: StageInfo,
    
    /// The last index of created vault
    pub vault_index: u64,

    /// Bump for SwissSystem account PDA seeds
    pub bump: u8,
}

// impl SwissSystem {
//     pub fn invariant(&self) -> Reuslt<()> {
//         require_keys_neq!(
//             self.organizer,
//             self.creator_key,
//             CustomError::SameAccoutns,
//         );

//         Ok(())
//     }
// }

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
    RegistrationPeriod { timestamp: i64 },
    
    /// The period during which the competition takes place
    CompetitionPeriod  { timestamp: i64 },
    
    /// The period during which the winner can claim the prize
    WithdrawPeriod     { timestamp: i64 },
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
    pub registration_period: i64,
    pub competition_period: i64,
    pub withdraw_period: i64,
}