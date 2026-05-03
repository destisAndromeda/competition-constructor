import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { CompetitionConstructorProgram } from '../target/types/competition_constructor_program';
import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { 
  state,
} from './shared.ts';

const { expect } = chai;

chai.use(chaiAsPromised);

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.CompetitionConstructorProgram as Program<CompetitionConstructorProgram>;
const systemProgram = anchor.web3.SystemProgram.programId;

after(async () => {
  try {
    const configAccount = await program.account.programConfig.fetch(state.programConfigPda);
    expect(configAccount.state.treasury.equals(configAccount.state.creatorKeyConfig)).to.be.false;
  } catch(e) {
    // skip
  }
});

describe('prgoram_config_init', () => {
  it('error: should fail with default authority', async () => {
    const defaultKey = PublicKey.default;

    await expect(
      program.methods
        .programConfigInit({
          creatorKey: defaultKey,
          treasury: defaultKey,
        })
        .accounts({
          authority: provider.wallet.publicKey,
          programConfig: state.programConfigPda,
        })
        .rpc()
    ).to.be.rejected.then((err: any) => {
      expect(err.error.errorCode.code).to.equal('InvalidAccount');
    });
  });

  it('error: should fail with unauthorized key', async () => {
    const unauthorized = anchor.web3.Keypair.generate();

    const signature = await provider.connection.requestAirdrop(
      unauthorized.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1,
    );

    await provider.connection.confirmTransaction(signature);
    // const balance = await provider.connection.getBalance(unauthorized.publicKey);

    await expect(
      program.methods
        .programConfigInit({
          creatorKey: state.creatorKeyConfig.publicKey,
          treasury: state.treasury.publicKey,
        })
        .accounts({
          authority: unauthorized.publicKey,
          programConfig: state.programConfigPda,
          systemProgram: systemProgram,
        })
        .signers([unauthorized])
        .rpc()
    ).to.be.rejected.then((err: any) => {
      expect(err.error.errorCode.code).to.equal('Unauthorized');
    });
  });

  it('error: should fail with equals creatorKey and treasury', async () => {
    const same = anchor.web3.Keypair.generate();

    await expect(
      program.methods
        .programConfigInit({
          creatorKey: same.publicKey,
          treasury: same.publicKey,
        })
        .accounts({
          authority: provider.wallet.publicKey,
          programConfig: state.programConfigPda,
          systemProgram: systemProgram,
        })
        .rpc()
    ).to.be.rejected.then((err: any) => {
      expect(err.error.errorCode.code).to.equal('SameAccounts');
    });
  });

  it('initialize', async () => {  
    await program.methods
      .programConfigInit({
          creatorKey: state.creatorKeyConfig.publicKey,
          treasury: state.treasury.publicKey,
        })
      .accounts({
        authority: provider.wallet.publicKey,
        programConfig: state.programConfigPda,
        systemProgram: systemProgram,
      })
      .rpc();

    const configAccount = await program.account.programConfig.fetch(state.programConfigPda);

    expect(configAccount.authority.equals(provider.wallet.publicKey)).to.be.true;
    expect(configAccount.creatorKey.equals(state.creatorKeyConfig.publicKey)).to.be.true;
    expect(configAccount.treasury.equals(state.treasury.publicKey)).to.be.true;
  });

  it('error: can not initialize program config twice ', async () => {
    await expect(
      program.methods
        .programConfigInit({
          creatorKey: state.creatorKeyConfig.publicKey,
          treasury: state.treasury.publicKey,
        })
        .accounts({
          authority: provider.wallet.publicKey,
          programConfig: state.programConfigPda,
          systemProgram: systemProgram,
        })
        .rpc()
    ).to.be.rejectedWith(/already in use/i);
  });

  it('creator_key and treasury are not equal', async () => {
    const configAccount = await program.account.programConfig.fetch(state.programConfigPda);
    expect(
      configAccount.creatorKey.equals(configAccount.treasury)
    ).to.be.false;
  });
});

