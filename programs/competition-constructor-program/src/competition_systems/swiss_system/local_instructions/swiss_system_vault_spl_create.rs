use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        Mint,
        TokenAccount,
        TokenInterface,
        TransferChecked,
        transfer_checked,
    },
};

use crate::state::*;
use crate::seeds::*;
use crate::error::*;
use crate::competition_systems::swiss_system::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwissSystemVaultSplCreateArgs {
    pub competition_index: u64,

    pub asset: Pubkey,

    pub prize: u64,
}

#[derive(Accounts)]
#[instruction(args: SwissSystemVaultSplCreateArgs)]
pub struct SwissSystemVaultSplCreate<'info> {
    #[account(mut)]
    pub organizer: Signer<'info>,

    #[account(
        init,
        payer = organizer,
        space = 8 + local_state::Vault::INIT_SPACE,
        seeds = [
            SEED_PREFIX,
            swiss_system.creator_key.as_ref(),
            SEED_VAULT,
            &swiss_system.vault_index.to_le_bytes(),
        ],
        bump,
    )]
    pub vault: Account<'info, local_state::Vault>,

    #[account(
        mut,
        seeds = [
            SEED_PREFIX,
            constructor.creator_key.as_ref(),
            SEED_COMPETITION,
            &args.competition_index.to_le_bytes(),
        ],
        bump = swiss_system.bump,
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

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = organizer,
        associated_token::token_program = token_program,
    )]
    pub organizer_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed, // UNSAFE
        payer = organizer,
        associated_token::mint = mint,
        associated_token::authority = vault,
        associated_token::token_program = token_program,
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

impl<'info> SwissSystemVaultSplCreate<'info> {
    pub fn swiss_system_vault_spl_create(
        ctx: Context<Self>,
        args: SwissSystemVaultSplCreateArgs,
    ) -> Result<()> {
        let winner = None;
        let asset  = None;

        let place = ctx.accounts.swiss_system.vault_index;
        let bump = ctx.bumps.vault;

        ctx.accounts.vault.set_inner( local_state::Vault {
            winner,
            asset,
            place,
            bump,
        });

        ctx.accounts.swiss_system.vault_index =
            ctx.accounts.swiss_system.vault_index.checked_add(1).ok_or(
                CustomError::Overflow,
        )?;

        let transfer = TransferChecked {
            from:      ctx.accounts.organizer_ata.to_account_info(),
            mint:      ctx.accounts.mint.to_account_info(),
            to:        ctx.accounts.vault_ata.to_account_info(),
            authority: ctx.accounts.organizer.to_account_info(),
        };

        let context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer,
        );

        transfer_checked(
            context,
            args.prize,
            ctx.accounts.mint.decimals,
        )?;

        Ok(())
    }
}