use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;
use crate::competition_systems::swiss_system::local_state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwissSystemLeaderBoardCreateArgs {
    pub competition_index: u64,
}

#[derive(Accounts)]
#[instruction(args: SwissSystemLeaderBoardCreateArgs)]
pub struct SwissSystemLeaderBoardCreate<'info> {
    #[account(mut)]
    pub organizer: Signer<'info>,

    #[account(
        init,
        payer = organizer,
        space = 8 + LeaderBoard::INIT_SPACE,
        seeds = [
            SEED_PREFIX,
            swiss_system.creator_key.as_ref(),
            SEED_LEADER_BOARD,
            organizer.key().as_ref(),
        ],
        bump,
    )]
    pub leaderboard: Account<'info, LeaderBoard>,

    #[account(
        has_one = organizer
            @ CustomError::Unauthorized,
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

    pub system_program: Program<'info, System>,
}

impl<'info> SwissSystemLeaderBoardCreate<'info> {
    pub fn swiss_system_leaderboard_create(
        ctx: Context<Self>,
        args: SwissSystemLeaderBoardCreateArgs,
    ) -> Result<()> {
        let bump = ctx.bumps.leaderboard;

        ctx.accounts.leaderboard.set_inner(LeaderBoard {
            list: Vec::new(),
            bump,
        });

        Ok(())
    }
}