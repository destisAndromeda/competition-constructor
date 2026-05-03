use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::*;
use crate::seeds::*;
use crate::competition_systems::swiss_system::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwissSystemUpdateArgs {
    pub competition_index: u64,

    pub account: Pubkey,
}

#[derive(Accounts)]
#[instruction(args: SwissSystemUpdateArgs)]
pub struct SwissSystemUpdate<'info> {
    #[account(mut)]
    pub organizer: Signer<'info>,

    #[account(
        mut,
        has_one = organizer
            @ CustomError::Unauthorized,
        seeds = [
            SEED_PREFIX,
            constructor.creator_key.key().as_ref(),
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
    program_config: Account<'info, ProgramConfig>,
}

impl<'info> SwissSystemUpdate<'info> {
    pub fn swiss_system_authority_update(
        ctx: Context<Self>,
        args: SwissSystemUpdateArgs,
    ) -> Result<()> {
        let swiss_system = &mut ctx.accounts.swiss_system;

        swiss_system.authority = args.account; 

        swiss_system.invariant()?;

        Ok(())
    }

    pub fn swiss_system_creator_key_update(
        ctx: Context<Self>,
        args: SwissSystemUpdateArgs,
    ) -> Result<()> {
        let swiss_system = &mut ctx.accounts.swiss_system;

        swiss_system.creator_key = args.account;

        swiss_system.invariant()?;

        Ok(())
    }
} 