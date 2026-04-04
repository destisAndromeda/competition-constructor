use anchor_lang::prelude::*;

mod error;
mod seeds;
mod state;
mod instructions;

use instructions::*;

declare_id!("8bDwCU1Y598BoD2BWnchBCeuvQ4rW3DUbV4c3tDvRkVX");

#[program]
pub mod competition_constructor_program {
    use super::*;

    pub fn program_config_init(
        ctx: Context<ProgramConfigInit>,
        args: ProgramConfigInitArgs,
    ) -> Result<()> {
        ProgramConfigInit::program_config_init(ctx, args)
    }

    // pub fn program_config_update_authority
    
    // pub fn program_config_update_creator_key

    // pub fn program_config_update_tresury
}

