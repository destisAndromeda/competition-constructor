import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { CompetitionConstructorProgram } from '../target/types/competition_constructor_program';
import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { state } from './shared.ts';

const { expect } = chai;

chai.use(chaiAsPromised);

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.CompetitionConstructorProgram as Program<CompetitionConstructorProgram>;

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

describe('swiss_system_stage_update', async () => {
  it('update stage', async () => {
    let unauthorized = anchor.web3.Keypair.generate();

    let signature = await provider.connection.requestAirdrop(
      unauthorized.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1,
    );

    await provider.connection.confirmTransaction(signature);

    await program.methods
      .swissSystemStageUpdate({
        competitionIndex: new anchor.BN(state.competitionIndex),
      })
      .accounts({
        caller: unauthorized.publicKey,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
      })
      .signers([unauthorized])
      .rpc();

    let swissSystemAccount = await program.account.swissSystem.fetch(state.swissSystemPda);

    expect(swissSystemAccount.stage.registrationPeriod).to.not.be.undefined;

    await sleep(6000);

    await program.methods
      .swissSystemStageUpdate({
        competitionIndex: new anchor.BN(state.competitionIndex),
      })
      .accounts({
        caller: unauthorized.publicKey,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
      })
      .signers([unauthorized])
      .rpc();

      swissSystemAccount = await program.account.swissSystem.fetch(state.swissSystemPda);
      expect(swissSystemAccount.stage.competitionPeriod).to.not.be.undefined;
  });
});