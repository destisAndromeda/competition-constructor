use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;
use crate::competition_systems::swiss_system::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwissSystemParticipantCreateArgs {
    pub competition_index: u64,

    pub participant: Pubkey,
}

#[derive(Accounts)]
#[instruction(args: SwissSystemParticipantCreateArgs)]
pub struct SwissSystemParticipantCreate<'info> {
    #[account(mut)]
    pub organizer: Signer<'info>,

    #[account(
        init,
        payer = organizer,
        space = 8 + local_state::Participant::INIT_SPACE,
        seeds = [
            SEED_PREFIX,
            swiss_system.creator_key.as_ref(),
            SEED_PARTICIPANT,
            args.participant.as_ref(),
        ],
        bump,
    )]
    pub participant: Account<'info, local_state::Participant>,

    #[account(
        has_one = organizer
            @ CustomError::Unauthorized,
        seeds = [
            SEED_PREFIX,
            constructor.creator_key.as_ref(),
            SEED_COMPETITION,
            &args.competition_index.to_le_bytes(),
        ],
        bump  = swiss_system.bump,
    )]
    pub swiss_system: Account<'info, local_state::SwissSystem>,

    #[account(
        seeds = [
            SEED_PREFIX,
            program_config.key().as_ref(),
            SEED_CONSTRUCTOR,
            program_config.creator_key.as_ref(),
        ],
        bump  = constructor.bump,
    )]
    pub constructor: Account<'info, Constructor>,

    #[account(
        seeds = [
            SEED_PREFIX,
            SEED_PROGRAM_CONFIG,
        ],
        bump  = program_config.bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,

    pub system_program: Program<'info, System>,
}

impl<'info> SwissSystemParticipantCreate<'info> {
    fn validate(&self, args: &SwissSystemParticipantCreateArgs) -> Result<()> {
        let Self {
            organizer,
            swiss_system,
            ..
        } = self;

        #[cfg(not(feature = "testing"))]
        {
            let current = Clock::get()?.unix_timestamp;
            
            if let Some(local_state::Stage::RegistrationPeriod { timestamp }) = swiss_system.stage {
                require!(
                    current < timestamp,
                    CustomError::InvalidStage,
                );
            } else {
                return err!(CustomError::InvalidStage);
            };
        }

        require_keys_neq!(
            args.participant,
            organizer.key(),
            CustomError::InvalidAccount,
        );

        require_keys_neq!(
            args.participant,
            swiss_system.creator_key,
            CustomError::InvalidAccount,
        );

        require_keys_neq!(
            args.participant,
            swiss_system.authority,
            CustomError::InvalidAccount,
        );

        Ok(())
    }

    #[access_control(ctx.accounts.validate(&args))]
    pub fn swiss_system_participant_create(
        ctx: Context<Self>,
        args: SwissSystemParticipantCreateArgs,
    ) -> Result<()> {
        let participant = args.participant;

        let status = ParticipantStatus::Active {
            timestamp: Clock::get()?.unix_timestamp,
        };

        let points = 0;
        let bump = ctx.bumps.participant;

        ctx.accounts.participant.set_inner( local_state::Participant {
            participant,
            points,
            status,
            bump,
        });

        Ok(())
    }
}