import {
  beforeEach,
  describe,
  expect,
  it,
} from 'vitest';
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { homedir } from "node:os";
import { join } from "node:path";
import { PublicKey } from '@solana/web3.js';
import { CompetitionConstructorProgram } from '../../target/types/competition_constructor_program';

process.env.ANCHOR_PROVIDER_URL ??= "http://127.0.0.1:8899";
process.env.ANCHOR_WALLET ??= join(homedir(), ".config/solana/id.json");

const SEED_PREFIX = "competition_constructor";
const SEED_PROGRAM_CONFIG = "program_config";

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.CompetitionConstructorProgram as Program<CompetitionConstructorProgram>;
const systemProgram = anchor.web3.SystemProgram.programId;

let programConfigPda: any;
let creatorKey: any;
let treasury: any;

beforeEach(() => {  
  programConfigPda = PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_PREFIX), Buffer.from(SEED_PROGRAM_CONFIG)],
    program.programId,
  );

  creatorKey = anchor.web3.Keypair.generate();
  treasury = anchor.web3.Keypair.generate();
});

describe('prgoram_config_init tests', () => {
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
          creatorKey: creatorKey.publicKey,
          treasury: treasury.publicKey,
        })
        .accounts({
          authority: unauthorized.publicKey,
          programConfig: programConfigPda[0],
          systemProgram: systemProgram,
        })
        .signers([unauthorized])
        .rpc()
    ).rejects.toMatchObject({
      error: {
        errorCode:
        {
          code: 'Unauthorized'
        }
      }
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
          programConfig: programConfigPda[0],
          systemProgram: systemProgram,
        })
        .rpc()
    ).rejects.toMatchObject({
      error: {
        errorCode: {
          code: 'InvalidCreatorKeyAndTreasury'
        }
      }
    });
  });

  it('initialize', async () => {  
      await program.methods
        .programConfigInit({
            creatorKey: creatorKey.publicKey,
            treasury: treasury.publicKey,
          })
        .accounts({
          authority: provider.wallet.publicKey,
          programConfig: programConfigPda[0],
          systemProgram: systemProgram,
        })
        .rpc();

      const configAccount = await program.account.programConfig.fetch(programConfigPda[0]);
  
      expect(
        configAccount.authority.equals(provider.wallet.publicKey)
      ).toBe(true);

      expect(
        configAccount.creatorKey.equals(creatorKey.publicKey)
      ).toBe(true);

      expect(
        configAccount.treasury.equals(treasury.publicKey)
      ).toBe(true);
  });

  it('error: can not initialize program config twice ', async () => {
    await expect(
      program.methods
        .programConfigInit({
          creatorKey: creatorKey,
          treasury: treasury,
        })
        .accounts({
          authority: provider.wallet.publicKey,
          programConfig: programConfigPda,
          systemProgram: systemProgram,
        })
        .rpc()
    ).rejects.toThrow();
  });

  it('creator_key and treasury are not equal', async () => {
    const configAccount = await program.account.programConfig.fetch(programConfigPda[0]);
    expect(
      configAccount.creatorKey.equals(configAccount.treasury)
    ).not.toBe(true);
  });
});

describe('program_config_update tests', () => {
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
        .programConfigUpdateAuthority({
          authority: unauthorized.publicKey,
        })
        .accounts({
          authority: unauthorized.publicKey,
          programConfig: programConfigPda,
        })
        .signers([unauthorized])
        .rpc()
    ).rejects.toMatchObject({
      error: {
        errorCode:
        {
          code: 'Unauthorized'
        }
      }
    });
  });

  it('authority update', async () => {
    const prevAuthority = (await program.account.programConfig.fetch(programConfigPda[0])).authority;
    const newAuthority = anchor.web3.Keypair.generate();

    const signature = await provider.connection.requestAirdrop(
      newAuthority.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1,
    );

    await provider.connection.confirmTransaction(signature);
    // const balance = await provider.connection.getBalance(newAuthority.publicKey);

    await program.methods
      .programConfigUpdateAuthority({
        authority: newAuthority.publicKey,
      })
      .accounts({
        authority: provider.wallet.publicKey,
        programConfig: programConfigPda[0],
      })
      .rpc();

      const configAccount = await program.account.programConfig.fetch(programConfigPda[0]);

      expect(
        configAccount.authority.equals(prevAuthority)
      ).not.toBe(true);

      await program.methods
      .programConfigUpdateAuthority({
        authority: provider.wallet.publicKey,
      })
      .accounts({
        authority: newAuthority.publicKey,
        programConfig: programConfigPda[0],
      })
      .signers([newAuthority])
      .rpc();
  });
  
    it('error: should fail with previous authority', async () => {
      // const configAccount = await program.account.programConfig.fetch(programConfigPda[0]); 
      // console.log('program config authority: ', configAccount.authority);
  
      await expect(
        program.methods
          .programConfigUpdateAuthority({
            authority: provider.wallet.publicKey,
          })
          .accounts({
            authority: provider.wallet.publicKey,
            programConfig: programConfigPda[0],
          })
          .rpc()
        ).rejects.toMatchObject({
          error: {
            errorCode: {
              code: 'DeprecatedAddress',
            }
          }
        });
    });

    it('creatorKey update', async () => {
      const prevCreatorKey = (await program.account.programConfig.fetch(programConfigPda[0])).creatorKey;
      const newCreatorKey = anchor.web3.Keypair.generate();

      await program.methods
        .programConfigUpdateCreatorKey({
          creatorKey: newCreatorKey.publicKey,
        })
        .accounts({
          authority: provider.wallet.publicKey,
          programConfig: programConfigPda,
        })
        .rpc();

        const configAccount = await program.account.programConfig.fetch(programConfigPda[0]);

        expect(
          configAccount.creatorKey.equals(prevCreatorKey)
        ).not.toBe(true);
    });

  it('error: should fail with previous creatorKey', async () => {
    const prevCreatorKey = (await program.account.programConfig.fetch(programConfigPda[0])).creatorKey;

    await expect(
      program.methods
        .programConfigUpdateCreatorKey({
          creatorKey: prevCreatorKey,
        })
        .accounts({
          authority: provider.wallet.publicKey,
          programConfig: programConfigPda[0],
        })
        .rpc()
    ).rejects.toMatchObject({
      error: {
        errorCode: {
          code: 'DeprecatedAddress',
        }
      }
    });
  });

  it('treasury update', async () => {
    const prevTreasury = (await program.account.programConfig.fetch(programConfigPda[0])).treasury;
    const newTreasury = anchor.web3.Keypair.generate();

    await program.methods
      .programConfigUpdateTreasury({
        treasury: newTreasury.publicKey,
      })
      .accounts({
        authority: provider.wallet.publicKey,
        programConfig: programConfigPda,
      })
      .rpc();

      const configAccount = await program.account.programConfig.fetch(programConfigPda[0]);

      expect(
        configAccount.treasury.equals(prevTreasury)
      ).not.toBe(true);
  });

  it('error: should fail with previous treasury', async () => {
    const prevTreasury = (await program.account.programConfig.fetch(programConfigPda[0])).treasury;

    await expect(
      program.methods
        .programConfigUpdateTreasury({
          treasury: prevTreasury,
        })
        .accounts({
          authority: provider.wallet.publicKey,
          programConfig: programConfigPda[0],
        })
        .rpc()
    ).rejects.toMatchObject({
      error: {
        errorCode: {
          code: 'DeprecatedAddress',
        }
      }
    });
  });
});