import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorNftProj } from "../target/types/anchor_nft_proj";

describe("anchor-nft-proj", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorNftProj as Program<AnchorNftProj>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initUser("Soldi", "https://").rpc();
    console.log("Your transaction signature", tx);
  });
});
