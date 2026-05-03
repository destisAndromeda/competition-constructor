use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ProgramConfigUpdateArgs {
    pub account: Pubkey,
}

#[derive(Accounts)]
pub struct ProgramConfigUpdate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority
            @ CustomError::Unauthorized,
        seeds = [
            SEED_PREFIX,
            SEED_PROGRAM_CONFIG,
        ],
        bump  = program_config.bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,
}

impl<'info> ProgramConfigUpdate<'info> {
    fn validate(&self, args: &ProgramConfigUpdateArgs) -> Result<()> {
        let Self {
            authority,
            program_config,
        } = self;
        
        require_keys_neq!(
            authority.key(),
            args.account,
            CustomError::DeprecatedAddress,
        );

        require_keys_neq!(
            program_config.creator_key,
            args.account,
            CustomError::DeprecatedAddress,
        );

        require_keys_neq!(
            program_config.treasury,
            args.account,
            CustomError::DeprecatedAddress,
        );

        Ok(())
    }

    #[access_control(ctx.accounts.validate(&args))]
    pub fn program_config_authority_update(
        ctx: Context<Self>,
        args: ProgramConfigUpdateArgs,
    ) -> Result<()> {
        let program_config = &mut ctx.accounts.program_config;

        program_config.authority = args.account;
        
        program_config.invariant()?;

        Ok(())
    }

    #[access_control(ctx.accounts.validate(&args))]
    pub fn program_config_creator_key_update(
        ctx: Context<Self>,
        args: ProgramConfigUpdateArgs,
    ) -> Result<()> {
        let program_config = &mut ctx.accounts.program_config;

        program_config.creator_key = args.account;

        program_config.invariant()?;

        Ok(())
    }

    #[access_control(ctx.accounts.validate(&args))]
    pub fn program_config_treasury_update(
        ctx: Context<Self>,
        args: ProgramConfigUpdateArgs,
    ) -> Result<()> {
        let program_config = &mut ctx.accounts.program_config;

        program_config.treasury = args.account;

        program_config.invariant()?;

        Ok(())
    }
}