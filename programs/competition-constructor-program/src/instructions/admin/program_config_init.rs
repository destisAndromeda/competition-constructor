use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ProgramConfigInitArgs {
    pub creator_key: Pubkey,

    pub treasury: Pubkey,
}

#[derive(Accounts)]
pub struct ProgramConfigInit<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + ProgramConfig::INIT_SPACE,
        seeds = [
            SEED_PREFIX,
            SEED_PROGRAM_CONFIG,
        ],
        bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,

    pub system_program: Program<'info, System>,
}

impl<'info> ProgramConfigInit<'info> {
    pub fn program_config_init(ctx: Context<Self>, args: ProgramConfigInitArgs) -> Result<()> {
        
        
        Ok(())
    }
}