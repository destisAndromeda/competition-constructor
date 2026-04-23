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
import { CompetitionConstructorProgram } from '../target/types/competition_constructor_program';

process.env.ANCHOR_PROVIDER_URL ??= "http://127.0.0.1:8899";
process.env.ANCHOR_WALLET ??= join(homedir(), ".config/solana/id.json");

const SEED_PREFIX = "competition_constructor";
const SEED_PROGRAM_CONFIG = "program_config";

describe('prgoram_config_init tests', () => {
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


  it('should fail with unauthorized key', async () => {
    const unauthorized = anchor.web3.Keypair.generate();
    
    const signature = await provider.connection.requestAirdrop(
      unauthorized.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1,
    );

    await provider.connection.confirmTransaction(signature);

    const balance = await provider.connection.getBalance(unauthorized.publicKey);
    console.log('balance: ', balance / anchor.web3.LAMPORTS_PER_SOL);

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
    ).rejects.toThrow();
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
});