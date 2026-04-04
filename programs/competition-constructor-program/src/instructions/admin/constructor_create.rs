use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ConstructorCreateArgs {
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
    pub fn constructor_create(ctx: Context<Self>, args: ConstructorCreateArgs) -> Result<()> {
        Ok(())
    }
}