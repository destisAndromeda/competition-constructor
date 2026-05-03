use anchor_lang::prelude::*;

#[cfg(not(feature = "testing"))]
pub const LEADER_BOARD_LIMIT: usize = 30; 

#[cfg(feature = "testing")]
pub const LEADER_BOARD_LIMIT: usize = 3;

#[account]
#[derive(InitSpace)]
pub struct LeaderBoard {
    /// List of participants
    #[max_len(LEADER_BOARD_LIMIT)]
    pub list: Vec<Option<ParticipantData>>,

    /// Bump for Leaderboard account PDA seeds
    pub bump: u8,
}

impl LeaderBoard {
    fn sort_list(&mut self) {
        self.list.sort_unstable_by(|a, b| {
            let a_points = a.as_ref().map(|p| p.points).unwrap_or(0);
            let b_points = b.as_ref().map(|p| p.points).unwrap_or(0);
            b_points.cmp(&a_points)
        });
    }

    pub fn sort_by_points(&mut self, participant: ParticipantData) -> Result<()> {
        if let Some(existing) = self
            .list
            .iter_mut()
            .filter_map(|p| p.as_mut())
            .find(|p| p.address == participant.address)
        {
            existing.points = participant.points;
        } else if self.list.len() < LEADER_BOARD_LIMIT {
            self.list.push(Some(participant));
        } else if let Some(last) = self.list.last().and_then(|p| p.as_ref()) {
            if participant.points <= last.points {
                return Ok(());
            }

            *self.list.last_mut().unwrap() = Some(participant);
        }

        self.sort_list();

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
    pub address: Pubkey,
    pub points:  u64,
}