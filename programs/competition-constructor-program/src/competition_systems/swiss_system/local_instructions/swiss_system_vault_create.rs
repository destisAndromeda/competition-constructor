use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;
use crate::competition_systems::swiss_system::local_state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwissSystemVaultCreateArgs {
    pub competition_index: u64,

    pub prize: u64,
}

#[derive(Accounts)]
#[instruction(args: SwissSystemVaultCreateArgs)]
pub struct SwissSystemVaultCreate<'info> {
    #[account(mut)]
    pub organizer: Signer<'info>,

    #[account(
        init,
        payer = organizer,
        space = 8 + Vault::INIT_SPACE,
        seeds = [
            SEED_PREFIX,
            swiss_system.creator_key.key().as_ref(),
            SEED_VAULT,
            &swiss_system.vault_index.to_le_bytes(),
        ],
        bump,
    )]
    pub vault: Account<'info, Vault>,

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
        bump  = program_config.bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,

    pub system_program: Program<'info, System>,
}

impl<'info> SwissSystemVaultCreate<'info> {
    pub fn swiss_system_vault_create(
        ctx: Context<Self>,
        args: SwissSystemVaultCreateArgs,
    ) -> Result<()> {
        // Init Vault account
        let organizer = ctx.accounts.organizer.key();
        let winner = None;
        
        let place = ctx.accounts.swiss_system.vault_index;
        let bump = ctx.bumps.vault;

        ctx.accounts.vault.set_inner(Vault {
            organizer,
            winner,
            place,
            bump,
        });

        // Deposit SOL in lamports
        let transfer = system_program::Transfer {
            from: ctx.accounts.organizer.to_account_info(),
            to:   ctx.accounts.vault.to_account_info(),
        };

        let context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer,
        );

        system_program::transfer(
            context,
            args.prize,
        )?;

        ctx.accounts.swiss_system.vault_index =
        ctx.accounts.swiss_system.vault_index.checked_add(1).ok_or(
            CustomError::Overflow,
        )?;

        Ok(())
    }
}