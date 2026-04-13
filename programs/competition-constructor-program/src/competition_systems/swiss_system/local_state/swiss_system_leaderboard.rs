use anchor_lang::prelude::*;

const LEADER_BOARD_LIMIT: usize = 30; 

#[account]
#[derive(InitSpace)]
pub struct LeaderBoard {
    /// Authority that can change points count in ParticiapntData
    pub authority: Pubkey,

    /// List of participants
    #[max_len(LEADER_BOARD_LIMIT)]
    pub participants: Vec<ParticipantData>,

    /// Bump for Leaderboard account PDA seeds
    pub bump: u8,
}

impl LeaderBoard {
    pub fn sort_by_points(&mut self) -> Result<()> {
        let participants = &mut self.participants[..LEADER_BOARD_LIMIT];

        participants.sort_unstable_by(|a, b| b.points.cmp(&a.points));

        Ok(())
    }
}

#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    InitSpace,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct ParticipantData {
    pub address: Option<Pubkey>,
    pub points:  u64,
}