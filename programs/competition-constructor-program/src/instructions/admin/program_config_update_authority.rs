use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ProgramConfigUpdateAuthorityArgs {
    pub authority: Pubkey,
}

#[derive(Accounts)]
#[instruction(args: ProgramConfigUpdateAuthorityArgs)]
pub struct ProgramConfigUpdateAuthority<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        address = authority.key()
            @ CustomError::Unauthorized,
        seeds = [
            SEED_PREFIX,
            SEED_PROGRAM_CONFIG,
        ],
        bump  = program_config.bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,
}

impl<'info> ProgramConfigUpdateAuthority<'info> {
    pub fn program_config_update_authority(
        ctx: Context<Self>,
        args: ProgramConfigUpdateAuthorityArgs,
    ) -> Result<()> {
        let program_config = &mut ctx.accounts.program_config;
        program_config.authority = args.authority;

        Ok(())
    }
}