import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { CompetitionConstructorProgram } from '../target/types/competition_constructor_program';
import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { state } from './shared.ts';

const { expect } = chai;

chai.use(chaiAsPromised);

const SEED_PREFIX = 'competition_constructor';
const SEED_PARTICIPANT = 'participant';

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.CompetitionConstructorProgram as Program<CompetitionConstructorProgram>;
const systemProgram = anchor.web3.SystemProgram.programId;

describe('swiss_system_particiapnt_create', () => {
  it('error: should fail with unauthoritzed organizer', async () => {
    const unauthorized = anchor.web3.Keypair.generate();

    const signature = await provider.connection.requestAirdrop(
      unauthorized.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1,
    );

    await provider.connection.confirmTransaction(signature);

    await expect(
      program.methods
      .swissSystemParticipantCreate({
        competitionIndex: new anchor.BN(state.competitionIndex),
        participant: state.participantOne.publicKey,
      })
      .accounts({
        organizer: unauthorized.publicKey,
        participant: state.participantOnePda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
        systemProgram: systemProgram,
      })
      .signers([unauthorized])
      .rpc()
    ).to.be.rejected.then((err) => {
      expect(err.error.errorCode.code).to.equals('Unauthorized');
    });
  });

  it('error: should fail with same organizer and participant', async () => {
    const [organizerAsParticipantPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(SEED_PREFIX),
        state.creatorKeySwissSystem.publicKey.toBuffer(),
        Buffer.from(SEED_PARTICIPANT),
        state.organizerSwissSystem.publicKey.toBuffer(),
      ],
      program.programId
    );

    await expect(
      program.methods
        .swissSystemParticipantCreate({
          competitionIndex: new anchor.BN(state.competitionIndex),
          participant: state.organizerSwissSystem.publicKey,
        })
        .accounts({
        organizer: state.organizerSwissSystem.publicKey,
        participant: organizerAsParticipantPda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
        systemProgram: systemProgram,
        })
        .signers([state.organizerSwissSystem])
        .rpc()
    ).to.be.rejected.then((err) => {
      expect(err.error.errorCode.code).to.equals('InvalidAccount');
    });
  });

  it('error: should fail with same organizer and creatorKey', async () => {
    const [creatorKeyAsParticipantPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(SEED_PREFIX),
        state.creatorKeySwissSystem.publicKey.toBuffer(),
        Buffer.from(SEED_PARTICIPANT),
        state.creatorKeySwissSystem.publicKey.toBuffer(),
      ],
      program.programId
    );

    await expect(
      program.methods
        .swissSystemParticipantCreate({
          competitionIndex: new anchor.BN(state.competitionIndex),
          participant: state.creatorKeySwissSystem.publicKey,
        })
        .accounts({
        organizer: state.organizerSwissSystem.publicKey,
        participant: creatorKeyAsParticipantPda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
        systemProgram: systemProgram,
        })
        .signers([state.organizerSwissSystem])
        .rpc()
    ).to.be.rejected.then((err) => {
      expect(err.error.errorCode.code).to.equals('InvalidAccount');
    });
  });

  it('error: should fail with same organizer and authority', async () => {
    const [authorityAsParticipantPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(SEED_PREFIX),
        state.creatorKeySwissSystem.publicKey.toBuffer(),
        Buffer.from(SEED_PARTICIPANT),
        state.authoritySwissSystem.publicKey.toBuffer(),
      ],
      program.programId
    );

    await expect(
      program.methods
        .swissSystemParticipantCreate({
          competitionIndex: new anchor.BN(state.competitionIndex),
          participant: state.authoritySwissSystem.publicKey,
        })
        .accounts({
        organizer: state.organizerSwissSystem.publicKey,
        participant: authorityAsParticipantPda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
        systemProgram: systemProgram,
        })
        .signers([state.organizerSwissSystem])
        .rpc()
    ).to.be.rejected.then((err) => {
      expect(err.error.errorCode.code).to.equals('InvalidAccount');
    });
  });

  it('participant create', async () => {
    await program.methods
      .swissSystemParticipantCreate({
        competitionIndex: new anchor.BN(state.competitionIndex),
        participant: state.participantOne.publicKey,
      })
      .accounts({
        organizer:   state.organizerSwissSystem.publicKey,
        participant: state.participantOnePda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
        systemProgram: systemProgram,
      })
      .signers([state.organizerSwissSystem])
      .rpc();
    
      const participantAccount = await program.account.participant.fetch(state.participantOnePda);
      
      expect(participantAccount.participant.equals(state.participantOne.publicKey)).to.be.true;
      expect(participantAccount.points.eq(new anchor.BN(0))).to.be.true;
      expect(participantAccount.status.active).to.not.be.undefined;
  });

  it('error: should fail when try initialize account twice', async () => {
    await expect(
      program.methods
      .swissSystemParticipantCreate({
        competitionIndex: new anchor.BN(state.competitionIndex),
        participant: state.participantOne.publicKey,
      })
      .accounts({
        organizer: state.organizerSwissSystem.publicKey,
        participant: state.participantOnePda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
        systemProgram: systemProgram,
      })
      .signers([state.organizerSwissSystem])
      .rpc()
    ).to.be.rejectedWith(/already in use/i);
  });

  it('create second participant', async () => {
    await program.methods
      .swissSystemParticipantCreate({
        competitionIndex: new anchor.BN(state.competitionIndex),
        participant: state.participantTwo.publicKey,
      })
      .accounts({
        organizer:   state.organizerSwissSystem.publicKey,
        participant: state.participantTwoPda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
        systemProgram: systemProgram,
      })
      .signers([state.organizerSwissSystem])
      .rpc();
    
      const participantAccount = await program.account.participant.fetch(state.participantTwoPda);
      
      expect(participantAccount.participant.equals(state.participantTwo.publicKey)).to.be.true;
      expect(participantAccount.points.eq(new anchor.BN(0))).to.be.true;
      expect(participantAccount.status.active).to.not.be.undefined;
  });

  it('create thrid participant', async () => {
    await program.methods
      .swissSystemParticipantCreate({
        competitionIndex: new anchor.BN(state.competitionIndex),
        participant: state.participantThree.publicKey,
      })
      .accounts({
        organizer:   state.organizerSwissSystem.publicKey,
        participant: state.participantThreePda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
        systemProgram: systemProgram,
      })
      .signers([state.organizerSwissSystem])
      .rpc();
    
      const participantAccount = await program.account.participant.fetch(state.participantThreePda);
      
      expect(participantAccount.participant.equals(state.participantThree.publicKey)).to.be.true;
      expect(participantAccount.points.eq(new anchor.BN(0))).to.be.true;
      expect(participantAccount.status.active).to.not.be.undefined;
  });

  it('create fourth participant', async () => {
    await program.methods
      .swissSystemParticipantCreate({
        competitionIndex: new anchor.BN(state.competitionIndex),
        participant: state.participantFour.publicKey,
      })
      .accounts({
        organizer:   state.organizerSwissSystem.publicKey,
        participant: state.participantFourPda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
        systemProgram: systemProgram,
      })
      .signers([state.organizerSwissSystem])
      .rpc();

      const participantAccount = await program.account.participant.fetch(state.participantFourPda);
      
      expect(participantAccount.participant.equals(state.participantFour.publicKey)).to.be.true;
      expect(participantAccount.points.eq(new anchor.BN(0))).to.be.true;
      expect(participantAccount.status.active).to.not.be.undefined;
  });
});