describe('program_config_update', () => {
  it('error: should fail with unauthorized key', async () => {
    const unauthorized = anchor.web3.Keypair.generate();

    const signature = await provider.connection.requestAirdrop(
      unauthorized.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1,
    );

    await provider.connection.confirmTransaction(signature);

    await expect(
      program.methods
        .programConfigAuthorityUpdate({
          account: unauthorized.publicKey,
        })
        .accounts({
          authority: unauthorized.publicKey,
          programConfig: state.programConfigPda,
        })
        .signers([unauthorized])
        .rpc()
    ).to.be.rejected.then((err: any) => {
      expect(err.error.errorCode.code).to.equal('Unauthorized');
    });
  });

  it('authority update', async () => {
    const prevAuthority = (await program.account.programConfig.fetch(state.programConfigPda)).authority;
    const newAuthority = anchor.web3.Keypair.generate();

    const signature = await provider.connection.requestAirdrop(
      newAuthority.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1,
    );

    await provider.connection.confirmTransaction(signature);
    // const balance = await provider.connection.getBalance(newAuthority.publicKey);

    await program.methods
      .programConfigAuthorityUpdate({
        account: newAuthority.publicKey,
      })
      .accounts({
        authority: provider.wallet.publicKey,
        programConfig: state.programConfigPda,
      })
      .rpc();

      const configAccount = await program.account.programConfig.fetch(state.programConfigPda);

      expect(
        configAccount.authority.equals(prevAuthority)
      ).to.be.false;

      await program.methods
      .programConfigAuthorityUpdate({
        account: provider.wallet.publicKey,
      })
      .accounts({
        authority: newAuthority.publicKey,
        programConfig: state.programConfigPda,
      })
      .signers([newAuthority])
      .rpc();
  });

  it('error: should fail with previous authority', async () => {
    // const configAccount = await program.account.programConfig.fetch(state.programConfigPda); 
    // console.log('program config authority: ', configAccount.authority);

    await expect(
      program.methods
        .programConfigAuthorityUpdate({
          account: provider.wallet.publicKey,
        })
        .accounts({
          authority: provider.wallet.publicKey,
          programConfig: state.programConfigPda,
        })
        .rpc()
      ).to.be.rejected.then((err: any) => {
        expect(err.error.errorCode.code).to.equal('DeprecatedAddress');
      });
  });

  it('creatorKey update', async () => {
    const prevCreatorKey = (await program.account.programConfig.fetch(state.programConfigPda)).creatorKey;
    const newCreatorKey = anchor.web3.Keypair.generate();

    await program.methods
      .programConfigCreatorKeyUpdate({
        account: newCreatorKey.publicKey,
      })
      .accounts({
        authority: provider.wallet.publicKey,
        programConfig: state.programConfigPda,
      })
      .rpc();

      const configAccount = await program.account.programConfig.fetch(state.programConfigPda);

      expect(
        configAccount.creatorKey.equals(prevCreatorKey)
      ).to.be.false;

      await program.methods
        .programConfigCreatorKeyUpdate({
          account: prevCreatorKey,
        })
        .accounts({
          authority: provider.wallet.publicKey,
          programConfig: state.programConfigPda,
        })
        .rpc();
  });

  it('error: should fail with previous creatorKeyConfig', async () => {
    const prevCreatorKey = (await program.account.programConfig.fetch(state.programConfigPda)).creatorKey;

    await expect(
      program.methods
        .programConfigCreatorKeyUpdate({
          account: prevCreatorKey,
        })
        .accounts({
          authority: provider.wallet.publicKey,
          programConfig: state.programConfigPda,
        })
        .rpc()
    ).to.be.rejected.then((err: any) => {
      expect(err.error.errorCode.code).to.equal('DeprecatedAddress');
    });
  });

  it('treasury update', async () => {
    const prevTreasury = (await program.account.programConfig.fetch(state.programConfigPda)).treasury;
    const newTreasury = anchor.web3.Keypair.generate();

    await program.methods
      .programConfigTreasuryUpdate({
        account: newTreasury.publicKey,
      })
      .accounts({
        authority: provider.wallet.publicKey,
        programConfig: state.programConfigPda,
      })
      .rpc();

      const configAccount = await program.account.programConfig.fetch(state.programConfigPda);

      expect(
        configAccount.treasury.equals(prevTreasury)
      ).to.be.false;
  });

  it('error: should fail with previous treasury', async () => {
    const prevTreasury = (await program.account.programConfig.fetch(state.programConfigPda)).treasury;

    await expect(
      program.methods
        .programConfigTreasuryUpdate({
          account: prevTreasury,
        })
        .accounts({
          authority: provider.wallet.publicKey,
          programConfig: state.programConfigPda,
        })
        .rpc()
    ).to.be.rejected.then((err: any) => {
      expect(err.error.errorCode.code).to.equal('DeprecatedAddress');
    });
  });
});
