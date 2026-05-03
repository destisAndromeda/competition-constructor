use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;

#[cfg(not(feature = "testing"))]
const INITIALIZER: Pubkey = pubkey!("GtmrJehR49tXwFh7W4x2kGy61czbEboYSkHQDJw7Ggeb"); // yeah, im slick ass

#[cfg(feature = "testing")]
const INITIALIZER: Pubkey = pubkey!("FAHrwTPYRsBFkjV8A5zF8TQrbHhMQimDPn1gQVtL4Sjj");

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ProgramConfigInitArgs {
    pub creator_key: Pubkey,

    pub treasury: Pubkey,
}

#[derive(Accounts)]
pub struct ProgramConfigInit<'info> {
    #[account(
        mut,
        address = INITIALIZER
            @ CustomError::Unauthorized,
    )]
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
        let authority = ctx.accounts.authority.key();
        let creator_key = args.creator_key;

        let _reserved: [u8; 64] = [0u8; 64];
        let treasury = args.treasury;
        let bump = ctx.bumps.program_config;

        ctx.accounts.program_config.set_inner( ProgramConfig {
            authority,
            creator_key,
            treasury,
            _reserved,
            bump,
        });

        ctx.accounts.program_config.invariant()?;

        Ok(())
    }
}