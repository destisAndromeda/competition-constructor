import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { CompetitionConstructorProgram } from '../../target/types/competition_constructor_program';
import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { state } from '../shared.ts';

const { expect } = chai;

chai.use(chaiAsPromised);

const VAULT_SIZE = 1 + 8 + 8 + 33 + 33;

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.CompetitionConstructorProgram as Program<CompetitionConstructorProgram>;
const systemProgram = anchor.web3.SystemProgram.programId;

describe('swiss_system_vault_create', () => {
  it('vault create', async () => {
    const prize = new anchor.BN(2_000_000);
    
    const organizerBalanceBefore = await provider.connection.getBalance(state.organizerSwissSystem.publicKey);
  
    await program.methods
      .swissSystemVaultCreate({
        competitionIndex: new anchor.BN(state.competitionIndex),
        prize: prize,
      })
      .accounts({
        organizer: state.organizerSwissSystem.publicKey,
        vault: state.vaultPda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
        systemProgram: systemProgram,
      })
      .signers([state.organizerSwissSystem])
      .rpc();

    const vaultBalance = await provider.connection.getBalance(state.vaultPda);
    const rentExempt = await provider.connection.getMinimumBalanceForRentExemption(
      VAULT_SIZE,
    );

    expect(vaultBalance).to.equal(prize.toNumber() + rentExempt);

    const organizerBalanceAfter = await provider.connection.getBalance(state.organizerSwissSystem.publicKey);
    expect(organizerBalanceBefore - organizerBalanceAfter).to.be.greaterThan(prize.toNumber());
  
    const vaultAccount = await program.account.vault.fetch(state.vaultPda);
    expect(vaultAccount.winner).to.be.null;
    expect(vaultAccount.place.toNumber()).to.equal(0);
  });
});