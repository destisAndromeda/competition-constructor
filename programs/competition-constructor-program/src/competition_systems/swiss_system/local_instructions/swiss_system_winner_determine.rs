use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;
use crate::competition_systems::swiss_system::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwissSystemWinnerDetermineArgs {
    pub competition_index: u64,
    
    pub organizer: Pubkey,

    pub vault_index: u64,
}

#[derive(Accounts)]
#[instruction(args: SwissSystemWinnerDetermineArgs)]
pub struct SwissSystemWinnerDetermine<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_PREFIX,
            swiss_system.creator_key.as_ref(),
            SEED_VAULT,
            &args.vault_index.to_le_bytes(),
        ],
        bump  = vault.bump,
    )]
    pub vault: Account<'info, local_state::Vault>,

    #[account(
        seeds = [
            SEED_PREFIX,
            swiss_system.creator_key.as_ref(),
            SEED_LEADER_BOARD,
            args.organizer.key().as_ref(),
        ],
        bump  = leaderboard.bump,
    )]
    pub leaderboard: Account<'info, local_state::LeaderBoard>,

    #[account(
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

impl<'info> SwissSystemWinnerDetermine<'info> {
    fn validate(&self) -> Result<()> {
        let Self {
            swiss_system,
            ..
        } = self;

        #[cfg(not(feature = "testing"))]
        {
            let current = Clock::get()?.unix_timestamp;

            if let Some(local_state::Stage::WithdrawPeriod { timestamp }) = self.swiss_system.stage {
                require!(
                    current >= timestamp,
                    CustomError::InvalidStage,
                );
            } else {
                return err!(CustomError::InvalidStage);
            };
        }

        Ok(())
    }

    #[access_control(ctx.accounts.validate())]
    pub fn swiss_system_winner_determine(
        ctx: Context<Self>,
        args: SwissSystemWinnerDetermineArgs,
    ) -> Result <()> {
        let vault = &mut ctx.accounts.vault;
        let leaderboard = &mut ctx.accounts.leaderboard;

        let place = vault.place as usize;

        if let Some(winner) = leaderboard.list[place] {
            vault.winner = Some(winner.address);
        } else {
            return err!(CustomError::WinnerIsNotDetermine);
        };

        Ok(())
    }
}