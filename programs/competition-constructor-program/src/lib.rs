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
        ProgramConfigInit::
            program_config_init(ctx, args)
    }

    pub fn program_config_update(
        ctx: Context<ProgramConfigUpdate>,
        args: ProgramConfigUpdateAuthorityArgs,
    ) -> Result<()> {
        ProgramConfigUpdate::
            program_config_update_authority(ctx, args)
    }

    pub fn program_config_update_creator_key(
        ctx: Context<ProgramConfigUpdate>,
        args: ProgramConfigUpdateCreatorKeyArgs,
    ) -> Result<()> {
        ProgramConfigUpdate::
            program_config_update_creator_key(ctx, args)
    }

    pub fn program_config_update_tresury(
        ctx: Context<ProgramConfigUpdate>,
        args: ProgramConfigUpdateTreasuryArgs,
    ) -> Result<()> {
        ProgramConfigUpdate::
            progmram_config_update_treasury(ctx, args)
    }

    pub fn constructor_create(
        ctx: Context<ConstructorCreate>,
        args: ConstructorCreateArgs,
    ) -> Result<()> {
        ConstructorCreate::
            constructor_create(ctx, args)
    }

    pub fn constructor_update_authority(
        ctx: Context<ConstructorUpdate>,
        args: ConstructorUpdateAuthorityArgs,
    ) -> Result<()> {
        ConstructorUpdate::
            constructor_update_authority(ctx, args)
    }

    pub fn constructor_update_creator_key(
        ctx: Context<ConstructorUpdate>,
        args: ConstructorUpdateCreatorKeyArgs,
        ) -> Result<()> {
        ConstructorUpdate::
            constructor_update_creator_key(ctx, args)
    }

    pub fn constructor_update_transaction_fee(
        ctx: Context<ConstructorUpdate>,
        args: ConstructorUpdateTransactionFeeArgs,
    ) -> Result<()> {
        ConstructorUpdate::
            constructor_update_transaction_fee(ctx, args)
    }
}

