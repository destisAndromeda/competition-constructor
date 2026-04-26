import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { CompetitionConstructorProgram } from '../target/types/competition_constructor_program';
import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { state } from './shared.ts';

const { expect } = chai;

chai.use(chaiAsPromised);

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.CompetitionConstructorProgram as Program<CompetitionConstructorProgram>;
const systemProgram = anchor.web3.SystemProgram.programId;

describe('swiss_system_create', () => {
  it('create swissSystem account', async() => {

    let signature = await provider.connection.requestAirdrop(
      state.organizerSwissSystem.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1,
    );

    await provider.connection.confirmTransaction(signature);

    await program.methods
      .swissSystemCreate({
        stageInfo: state.stageInfo,
        creatorKey: state.creatorKeySwissSystem.publicKey,
        authority: state.authoritySwissSystem.publicKey,
      })
      .accounts({
        organizer: state.organizerSwissSystem.publicKey,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
        systemProgram: systemProgram,
      })
      .signers([state.organizerSwissSystem])
      .rpc();

      const swissSystemAccount = await program.account.swissSystem.fetch(state.swissSystemPda);

      expect(swissSystemAccount.authority.equals(state.authoritySwissSystem.publicKey)).to.be.true;
      expect(swissSystemAccount.creatorKey.equals(state.creatorKeySwissSystem.publicKey)).to.be.true;
      expect(swissSystemAccount.stageInfo.registrationPeriod.eq(state.stageInfo.registrationPeriod)).to.be.true;  
      expect(swissSystemAccount.stageInfo.competitionPeriod.eq(state.stageInfo.competitionPeriod)).to.be.true;  
      expect(swissSystemAccount.stageInfo.withdrawPeriod.eq(state.stageInfo.withdrawPeriod)).to.be.true;  
  });
});