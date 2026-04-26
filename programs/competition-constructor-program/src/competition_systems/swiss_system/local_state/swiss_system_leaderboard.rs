use anchor_lang::prelude::*;

pub const LEADER_BOARD_LIMIT: usize = 30; 

#[account]
#[derive(InitSpace)]
pub struct LeaderBoard {
    /// List of participants
    #[max_len(LEADER_BOARD_LIMIT)]
    pub list: Vec<ParticipantData>,

    /// Bump for Leaderboard account PDA seeds
    pub bump: u8,
}

impl LeaderBoard {
    pub fn sort_by_points(&mut self, participant: ParticipantData) -> Result<()> {
        if self.list.len() < LEADER_BOARD_LIMIT {
            self.list.push(participant);
        } else {
            if let Some(last) = self.list.last() {
                if participant.points > last.points {
                    *self.list.last_mut().unwrap() = participant;
                }
                return Ok(());
            }
        }

        self.list.sort_unstable_by(|a, b| b.points.cmp(&a.points));

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