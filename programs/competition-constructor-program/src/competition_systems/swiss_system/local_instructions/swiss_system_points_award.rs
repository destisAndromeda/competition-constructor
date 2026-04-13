use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;
use crate::competition_systems::swiss_system::local_state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwissSystemPointsAwardArgs {
    pub competition_index: u64,

    pub organizer: Pubkey,

    pub participant_index: u64,

    pub points: u64,
}

#[derive(Accounts)]
#[instruction(args: SwissSystemPointsAwardArgs)]
pub struct SwissSystemPointsAward<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority
            @ CustomError::Unauthorized,
        seeds = [
            SEED_PREFIX,
            swiss_system.creator_key.as_ref(),
            SEED_PARTICIPANT,
            &args.participant_index.to_le_bytes(),
        ],
        bump  = participant.bump,
    )]
    pub participant: Account<'info, Participant>,

    #[account(
        mut,
        has_one = authority
            @ CustomError::Unauthorized,
        seeds = [
            SEED_PREFIX,
            swiss_system.creator_key.as_ref(),
            SEED_LEADER_BOARD,
            args.organizer.as_ref(),
        ],
        bump  = leaderboard.bump,
    )]
    pub leaderboard: Account<'info, LeaderBoard>,

    #[account(
        seeds = [
            SEED_PREFIX,
            constructor.creator_key.as_ref(),
            SEED_COMPETITION,
            &args.competition_index.to_le_bytes(),
        ],
        bump  = swiss_system.bump,
    )]
    pub swiss_system: Account<'info, SwissSystem>,

    #[account(
        seeds = [
            SEED_PREFIX,
            program_config.key().as_ref(),
            SEED_CONSTRUCTOR,
            program_config.creator_key.as_ref(),
        ],
        bump  = constructor.bump,
    )]
    pub constructor: Account<'info, Constructor>,

    #[account(
        seeds = [
            SEED_PREFIX,
            SEED_PROGRAM_CONFIG,
        ],
        bump  = program_config.bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,
}

impl<'info> SwissSystemPointsAward<'info> {
    pub fn swiss_system_points_award(
        ctx: Context<Self>,
        args: SwissSystemPointsAwardArgs
    ) -> Result<()> {
        ctx.accounts.participant.points += args.points;

        ctx.accounts.leaderboard.sort_by_points(ParticipantData {
            address: Some(ctx.accounts.participant.key()),
            points: ctx.accounts.participant.points,
        });

        Ok(())
    }
}