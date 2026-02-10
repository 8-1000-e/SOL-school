import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { Game } from "../target/types/game";
import { expect } from "chai";

describe("game", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Game as Program<Game>;

  // On dérive le PDA côté client avec les mêmes seeds que dans le programme Rust
  const [userStatsPDA, _] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("user-stats"),
      provider.wallet.publicKey.toBuffer(),
    ],
    program.programId
  );

  it("Creates user stats", async () => {
    await program.methods
      .createUserStats("brian")
      .accounts({
        payer: provider.wallet.publicKey,
      })
      .rpc();

    const account = await program.account.userStats.fetch(userStatsPDA);
    expect(account.name).to.equal("brian");
    expect(account.level).to.equal(0);
    console.log("name:", account.name, "level:", account.level);
  });

  it("Changes user name", async () => {
    await program.methods
      .changeUserName("tom")
      .accounts({
        payer: provider.wallet.publicKey,
      })
      .rpc();

    const account = await program.account.userStats.fetch(userStatsPDA);
    expect(account.name).to.equal("tom");
    console.log("new name:", account.name);
  });
});
