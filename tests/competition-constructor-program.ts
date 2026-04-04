import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CompetitionConstructorProgram } from "../target/types/competition_constructor_program";

describe("competition-constructor-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.competitionConstructorProgram as Program<CompetitionConstructorProgram>;
});
