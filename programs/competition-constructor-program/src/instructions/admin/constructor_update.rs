use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ConstructorUpdateAuthorityArgs {
    pub authority: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ConstructorUpdateCreatorKeyArgs {
    pub creator_key: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ConstructorUpdateTransactionFeeArgs {
    pub transaction_fee: u64,
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
    pub fn constructor_update_authority(
        ctx: Context<Self>,
        args: ConstructorUpdateAuthorityArgs,
    ) -> Result<()> {
        let constructor = &mut ctx.accounts.constructor;

        require_neq!(
            constructor.authority,
            args.authority,
            CustomError::DeprecatedAddress,
        );

        constructor.authority = args.authority;

        Ok(())
    }

    pub fn constructor_update_creator_key(
        ctx: Context<Self>,
        args: ConstructorUpdateCreatorKeyArgs,
    ) -> Result<()> {
        let constructor = &mut ctx.accounts.constructor;

        require_neq!(
            constructor.creator_key,
            args.creator_key,
            CustomError::DeprecatedAddress,
        );

        constructor.creator_key = args.creator_key;
        constructor.competition_index = 0;

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

        Ok(())
    }
}