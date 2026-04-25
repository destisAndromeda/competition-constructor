import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair, PublicKey } from '@solana/web3.js';
import { CompetitionConstructorProgram } from '../target/types/competition_constructor_program';

const SEED_PREFIX = 'competition_constructor';
const SEED_PROGRAM_CONFIG = 'program_config';
const SEED_CONSTRUCTOR = 'constructor';

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.CompetitionConstructorProgram as Program<CompetitionConstructorProgram>;

let [programConfigPda] = PublicKey.findProgramAddressSync(
  [Buffer.from(SEED_PREFIX), Buffer.from(SEED_PROGRAM_CONFIG)],
  program.programId,
);

let creatorKeyConfig = anchor.web3.Keypair.generate();
let treasury = anchor.web3.Keypair.generate();

let creatorKeyConstructor = anchor.web3.Keypair.generate();

let [constructorPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from(SEED_PREFIX),
    programConfigPda.toBuffer(),
    Buffer.from(SEED_CONSTRUCTOR),
    creatorKeyConfig.publicKey.toBuffer()
  ],
  program.programId,
);

let authorityConstructor = anchor.web3.Keypair.generate();

export const state: {
  programConfigPda: PublicKey;
  creatorKeyConfig: Keypair;
  treasury: Keypair;

  constructorPda: PublicKey,
  creatorKeyConstructor: Keypair,
  authorityConstructor: Keypair,
} = {
  programConfigPda: programConfigPda,
  creatorKeyConfig: creatorKeyConfig,
  treasury: treasury, 

  constructorPda: constructorPda,
  creatorKeyConstructor: creatorKeyConstructor,
  authorityConstructor: authorityConstructor,
};