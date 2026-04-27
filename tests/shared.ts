import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair, PublicKey } from '@solana/web3.js';
import { CompetitionConstructorProgram } from '../target/types/competition_constructor_program';

const SEED_PREFIX = 'competition_constructor';
const SEED_PROGRAM_CONFIG = 'program_config';
const SEED_CONSTRUCTOR = 'constructor';
const SEED_COMPETITION = 'competition';
const SEED_VAULT = 'vault';
const SEED_LEADER_BOARD = 'leaderboard';
const SEED_PARTICIPANT = 'participant';

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

let competitionIndex = 0;

let competitionIndexBuffer = Buffer.alloc(8);
competitionIndexBuffer.writeBigUint64LE(BigInt(competitionIndex));

let [swissSystemPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from(SEED_PREFIX),
    creatorKeyConstructor.publicKey.toBuffer(),
    Buffer.from(SEED_COMPETITION),
    competitionIndexBuffer,
  ],
  program.programId,
);

let organizerSwissSystem = anchor.web3.Keypair.generate();
let creatorKeySwissSystem = anchor.web3.Keypair.generate();
let authoritySwissSystem = anchor.web3.Keypair.generate();


const now = Math.floor(Date.now() / 1000);

let stageInfo = {
  registrationPeriod: new anchor.BN(now - 10),
  competitionPeriod: new anchor.BN(now + 15),
  withdrawPeriod: new anchor.BN(now + 20),
};

let vaultIndex = 0;

let vaultIndexBuffer = Buffer.alloc(8);
vaultIndexBuffer.writeBigUint64LE(BigInt(vaultIndex));

let [vaultPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from(SEED_PREFIX),
    creatorKeySwissSystem.publicKey.toBuffer(),
    Buffer.from(SEED_VAULT),
    vaultIndexBuffer,
  ],
  program.programId,
);

let [leaderboardPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from(SEED_PREFIX),
    creatorKeySwissSystem.publicKey.toBuffer(),
    Buffer.from(SEED_LEADER_BOARD),
    organizerSwissSystem.publicKey.toBuffer(),
  ],
  program.programId,
);

let participantOne = anchor.web3.Keypair.generate();
// let participantTwo = anchor.web3.Keypair.generate();

let [participantOnePda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from(SEED_PREFIX),
    creatorKeySwissSystem.publicKey.toBuffer(),
    Buffer.from(SEED_PARTICIPANT),
    participantOne.publicKey.toBuffer(),
  ],
  program.programId,
);

// let [participantTwoPda] = PublicKey.findProgramAddressSync(
//   [
//     Buffer.from(SEED_PREFIX),
//     creatorKeySwissSystem.publicKey.toBuffer(),
//     Buffer.from(SEED_PARTICIPANT),
//     participantTwo.publicKey.toBuffer(),
//   ],
//   program.programId,
// );

export const state: {
  programConfigPda: PublicKey;
  creatorKeyConfig: Keypair;
  treasury: Keypair;

  constructorPda: PublicKey,
  creatorKeyConstructor: Keypair,
  authorityConstructor: Keypair,

  swissSystemPda: PublicKey,
  competitionIndex: number,
  organizerSwissSystem: Keypair,
  creatorKeySwissSystem: Keypair,
  authoritySwissSystem: Keypair,
  stageInfo: {
    registrationPeriod: anchor.BN,
    competitionPeriod: anchor.BN,
    withdrawPeriod: anchor.BN,
  },

  vaultPda: PublicKey,
  vaultIndex: number,

  leaderboardPda: PublicKey,

  participantOne: Keypair,
  participantOnePda: PublicKey,
} = {
  programConfigPda: programConfigPda,
  creatorKeyConfig: creatorKeyConfig,
  treasury: treasury, 

  constructorPda: constructorPda,
  creatorKeyConstructor: creatorKeyConstructor,
  authorityConstructor: authorityConstructor,

  swissSystemPda: swissSystemPda,
  competitionIndex: competitionIndex,
  organizerSwissSystem: organizerSwissSystem,
  creatorKeySwissSystem: creatorKeySwissSystem,
  authoritySwissSystem: authoritySwissSystem,
  stageInfo: stageInfo,

  vaultPda: vaultPda,
  vaultIndex: vaultIndex,

  leaderboardPda: leaderboardPda,

  participantOne: participantOne,
  participantOnePda: participantOnePda,
};