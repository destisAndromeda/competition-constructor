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
  it('error: should fail with same authority and creatorKey', async () => {
    let signature = await provider.connection.requestAirdrop(
      state.organizerSwissSystem.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1,
    );

    await provider.connection.confirmTransaction(signature);

    await expect(
      program.methods
        .swissSystemCreate({
          stageInfo: state.stageInfo,
          creatorKey: state.creatorKeySwissSystem.publicKey,
          authority: state.creatorKeySwissSystem.publicKey,
        })
        .accounts({
          organizer: state.organizerSwissSystem.publicKey,
          swissSystem: state.swissSystemPda,
          constructor: state.constructorPda,
          programConfig: state.programConfigPda,
          systemProgram: systemProgram,
        })
        .signers([state.organizerSwissSystem])
        .rpc()
    ).to.be.rejected.then((err: any) => {
      expect(err.error.errorCode.code).to.equals('SameAccounts');
    });
  });

  it('error: should fail with same organizer and creatorKey', async () => {
    await expect(
      program.methods
        .swissSystemCreate({
          stageInfo: state.stageInfo,
          creatorKey: state.organizerSwissSystem.publicKey,
          authority: state.creatorKeySwissSystem.publicKey,
        })
        .accounts({
          organizer: state.organizerSwissSystem.publicKey,
          swissSystem: state.swissSystemPda,
          constructor: state.constructorPda,
          programConfig: state.programConfigPda,
          systemProgram: systemProgram,
        })
        .signers([state.organizerSwissSystem])
        .rpc()
    ).to.be.rejected.then((err: any) => {
      expect(err.error.errorCode.code).to.equals('SameAccounts');
    });
  });

  it('error: should fail with same authority and organizer', async () => {
    await expect(
      program.methods
        .swissSystemCreate({
          stageInfo: state.stageInfo,
          creatorKey: state.organizerSwissSystem.publicKey,
          authority: state.organizerSwissSystem.publicKey,
        })
        .accounts({
          organizer: state.organizerSwissSystem.publicKey,
          swissSystem: state.swissSystemPda,
          constructor: state.constructorPda,
          programConfig: state.programConfigPda,
          systemProgram: systemProgram,
        })
        .signers([state.organizerSwissSystem])
        .rpc()
    ).to.be.rejected.then((err: any) => {
      expect(err.error.errorCode.code).to.equals('SameAccounts');
    });
  });

  it('create swissSystem account', async() => {
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

  it('error: should file when initialize swissSystem twice', async () => {
    await expect(
      program.methods
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
        .rpc()
    ).to.be.rejected.then((err) => {
      expect(err.error.errorCode.code).to.equals('ConstraintSeeds');
    });
  });
});

describe('swiss_system_update', () => {
  it('error: should fail with unauthorized organizer', async () => {
    let unauthorized = anchor.web3.Keypair.generate();

    let signature = await provider.connection.requestAirdrop(
      unauthorized.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1,
    );

    await provider.connection.confirmTransaction(signature);

    let newAuthority = anchor.web3.Keypair.generate();

    await expect(
      program.methods
        .swissSystemAuthorityUpdate({
          competitionIndex: new anchor.BN(state.competitionIndex),
          account: newAuthority.publicKey,
        })
        .accounts({
          organizer: unauthorized.publicKey,
          swissSystem: state.swissSystemPda,
          constructor: state.constructorPda,
          programConfig: state.programConfigPda,
        })
        .signers([unauthorized])
        .rpc()
    ).to.be.rejected.then((err) => {
      expect(err.error.errorCode.code).to.equals('Unauthorized');
    });
  });

  it('update authority', async () => {
    let newAuthority = anchor.web3.Keypair.generate();
    let prevAccount = await program.account.swissSystem.fetch(state.swissSystemPda);
    let prevAuthority = prevAccount.authority;

    await program.methods
      .swissSystemAuthorityUpdate({
        competitionIndex: new anchor.BN(state.competitionIndex),
        account: newAuthority.publicKey,
      })
      .accounts({
        organizer: state.organizerSwissSystem.publicKey,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
      })
      .signers([state.organizerSwissSystem])
      .rpc();

    const swissSystemAccount = await program.account.swissSystem.fetch(state.swissSystemPda); 

    expect(swissSystemAccount.authority.equals(newAuthority.publicKey)).to.be.true;

    await program.methods
      .swissSystemAuthorityUpdate({
        competitionIndex: new anchor.BN(state.competitionIndex),
        account: prevAuthority,
      })
      .accounts({
        organizer: state.organizerSwissSystem.publicKey,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
      })
      .signers([state.organizerSwissSystem])
      .rpc();
  });

  it('update creatorKey', async () => {
    let newCreatorKey = anchor.web3.Keypair.generate();
    let prevAccount = await program.account.swissSystem.fetch(state.swissSystemPda);
    let prevCreatorKey = prevAccount.creatorKey;

    await program.methods
      .swissSystemCreatorKeyUpdate({
        competitionIndex: new anchor.BN(state.competitionIndex),
        account: newCreatorKey.publicKey,
      })
      .accounts({
        organizer: state.organizerSwissSystem.publicKey,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
      })
      .signers([state.organizerSwissSystem])
      .rpc();

    let swissSystemAccount = await program.account.swissSystem.fetch(state.swissSystemPda); 
    expect(swissSystemAccount.creatorKey.equals(newCreatorKey.publicKey)).to.be.true;

    await program.methods
      .swissSystemCreatorKeyUpdate({
        competitionIndex: new anchor.BN(state.competitionIndex),
        account: prevCreatorKey,
      })
      .accounts({
        organizer: state.organizerSwissSystem.publicKey,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
      })
      .signers([state.organizerSwissSystem])
      .rpc();

    swissSystemAccount = await program.account.swissSystem.fetch(state.swissSystemPda);
    expect(swissSystemAccount.creatorKey.equals(newCreatorKey.publicKey)).to.be.false;
  });
});
