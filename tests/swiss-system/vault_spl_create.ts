import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { CompetitionConstructorProgram } from '../../target/types/competition_constructor_program';
import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { state } from '../shared.ts';
import {
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from '@solana/spl-token';

const { expect } = chai;

chai.use(chaiAsPromised);

const SEED_PREFIX = 'competition_constructor';
const SEED_VAULT = 'vault';

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.CompetitionConstructorProgram as Program<CompetitionConstructorProgram>;
const systemProgram = anchor.web3.SystemProgram.programId;

describe('swiss_system_vault_create', () => {
  it('vault spl create', async () => {
    const connection = provider.connection;
    const payer = (provider.wallet as anchor.Wallet).payer;
    const mintAuthority = anchor.web3.Keypair.generate();
    const freezeAuthority = null;
    const decimals = 6;

    const mint = await createMint(
        connection,
        payer,
        mintAuthority.publicKey,
        freezeAuthority,
        decimals,
    );

    const organizerAta = await createAssociatedTokenAccount(
      provider.connection,
      payer,
      mint,
      state.organizerSwissSystem.publicKey,
    );

    const prize = 1_000_000;
    await mintTo(
      provider.connection,
      payer,
      mint,
      organizerAta,
      mintAuthority,
      prize,
    );

    const swissSystemAccount = await program.account.swissSystem.fetch(state.swissSystemPda)
    let vaultIndexBuffer = Buffer.alloc(8);
    vaultIndexBuffer.writeBigUint64LE(BigInt(swissSystemAccount.vaultIndex.toNumber()));
  
    const [vaultPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from(SEED_PREFIX),
        swissSystemAccount.creatorKey.toBuffer(),
        Buffer.from(SEED_VAULT),
        vaultIndexBuffer,
      ],
      program.programId,
    );

    const vaultAta = getAssociatedTokenAddressSync(
      mint,
      vaultPda,
      true,
    );
  
    await program.methods
      .swissSystemVaultSplCreate({
        competitionIndex: new anchor.BN(state.competitionIndex),
        asset: mint,
        prize: new anchor.BN(prize),
      })
      .accounts({
        organizer: state.organizerSwissSystem.publicKey,
        vault: vaultPda,
        swissSystem: state.swissSystemPda,
        constructor: state.constructorPda,
        programConfig: state.programConfigPda,
        mint: mint,
        organizerAta: organizerAta,
        vaultAta: vaultAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: systemProgram,
      })
      .signers([state.organizerSwissSystem])
      .rpc();

    const vaultAtaBalance = await provider.connection.getTokenAccountBalance(vaultAta);
    expect(vaultAtaBalance.value.amount).to.equal(prize.toString());

    const organizerAtaBalance = await provider.connection.getTokenAccountBalance(organizerAta);
    expect(organizerAtaBalance.value.amount).to.equal('0');
  });
});