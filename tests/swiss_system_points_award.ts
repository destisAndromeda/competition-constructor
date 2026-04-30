import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { CompetitionConstructorProgram } from '../target/types/competition_constructor_program';
import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { state } from './shared.ts';
import { PublicKey } from '@solana/web3.js';
import { isThawAccountInstruction } from '@solana/spl-token';

const { expect } = chai;

chai.use(chaiAsPromised);

const SEED_PREFIX = 'competition_constructor';
const SEED_LEADER_BOARD = 'leaderboard';
const SEED_PARTICIPANT = 'participant';

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.CompetitionConstructorProgram as Program<CompetitionConstructorProgram>;
const systemProgram = anchor.web3.SystemProgram.programId;

describe('swiss_system_points_award', () => {
  it('error: should fail with invalid authority', async () => {
    const award = new anchor.BN(0);
    let invalidAuthority = anchor.web3.Keypair.generate();

    const signature = await provider.connection.requestAirdrop(
      invalidAuthority.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1,
    );
    
    await provider.connection.confirmTransaction(signature);

    await expect(
      program.methods
        .swissSystemPointsAward({
          competitionIndex: new anchor.BN(state.competitionIndex),
          organizer: state.organizerSwissSystem.publicKey,
          participant: state.participantOne.publicKey,
          points: award,
        })
        .accounts({
          authority: invalidAuthority.publicKey,
          participant: state.participantOnePda,
          leaderboard: state.leaderboardPda,
          swissSystem: state.swissSystemPda,
          constructor: state.constructorPda,
          programConfig: state.programConfigPda,
        })
        .signers([invalidAuthority])
        .rpc()
    ).to.be.rejected.then((err) => {
      expect(err.error.errorCode.code).to.be.equals('Unauthorized');
    });
  });
  
  it('error: should fail with 0 points', async () => {
    const award = new anchor.BN(0);

    await expect(
      program.methods
        .swissSystemPointsAward({
          competitionIndex: new anchor.BN(state.competitionIndex),
          organizer: state.organizerSwissSystem.publicKey,
          participant: state.participantOne.publicKey,
          points: award,
        })
        .accounts({
          authority: state.authoritySwissSystem.publicKey,
          participant: state.participantOnePda,
          leaderboard: state.leaderboardPda,
          swissSystem: state.swissSystemPda,
          constructor: state.constructorPda,
          programConfig: state.programConfigPda,
        })
        .signers([state.authoritySwissSystem])
        .rpc()
    ).to.be.rejected.then((err) => {
      expect(err.error.errorCode.code).to.be.equals('IncorrectValue');
    });
  });

  it('award points for participant number one', async () => {
    const award = new anchor.BN(1);

    await program.methods
      .swissSystemPointsAward({
        competitionIndex: new anchor.BN(state.competitionIndex),
        organizer: state.organizerSwissSystem.publicKey,
        participant: state.participantOne.publicKey,
        points: award,
      })
      .accounts({
        authority: state.authoritySwissSystem.publicKey,
        participant: state.participantOnePda,
        leaderboard: state.leaderboardPda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
      })
      .signers([state.authoritySwissSystem])
      .rpc();

    const participantAccount = await program.account.participant.fetch(state.participantOnePda);
    expect(participantAccount.points.eq(award)).to.be.true;
  });

  it('leaderboard should be changed for the first time', async () => {
    const leaderboardAccount = await program.account.leaderBoard.fetch(state.leaderboardPda);

    expect(leaderboardAccount.list).to.have.lengthOf(1);

    const entry = leaderboardAccount.list[0];
    expect(entry.address?.toString()).to.equal(state.participantOne.publicKey.toString());
    expect(entry.points.eq(new anchor.BN(1))).to.be.true;
  });

  it('award points for participant number two', async () => {
    const award = new anchor.BN(2);

    await program.methods
      .swissSystemPointsAward({
        competitionIndex: new anchor.BN(state.competitionIndex),
        organizer: state.organizerSwissSystem.publicKey,
        participant: state.participantTwo.publicKey,
        points: award,
      })
      .accounts({
        authority: state.authoritySwissSystem.publicKey,
        participant: state.participantTwoPda,
        leaderboard: state.leaderboardPda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
      })
      .signers([state.authoritySwissSystem])
      .rpc();

    const participantAccount = await program.account.participant.fetch(state.participantTwoPda);
    expect(participantAccount.points.eq(award)).to.be.true;
  });

  it('leaderboard should be changed for the second time', async () => {
    const leaderboardAccount = await program.account.leaderBoard.fetch(state.leaderboardPda);

    expect(leaderboardAccount.list).to.have.lengthOf(2);

    const entry = leaderboardAccount.list[0];
    expect(entry.address?.toString()).to.equal(state.participantTwo.publicKey.toString());
    expect(entry.points.eq(new anchor.BN(2))).to.be.true;
  });

  it('add 1 point and check if position has not changed | participantOne', async () => {
    const award = new anchor.BN(1);

    await program.methods
      .swissSystemPointsAward({
        competitionIndex: new anchor.BN(state.competitionIndex),
        organizer: state.organizerSwissSystem.publicKey,
        participant: state.participantOne.publicKey,
        points: award,
      })
      .accounts({
        authority: state.authoritySwissSystem.publicKey,
        participant: state.participantOnePda,
        leaderboard: state.leaderboardPda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
      })
      .signers([state.authoritySwissSystem])
      .rpc();

    const leaderboardAccount = await program.account.leaderBoard.fetch(state.leaderboardPda);

    expect(leaderboardAccount.list).to.have.lengthOf(2);

    const entry = leaderboardAccount.list[1];
    expect(entry.address?.toString()).to.equal(state.participantOne.publicKey.toString());
    expect(entry.points.eq(new anchor.BN(2))).to.be.true;
  });

  it('add 2 points and check if position has not changed | participantThree', async () => {
    const award = new anchor.BN(2);

    await program.methods
      .swissSystemPointsAward({
        competitionIndex: new anchor.BN(state.competitionIndex),
        organizer: state.organizerSwissSystem.publicKey,
        participant: state.participantThree.publicKey,
        points: award,
      })
      .accounts({
        authority: state.authoritySwissSystem.publicKey,
        participant: state.participantThreePda,
        leaderboard: state.leaderboardPda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
      })
      .signers([state.authoritySwissSystem])
      .rpc();

    const leaderboardAccount = await program.account.leaderBoard.fetch(state.leaderboardPda);

    expect(leaderboardAccount.list).to.have.lengthOf(3);

    const entry = leaderboardAccount.list[2];
    expect(entry.address?.toString()).to.equal(state.participantThree.publicKey.toString());
    expect(entry.points.eq(new anchor.BN(2))).to.be.true;
  });

  it('error: should fail with incorrect participant and PDA', async () => {
    let invalidAccount = anchor.web3.Keypair.generate();
    const award = new anchor.BN(1);

    let [invalidAccountPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from(SEED_PREFIX),
        state.creatorKeySwissSystem.publicKey.toBuffer(),
        Buffer.from(SEED_PARTICIPANT),
        invalidAccount.publicKey.toBuffer(),
      ],
      program.programId,
    );

    await expect(
      program.methods
        .swissSystemPointsAward({
          competitionIndex: new anchor.BN(state.competitionIndex),
          organizer: state.organizerSwissSystem.publicKey,
          participant: invalidAccount.publicKey,
          points: award,
        })
        .accounts({
          authority: state.authoritySwissSystem.publicKey,
          participant: invalidAccountPda,
          leaderboard: state.leaderboardPda,
          swissSystem: state.swissSystemPda,
          constructor: state.constructorPda,
          programConfig: state.programConfigPda,
        })
        .signers([state.authoritySwissSystem])
        .rpc()
    ).to.be.rejectedWith(/AccountNotInitialized|account does not exist/i);
  });

  it('error: should fail with incorrect participant', async () => {
    let invalidAccount = anchor.web3.Keypair.generate();
    const award = new anchor.BN(1);

    await expect(
      program.methods
        .swissSystemPointsAward({
          competitionIndex: new anchor.BN(state.competitionIndex),
          organizer: state.organizerSwissSystem.publicKey,
          participant: invalidAccount.publicKey,
          points: award,
        })
        .accounts({
          authority: state.authoritySwissSystem.publicKey,
          participant: state.participantOnePda,
          leaderboard: state.leaderboardPda,
          swissSystem: state.swissSystemPda,
          constructor: state.constructorPda,
          programConfig: state.programConfigPda,
        })
        .signers([state.authoritySwissSystem])
        .rpc()
    ).to.be.rejectedWith(/InvalidAccount|ConstraintSeeds/i);
  });

  it('try to add with low points', async () => {
    const weakAward = new anchor.BN(1);

    await program.methods
      .swissSystemPointsAward({
        competitionIndex: new anchor.BN(state.competitionIndex),
        organizer: state.organizerSwissSystem.publicKey,
        participant: state.participantFour.publicKey,
        points: weakAward,
      })
      .accounts({
        authority: state.authoritySwissSystem.publicKey,
        participant: state.participantFourPda,
        leaderboard: state.leaderboardPda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
      })
      .signers([state.authoritySwissSystem])
      .rpc();

    const leaderboardAfterWeak = await program.account.leaderBoard.fetch(state.leaderboardPda);

    expect(leaderboardAfterWeak.list).to.have.lengthOf(3);

    const addresses = leaderboardAfterWeak.list.map(e => e.address?.toString());
    expect(addresses).to.not.include(state.participantFour.publicKey.toString());
  });

  it('try to add with high points', async () => {
    const strongAward = new anchor.BN(10);

    await program.methods
      .swissSystemPointsAward({
        competitionIndex: new anchor.BN(state.competitionIndex),
        organizer: state.organizerSwissSystem.publicKey,
        participant: state.participantFour.publicKey,
        points: strongAward,
      })
      .accounts({
        authority: state.authoritySwissSystem.publicKey,
        participant: state.participantFourPda,
        leaderboard: state.leaderboardPda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
      })
      .signers([state.authoritySwissSystem])
      .rpc();

    const leaderboardAfterStrong = await program.account.leaderBoard.fetch(state.leaderboardPda);
    expect(leaderboardAfterStrong.list).to.have.lengthOf(3);

    expect(leaderboardAfterStrong.list[0].address?.toString())
      .to.equal(state.participantFour.publicKey.toString());
    expect(leaderboardAfterStrong.list[0].points.eq(new anchor.BN(11))).to.be.true;
  });
});