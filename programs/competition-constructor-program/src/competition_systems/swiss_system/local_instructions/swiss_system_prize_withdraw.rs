use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;
use crate::competition_systems::swiss_system::local_state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwissSystemPrizeWithdrawArgs {
    pub competition_index: u64,
    pub vault_index: u64,
}

#[derive(Accounts)]
#[instruction(args: SwissSystemPrizeWithdrawArgs)]
pub struct SwissSystemPrizeWithdraw<'info> {
    #[account(
        mut,
        seeds = [
            SEED_PREFIX,
            swiss_system.creator_key.key().as_ref(),
            SEED_VAULT,
            &args.vault_index.to_le_bytes(),
        ],
        bump  = swiss_system.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        seeds = [
            SEED_PREFIX,
            constructor.creator_key.key().as_ref(),
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

    // For transfer context
    pub system_program: Program<'info, System>,

    /// CHECK: Account of winner ; Need for transfer
    #[account(mut)]
    pub winner: UncheckedAccount<'info>,
}

impl<'info> SwissSystemPrizeWithdraw<'info> {
    pub fn validate(&self) -> Result<()> {
        let Self {
            winner,
            vault,
            ..
        } = self;

        if let Some(winner_address) = vault.winner {
            require_keys_eq!(
                winner_address,
                winner.key(),
                CustomError::Unauthorized,
            );
        } else {
            return err!(CustomError::WinnerIsNotDetermine);
        };

        Ok(())
    }
    
    #[access_control(ctx.accounts.validate())]
    pub fn swiss_system_prize_withdraw(
        ctx: Context<Self>,
        args: SwissSystemPrizeWithdrawArgs,
    ) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let lamports = ctx.accounts.vault.to_account_info().lamports();

        if let Some(_) = vault.winner {
            let transfer = system_program::Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to:   ctx.accounts.winner.to_account_info(),
            };

            let creator_key = ctx.accounts.swiss_system.creator_key.as_ref();

            let signer_seeds = &[
                SEED_PREFIX,
                creator_key,
                SEED_VAULT,
                &[args.vault_index.try_into().unwrap()],
                &[vault.bump],
            ];

            let signer_seeds_slice = &[&signer_seeds[..]];

            let context = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                transfer,
                signer_seeds_slice,
            );

            system_program::transfer(
                context,
                lamports,
            )?;

            Ok(())
        } else {
            return err!(CustomError::WinnerIsNotDetermine);
        }
    }
}