import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NftGiftcard } from "../target/types/nft_giftcard";

describe("nft-giftcard", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NftGiftcard as Program<NftGiftcard>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
