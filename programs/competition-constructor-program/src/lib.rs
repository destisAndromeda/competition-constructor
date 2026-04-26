use anchor_lang::prelude::*;

mod error;
mod seeds;
mod state;
mod instructions;
mod competition_systems;

use instructions::*;
use competition_systems::*;

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

    pub fn program_config_authority_update(
        ctx: Context<ProgramConfigUpdate>,
        args: ProgramConfigUpdateArgs,
    ) -> Result<()> {
        ProgramConfigUpdate::
            program_config_authority_update(ctx, args)
    }

    pub fn program_config_creator_key_update(
        ctx: Context<ProgramConfigUpdate>,
        args: ProgramConfigUpdateArgs,
    ) -> Result<()> {
        ProgramConfigUpdate::
            program_config_creator_key_update(ctx, args)
    }

    pub fn program_config_treasury_update(
        ctx: Context<ProgramConfigUpdate>,
        args: ProgramConfigUpdateArgs,
    ) -> Result<()> {
        ProgramConfigUpdate::
            program_config_treasury_update(ctx, args)
    }

    pub fn constructor_create(
        ctx: Context<ConstructorCreate>,
        args: ConstructorCreateArgs,
    ) -> Result<()> {
        ConstructorCreate::
            constructor_create(ctx, args)
    }

    pub fn constructor_authority_update(
        ctx: Context<ConstructorAuthorityUpdate>,
        args: ConstructorAuthorityUpdateArgs,
    ) -> Result<()> {
        ConstructorAuthorityUpdate::
            constructor_authority_update(ctx, args)
    }

    pub fn constructor_creator_key_update(
        ctx: Context<ConstructorUpdate>,
        args: ConstructorUpdateArgs,
        ) -> Result<()> {
        ConstructorUpdate::
            constructor_creator_key_update(ctx, args)
    }

    pub fn constructor_transaction_fee_update(
        ctx: Context<ConstructorUpdate>,
        args: ConstructorTransactionFeeUpdateArgs,
    ) -> Result<()> {
        ConstructorUpdate::
            constructor_transaction_fee_update(ctx, args)
    }

    pub fn swiss_system_create(
        ctx: Context<SwissSystemCreate>,
        args: SwissSystemCreateArgs,
    ) -> Result<()> {
        SwissSystemCreate::
            swiss_system_create(ctx, args)
    }

    pub fn swiss_system_authority_update(
        ctx: Context<SwissSystemUpdate>,
        args: SwissSystemUpdateArgs,
    ) -> Result<()> {
        SwissSystemUpdate::
            swiss_system_authority_update(ctx, args)
    }
    
    pub fn swiss_system_creator_key_update(
        ctx: Context<SwissSystemUpdate>,
        args: SwissSystemUpdateArgs,
    ) -> Result<()> {
        SwissSystemUpdate::
            swiss_system_creator_key_update(ctx, args)
    }

    pub fn swiss_system_stage_update(
        ctx: Context<SwissSystemStageUpdate>,
        args: SwissSystemStageUpdateArgs,
    ) -> Result<()> {
        SwissSystemStageUpdate::
            swiss_system_stage_update(ctx, args)
    }

    pub fn swiss_system_vault_create(
        ctx: Context<SwissSystemVaultCreate>,
        args: SwissSystemVaultCreateArgs,
    ) -> Result<()> {
        SwissSystemVaultCreate::
            swiss_system_vault_create(ctx, args)
    }

    pub fn swiss_system_prize_withdraw(
        ctx: Context<SwissSystemPrizeWithdraw>,
        args: SwissSystemPrizeWithdrawArgs,
    ) -> Result<()> {
        SwissSystemPrizeWithdraw::
            swiss_system_prize_withdraw(ctx, args)
    }

    pub fn swiss_system_participant_create(
        ctx: Context<SwissSystemParticipantCreate>,
        args: SwissSystemParticipantCreateArgs,
    ) -> Result<()> {
        SwissSystemParticipantCreate::
            swiss_system_participant_create(ctx, args)
    }

    pub fn swiss_system_leaderboard_create(
        ctx: Context<SwissSystemLeaderBoardCreate>,
        args: SwissSystemLeaderBoardCreateArgs,
    ) -> Result<()> {
        SwissSystemLeaderBoardCreate::
            swiss_system_leaderboard_create(ctx, args)
    }

    pub fn swiss_system_points_award(
        ctx: Context<SwissSystemPointsAward>,
        args: SwissSystemPointsAwardArgs
    ) -> Result<()> {
        SwissSystemPointsAward::
            swiss_system_points_award(ctx, args)
    }
}

