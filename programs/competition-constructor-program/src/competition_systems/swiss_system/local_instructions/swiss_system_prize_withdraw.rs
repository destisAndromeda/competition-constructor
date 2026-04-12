use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;
use crate::competition_systems::swiss_system::local_state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwissSystemPrizeWithdrawArgs {
    pub competition_index: u64,

}

#[derive(Accounts)]
pub struct SwissSystemPrizeWithdraw<'info> {
    #[account(mut)]
    pub organizer: Signer<'info>,

    #[account(
        seeds = [
            SEED_PREFIX,
            constructor.creator_key.key().as_ref(),
            SEED_COMPETITION,
            organizer.key().as_ref(),
            SEED_SWISS_SYSTEM,
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
            program_config.creator_key.key().as_ref(),
        ],
        bump  = constructor.bump,
    )]
    pub constructor: Account<'info, Constructor>,

    #[account(
        seeds = [
            SEED_PREFIX,
            SEED_PROGRAM_CONFIG,
        ],
        bump  = program_config.bump
    )]
    pub program_config: Account<'info, ProgramConfig>,
}