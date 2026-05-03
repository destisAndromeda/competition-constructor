use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ConstructorTransactionFeeUpdateArgs {
    pub transaction_fee: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ConstructorUpdateArgs {
    pub creator_key: Pubkey,
}

#[derive(Accounts)]
pub struct ConstructorUpdate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority
            @ CustomError::Unauthorized,
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
        constraint = authority.key() != program_config.creator_key
            @ CustomError::Unauthorized,
        seeds = [
            SEED_PREFIX,
            SEED_PROGRAM_CONFIG,
        ],
        bump  = program_config.bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,
}

impl<'info> ConstructorUpdate<'info> {
    fn validate(&self, args: &ConstructorUpdateArgs) -> Result<()> {
        let Self {
            authority,
            constructor,
            program_config,
            ..
        } = self;

        require_keys_neq!(
            authority.key(),
            args.creator_key,
            CustomError::InvalidAccount,
        );

        require_keys_neq!(
            program_config.creator_key,
            args.creator_key,
            CustomError::InvalidAccount,
        );

        Ok(())
    }

    #[access_control(ctx.accounts.validate(&args))]
    pub fn constructor_creator_key_update(
        ctx: Context<Self>,
        args: ConstructorUpdateArgs,
    ) -> Result<()> {
        let constructor = &mut ctx.accounts.constructor;

        constructor.creator_key = args.creator_key;

        constructor.invariant()?;

        Ok(())
    }

    pub fn constructor_transaction_fee_update(
        ctx: Context<Self>,
        args: ConstructorTransactionFeeUpdateArgs,
    ) -> Result<()> {
        let constructor = &mut ctx.accounts.constructor;

        require_neq!(
            constructor.transaction_fee,
            args.transaction_fee,
            CustomError::InvalidAmount,
        );

        constructor.transaction_fee = args.transaction_fee;

        constructor.invariant()?;

        Ok(())
    }
}