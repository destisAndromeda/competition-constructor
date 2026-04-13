use anchor_lang::prelude::*;

pub const LEADER_BOARD_LIMIT: usize = 30; 

#[account]
#[derive(InitSpace)]
pub struct LeaderBoard {
    /// Authority that can reward points
    pub authority: Pubkey,

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

// impl LeaderBoard {
//     pub fn insert_and_sort(&mut self, participant: ParticipantData) -> Result<()> {
//         // Если список не полон, просто добавляем
//         if self.list.len() < LEADER_BOARD_LIMIT {
//             self.list.push(participant);
//         } else {
//             // Если полон, проверяем, лучше ли новый участник худшего (последнего)
//             if let Some(last) = self.list.last() {
//                 if participant.points > last.points {
//                     // Заменяем худшего
//                     *self.list.last_mut().unwrap() = participant;
//                 } else {
//                     // Не добавляем, если хуже
//                     return Ok(());
//                 }
//             }
//         }

//         // Сортируем по убыванию очков (выше — больше points)
//         self.list.sort_unstable_by(|a, b| b.points.cmp(&a.points));

//         Ok(())
//     }
// }

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