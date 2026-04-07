use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;
use crate::competition_systems::swiss_system::local_state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwissSystemCreateArgs {
    pub stage_info: StageInfo,
}

#[derive(Accounts)]
pub struct SwissSystemCreate<'info> {
    #[account(mut)]
    pub organizer: Signer<'info>,

    #[account(
        init,
        payer = organizer,
        space = 8 + SwissSystem::INIT_SPACE,
        seeds = [
            SEED_PREFIX,
            constructor.key().as_ref(),
            SEED_CONSTRUCTOR,
            organizer.key().as_ref(),
            SEED_SWISS_SYSTEM,
            &constructor.competition_index.to_le_bytes(),
        ],
        bump,
    )]
    pub swiss_system: Account<'info, SwissSystem>,

    #[account(
        mut,
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
        bump  = program_config.bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,

    pub system_program: Program<'info, System>,
}

impl<'info> SwissSystemCreate<'info> {
    pub fn swiss_system_create(ctx: Context<Self>, args: SwissSystemCreateArgs) -> Result<()> {
        let organizer = ctx.accounts.organizer.key();
        let stage = None;
        
        let stage_info = args.stage_info;
        let activated = false;

        let vault_index = 0;
        let participant_index = 0;

        let bump = ctx.bumps.swiss_system;

        ctx.accounts.swiss_system.set_inner( SwissSystem {
            organizer,
            stage,
            stage_info,
            activated,
            vault_index,
            participant_index,
            bump,
        });

        Ok(())
    }
}