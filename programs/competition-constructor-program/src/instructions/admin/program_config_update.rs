use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ProgramConfigUpdateAuthorityArgs {
    pub authority: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ProgramConfigUpdateCreatorKeyArgs {
    pub creator_key: Pubkey,
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
    pub fn program_config_update_authority(
        ctx: Context<Self>,
        args: ProgramConfigUpdateAuthorityArgs,
    ) -> Result<()> {
        let program_config = &mut ctx.accounts.program_config;
        
        require_neq!(
            program_config.authority,
            args.authority,
            CustomError::Deprecated,
        );
        
        program_config.authority = args.authority;

        Ok(())
    }

    pub fn program_config_update_creator_key(
        ctx: Context<Self>,
        args: ProgramConfigUpdateCreatorKeyArgs,
    ) -> Result<()> {
        let program_config = &mut ctx.accounts.program_config;

        require_neq!(
            program_config.creator_key,
            args.creator_key,
            CustomError::Deprecated,
        );

        program_config.creator_key = args.creator_key;

        Ok(())
    }
}