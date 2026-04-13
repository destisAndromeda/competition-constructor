use anchor_lang::prelude::*;

use crate::state::*;
use crate::seeds::*;
use crate::error::*;
use crate::competition_systems::swiss_system::local_state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwissSystemParticipantCreateArgs {
    pub competition_index: u64,

    pub participant: Pubkey,

    pub authority: Pubkey,
}

#[derive(Accounts)]
#[instruction(args: SwissSystemParticipantCreateArgs)]
pub struct SwissSystemParticipantCreate<'info> {
    #[account(mut)]
    pub organizer: Signer<'info>,

    #[account(
        init,
        payer = organizer,
        space = 8 + Participant::INIT_SPACE,
        seeds = [
            SEED_PREFIX,
            swiss_system.creator_key.as_ref(),
            SEED_PARTICIPANT,
            args.participant.as_ref(),
        ],
        bump,
    )]
    pub participant: Account<'info, Participant>,

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
    pub swiss_system: Account<'info, SwissSystem>,

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
    pub fn swiss_system_participant_create(
        ctx: Context<Self>,
        args: SwissSystemParticipantCreateArgs,
    ) -> Result<()> {
        let authority = args.authority;
        let participant = args.participant;

        let status = ParticipantStatus::Active {
            timestamp: Clock::get()?.unix_timestamp,
        };

        let points = 0;
        let bump = ctx.bumps.participant;

        ctx.accounts.participant.set_inner(Participant {
            authority,
            participant,
            points,
            status,
            bump,
        });

        Ok(())
    }
}