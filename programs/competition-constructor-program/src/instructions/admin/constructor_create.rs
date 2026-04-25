use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ConstructorCreateArgs {
    // authority that can update pda
    pub authority: Pubkey,

    // for competitions pda seeds
    pub creator_key: Pubkey,

    // in lamports
    pub transaction_fee: u64,
}

#[derive(Accounts)]
pub struct ConstructorCreate<'info> {
    #[account(mut)]
    pub creator_key: Signer<'info>,

    #[account(
        init,
        payer = creator_key,
        space = 8 + Constructor::INIT_SPACE,
        seeds = [
            SEED_PREFIX,
            program_config.key().as_ref(),
            SEED_CONSTRUCTOR,
            creator_key.key().as_ref(),
        ],
        bump,
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

    pub system_program: Program<'info, System>,
}

impl<'info> ConstructorCreate<'info> {
    fn validate(&self, args: &ConstructorCreateArgs) -> Result<()> {
        let Self {
            creator_key,
            ..
        } = self;

        require_keys_neq!(
            creator_key.key(),
            args.creator_key,
            CustomError::InvalidAccount,
        );

        Ok(())
    }

    #[access_control(ctx.accounts.validate(&args))]
    pub fn constructor_create(ctx: Context<Self>, args: ConstructorCreateArgs) -> Result<()> {
        let authority = args.authority;
        let creator_key = args.creator_key;

        let competition_index = 0;
        let transaction_fee = args.transaction_fee;
        
        let bump = ctx.bumps.constructor;

        ctx.accounts.constructor.set_inner(Constructor {
            authority,
            creator_key,
            competition_index,
            transaction_fee,
            bump,
        });

        ctx.accounts.constructor.invariant()?;

        Ok(())
    }
}