use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ConstructorUpdateTransactionFeeArgs {
    pub transaction_fee: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ConstructorUpdateArgs {
    pub account: Pubkey,
}

#[derive(Accounts)]
pub struct ConstructorUpdate<'info> {
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
    pub program_config: Account<'info, ProgramConfig>,
}

impl<'info> ConstructorUpdate<'info> {
    fn validate(&self, args: &ConstructorUpdateArgs) -> Result<()> {
        let Self {
            creator_key,
            constructor,
            ..
        } = self;
        
        require_keys_neq!(
            creator_key.key(),
            args.account,
            CustomError::InvalidAccount,
        );

        require_keys_neq!(
            constructor.authority,
            args.account,
            CustomError::InvalidAccount,
        );

        require_keys_neq!(
            constructor.creator_key,
            args.account,
            CustomError::InvalidAccount
        );

        Ok(())
    }
    
    #[access_control(ctx.accounts.validate(&args))]
    pub fn constructor_update_authority(
        ctx: Context<Self>,
        args: ConstructorUpdateArgs,
    ) -> Result<()> {
        let constructor = &mut ctx.accounts.constructor;
        
        constructor.authority = args.account;

        constructor.invariant()?;

        Ok(())
    }

    #[access_control(ctx.accounts.validate(&args))]
    pub fn constructor_update_creator_key(
        ctx: Context<Self>,
        args: ConstructorUpdateArgs,
    ) -> Result<()> {
        let constructor = &mut ctx.accounts.constructor;

        constructor.creator_key = args.account;

        constructor.invariant()?;

        Ok(())
    }

    pub fn constructor_update_transaction_fee(
        ctx: Context<Self>,
        args: ConstructorUpdateTransactionFeeArgs,
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