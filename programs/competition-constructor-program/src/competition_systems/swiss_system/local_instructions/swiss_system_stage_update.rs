use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::competition_systems::swiss_system::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwissSystemStageUpdateArgs {
    pub competition_index: u64,
}

#[derive(Accounts)]
#[instruction(args: SwissSystemStageUpdateArgs)]
pub struct SwissSystemStageUpdate<'info> {
    #[account(
        mut,
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
    pub fn swiss_system_stage_update(
        ctx: Context<Self>,
        _args: SwissSystemStageUpdateArgs,
    ) -> Result<()> {
        let stage_info = ctx.accounts.swiss_system.stage_info.clone();
        let swiss_system = &mut ctx.accounts.swiss_system;
        let current_time = Clock::get()?.unix_timestamp;

        if current_time >= stage_info.withdraw_period {
            swiss_system.stage = Some(
                local_state::Stage::RegistrationPeriod {
                    timestamp: Clock::get()?.unix_timestamp,
            });
        } else if current_time >= stage_info.competition_period {
            swiss_system.stage = Some(
                local_state::Stage::CompetitionPeriod {
                    timestamp: Clock::get()?.unix_timestamp,
            });
        } else if current_time >= stage_info.registration_period {
            swiss_system.stage = Some(
                local_state::Stage::WithdrawPeriod {
                    timestamp: Clock::get()?.unix_timestamp,
            });
        }

        Ok(())
    }
}