import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { CompetitionConstructorProgram } from '../../target/types/competition_constructor_program';
import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { state } from '../shared';
import { PublicKey } from '@solana/web3.js';

const { expect } = chai;

chai.use(chaiAsPromised);

const SEED_PREFIX = 'competition_constructor';
const SEED_LEADER_BOARD = 'leaderboard';
const SEED_PARTICIPANT = 'participant';

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.CompetitionConstructorProgram as Program<CompetitionConstructorProgram>;
const systemProgram = anchor.web3.SystemProgram.programId;

describe('swiss_system_winner_determine', () => {
  it('should write winner address to the vault', async () => {
    const unauthorized = anchor.web3.Keypair.generate();

    const signature = await provider.connection.requestAirdrop(
      unauthorized.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1,
    );

    await provider.connection.confirmTransaction(signature);

    await program.methods
      .swissSystemWinnerDetermine({
        competitionIndex: new anchor.BN(state.competitionIndex),
        organizer: state.organizerSwissSystem.publicKey,
        vaultIndex: new anchor.BN(state.vaultIndex),
      })
      .accounts({
        caller: unauthorized.publicKey,
        vault: state.vaultPda,
        leaderboard: state.leaderboardPda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        prgoramConfig: state.programConfigPda,
      })
      .signers([unauthorized])
      .rpc();

      const vaultAccount = await program.account.vault.fetch(state.vaultPda);
      const leaderboardAccount = await program.account.leaderBoard.fetch(state.leaderboardPda);
      
      expect(vaultAccount.winner?.equals(leaderboardAccount.list[0]?.address)).to.be.true;
  });
});