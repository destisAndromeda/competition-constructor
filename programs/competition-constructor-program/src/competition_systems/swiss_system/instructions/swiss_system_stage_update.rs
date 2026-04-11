use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;
use crate::competition_systems::swiss_system::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwissSystemStageUpdateArgs {
    pub organizer: Pubkey,
}

#[derive(Accounts)]
#[instruction(args: SwissSystemStageUpdateArgs)]
pub struct SwissSystemStageUpdate<'info> {
    #[account(
        mut,
        seeds = [
            SEED_PREFIX,
            constructor.key().as_ref(),
            SEED_CONSTRUCTOR,
            args.organizer.key().as_ref(),
            SEED_SWISS_SYSTEM,
            &constructor.competition_index.to_le_bytes(),
        ],
        bump  = swiss_system.bump,
    )]
    pub swiss_system: Account<'info, local_state::SwissSystem>,

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
        bump  = program_config.bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,
}

impl<'info> SwissSystemStageUpdate<'info> {
    fn validate(&self) -> Result<()> {
        let Self {
            swiss_system,
            ..
        } = self;

        require!(
            swiss_system.activated,
            CustomError::Inactive,
        );

        Ok(())
    }

    #[access_control(ctx.accounts.validate())]
    pub fn swiss_system_stage_update(ctx: Context<Self>, _args: SwissSystemStageUpdateArgs) -> Result<()> {
        let stage_info = ctx.accounts.swiss_system.stage_info.clone();
        let swiss_system = &mut ctx.accounts.swiss_system;
        let current_time = Clock::get()?.unix_timestamp;

        if current_time >= stage_info.withdraw_period {
            swiss_system.stage = Some(local_state::Stage::RegistrationPeriod {
                timestamp: Clock::get()?.unix_timestamp,
            });
        } else if current_time >= stage_info.competition_period {
            swiss_system.stage = Some(local_state::Stage::CompetitionPeriod {
                timestamp: Clock::get()?.unix_timestamp,
            });
        } else if current_time >= stage_info.registration_period {
            swiss_system.stage = Some(local_state::Stage::RegistrationPeriod {
                timestamp: Clock::get()?.unix_timestamp,
            });
        }

        Ok(())
    }
}