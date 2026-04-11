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

    pub fn constructor_create(
        ctx: Context<ConstructorCreate>,
        args: ConstructorCreateArgs,
    ) -> Result<()> {
        ConstructorCreate::constructor_create(ctx, args)
    }

    // pub fn constructor_authority_update

    // pub fn consturctor_creator_key_update (if set new key then should put zero into transaction_index)
    
    // pub fn constructor_transaction_fee_update
}

