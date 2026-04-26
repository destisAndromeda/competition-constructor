use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ConstructorAuthorityUpdateArgs {
    authority: Pubkey,
}

#[derive(Accounts)]
pub struct ConstructorAuthorityUpdate<'info> {
    #[account(mut)]
    pub creator_key: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_PREFIX,
            program_config.key().as_ref(),
            SEED_CONSTRUCTOR,
            creator_key.key().as_ref(),
        ],
        bump  = constructor.bump,
    )]
    pub constructor: Account<'info, Constructor>,

    #[account(
        has_one = creator_key
            @ CustomError::Unauthorized,
        seeds = [
            SEED_PREFIX,
            SEED_PROGRAM_CONFIG,
        ],
        bump  = program_config.bump,
    )]
    program_config: Account<'info, ProgramConfig>,
}

impl<'info> ConstructorAuthorityUpdate<'info> {
    fn validate(&self, args: &ConstructorAuthorityUpdateArgs) -> Result<()> {
        let Self {
            creator_key,
            ..
        } = self;

        require_keys_neq!(
            creator_key.key(),
            args.authority,
            CustomError::InvalidAccount,
        );

        Ok(())
    }
    
    #[access_control(ctx.accounts.validate(&args))]
    pub fn constructor_authority_update(
        ctx: Context<Self>,
        args: ConstructorAuthorityUpdateArgs,
    ) -> Result<()> {
        let constructor = &mut ctx.accounts.constructor;

        constructor.authority = args.authority;

        constructor.invariant()?;

        Ok(())
    }
}