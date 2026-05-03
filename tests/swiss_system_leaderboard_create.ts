import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { CompetitionConstructorProgram } from '../target/types/competition_constructor_program';
import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { state } from './shared.ts';

const { expect } = chai;

chai.use(chaiAsPromised);

const SEED_PREFIX = 'competition_constructor';
const SEED_LEADER_BOARD = 'leaderboard';

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.CompetitionConstructorProgram as Program<CompetitionConstructorProgram>;
const systemProgram = anchor.web3.SystemProgram.programId;

describe('swiss_system_leaderboard_create', () => {
  it('error: should fail with unauthoritzed organizer', async () => {
    const unauthorized = anchor.web3.Keypair.generate();

    const signature = await provider.connection.requestAirdrop(
      unauthorized.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1,
    );

    await provider.connection.confirmTransaction(signature);

    const [unauthorizedLeaderboardPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(SEED_PREFIX),
        state.creatorKeySwissSystem.publicKey.toBuffer(),
        Buffer.from(SEED_LEADER_BOARD),
        unauthorized.publicKey.toBuffer(),
      ],
      program.programId
    );

    await expect(
      program.methods
      .swissSystemLeaderboardCreate({
        competitionIndex: new anchor.BN(state.competitionIndex),
      })
      .accounts({
        organizer: unauthorized.publicKey,
        leaderboard: unauthorizedLeaderboardPda,
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

  it('leader board create', async () => {
    const tx = await program.methods
      .swissSystemLeaderboardCreate({
        competitionIndex: new anchor.BN(state.competitionIndex),
      })
      .accounts({
        organizer: state.organizerSwissSystem.publicKey,
        leaderboard: state.leaderboardPda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
        systemProgram: systemProgram,
      })
      .signers([state.organizerSwissSystem])
      .rpc();

    const leaderboardAccount = await program.account.leaderBoard.fetch(state.leaderboardPda);
    expect(leaderboardAccount).to.not.be.null;
    expect(leaderboardAccount.list).to.deep.equal([]);
    
    const accountInfo = await provider.connection.getAccountInfo(state.leaderboardPda);
    expect(accountInfo!.owner.toString()).to.equal(program.programId.toString());
  });

  it('error: should fail when try initialize account twice', async () => {
    await expect(
      program.methods
      .swissSystemLeaderboardCreate({
        competitionIndex: new anchor.BN(state.competitionIndex),
      })
      .accounts({
        organizer: state.organizerSwissSystem.publicKey,
        leaderboard: state.leaderboardPda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
        systemProgram: systemProgram,
      })
      .signers([state.organizerSwissSystem])
      .rpc()
    ).to.be.rejectedWith(/already in use/i);
  });
});