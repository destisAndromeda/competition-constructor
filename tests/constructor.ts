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

describe('constructor_create', () => {
  it('error: should fail with same authority and creatorKey', async () => {
    const same = anchor.web3.Keypair.generate();
    const fee  = 0;

    let signature = await provider.connection.requestAirdrop(
      state.creatorKeyConfig.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1,
    );

    await provider.connection.confirmTransaction(signature);

    signature = await provider.connection.requestAirdrop(
      same.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1,
    );

    await provider.connection.confirmTransaction(signature);

    await expect(
      program.methods
        .constructorCreate({
          creatorKey: same.publicKey,
          authority: same.publicKey,
          transactionFee: new anchor.BN(fee),
        })
        .accounts({
          creatorKey: state.creatorKeyConfig.publicKey,
          constructor: state.constructorPda,
          programConfig: state.programConfigPda,
          systemProgram: systemProgram,
        })
        .signers([state.creatorKeyConfig])
        .rpc()
    ).to.be.rejected.then((err: any) => {
      expect(err.error.errorCode.code).to.equals('SameAccounts');
    });
  });

  it('error: should fail with same creatorKey\'s', async () => {
    await expect(
      program.methods
        .constructorCreate({
          authority: state.authorityConstructor.publicKey,
          creatorKey: state.creatorKeyConfig.publicKey,
          transactionFee: new anchor.BN(0),
        })
        .accounts({
          creatorKey: state.creatorKeyConfig.publicKey,
          constructor: state.constructorPda,
          programConfig: state.programConfigPda,
          systemProgram: systemProgram,
        })
        .signers([state.creatorKeyConfig])
        .rpc()
    ).to.be.rejected.then((err) => {
      expect(err.error.errorCode.code).to.equals('InvalidAccount');
    });
	});

  it('constructor create', async () => {
		let fee = 0;

    await program.methods
			.constructorCreate({
				authority: state.authorityConstructor.publicKey,
				creatorKey: state.creatorKeyConstructor.publicKey,
				transactionFee: new anchor.BN(fee),
			})
			.accounts({
				creatorKey: state.creatorKeyConfig.publicKey,
				constructor: state.constructorPda,
				programConfig: state.programConfigPda,
				systemProgram: systemProgram,
			})
			.signers([state.creatorKeyConfig])
			.rpc();

			const constructor = await program.account.constructor.fetch(state.constructorPda);

			expect(constructor.authority.equals(state.authorityConstructor.publicKey)).to.be.true;
			expect(constructor.creatorKey.equals(state.creatorKeyConstructor.publicKey)).to.be.true;
			expect(constructor.transactionFee.eq(new anchor.BN(fee))).to.be.true;
	});

  it('error: can not initialize constructor twice', async() => {
    await expect(
      program.methods
        .constructorCreate({
          authority: state.authorityConstructor.publicKey,
          creatorKey: state.creatorKeyConstructor.publicKey,
          transactionFee: new anchor.BN(0)
        })
        .accounts({
          creatorKey: state.creatorKeyConfig.publicKey,
          constructor: state.constructorPda,
          programConfig: state.programConfigPda,
          systemProgram: systemProgram,
        })
        .signers([state.creatorKeyConfig])
        .rpc()
    ).to.be.rejectedWith(/already in use/i);
  });
});

describe('constructor_update', () => {
  it('authority update', async () => {
    let newAuthority = anchor.web3.Keypair.generate();
    let prevAuthority = await program.accounts.constructor.fetch(state.constructorPda).authority;


  });
});