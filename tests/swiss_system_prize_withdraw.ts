import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { CompetitionConstructorProgram } from '../target/types/competition_constructor_program';
import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { state } from './shared.ts';
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

describe('swiss_system_prize_withdraw', () => {
  it('withdraw with unauthorized signer', async () => {
    const unauthorized = anchor.web3.Keypair.generate();
  
    const signature = await provider.connection.requestAirdrop(
      unauthorized.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1,
    );
    
      await program.methods
        .swissSystemPrizeWithdraw({
          competitionIndex: new anchor.BN(state.competitionIndex),
          vaultIndex: new anchor.BN(state.vaultIndex),
        })
        .accounts({
          caller: unauthorized.publicKey,
          valut: state.vaultPda,
          swissSystem: state.swissSystemPda,
          constructor: state.constructorPda,
          programConfig: state.programConfigPda,
          systemProgram: systemProgram,
          winner: state.participantOne.publicKey,
        })
        .signers([unauthorized])
        .rpc();
    });
});