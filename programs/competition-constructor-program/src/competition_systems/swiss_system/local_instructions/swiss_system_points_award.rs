use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;
use crate::competition_systems::swiss_system::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwissSystemPointsAwardArgs {
    pub competition_index: u64,

    pub organizer: Pubkey,

    pub participant: Pubkey,

    pub points: u64,
}

#[derive(Accounts)]
#[instruction(args: SwissSystemPointsAwardArgs)]
pub struct SwissSystemPointsAward<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint =
            args.participant == participant.participant
            @ CustomError::InvalidAccount,
        seeds = [
            SEED_PREFIX,
            swiss_system.creator_key.as_ref(),
            SEED_PARTICIPANT,
            args.participant.as_ref(),
        ],
        bump  = participant.bump,
    )]
    pub participant: Account<'info, local_state::Participant>,

    #[account(
        mut,
        seeds = [
            SEED_PREFIX,
            swiss_system.creator_key.as_ref(),
            SEED_LEADER_BOARD,
            args.organizer.as_ref(),
        ],
        bump  = leaderboard.bump,
    )]
    pub leaderboard: Account<'info, local_state::LeaderBoard>,

    #[account(
        has_one = authority
            @ CustomError::Unauthorized,
        seeds = [
            SEED_PREFIX,
            constructor.creator_key.as_ref(),
            SEED_COMPETITION,
            &args.competition_index.to_le_bytes(),
        ],
        bump  = swiss_system.bump,
    )]
    pub swiss_system: Account<'info, local_state::SwissSystem>,

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
        require!(
            args.points != 0,
            CustomError::IncorrectValue,
        );

        ctx.accounts.participant.points =
            ctx.accounts.participant.points.checked_add(args.points).ok_or(
                CustomError::Overflow,
            )?;

        ctx.accounts.leaderboard.sort_by_points( local_state::ParticipantData {
            address: args.participant,
            points:  ctx.accounts.participant.points,
        });

        Ok(())
    }
}