import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloWorld } from "../target/types/hello_world";

describe("hello-world", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.helloWorld as Program<HelloWorld>;

  it("Is initialized!", async () => {
    const hwAccount = anchor.web3.Keypair.generate(); // ← Keypair séparé !

    await program.methods
      .initialize()
      .accounts({
        payer: provider.wallet.publicKey,
        hwAcc: hwAccount.publicKey,        // ← utilise le nouveau keypair
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([hwAccount])                // ← doit signer car on crée ce compte
      .rpc();
  });

});
