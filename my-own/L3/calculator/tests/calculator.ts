import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const calKeypair = anchor.web3.Keypair.generate();
  const program = anchor.workspace.calculator as Program<Calculator>;

  async function readCalc() {
    const account = await program.account.calculator.fetch(calKeypair.publicKey);
    console.log("  → greetings:", account.greetings);
    console.log("  → result:", account.result.toString());
    return account;
  }


  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize()
    .accounts({
      payer: provider.wallet.publicKey,
      calcAcc: calKeypair.publicKey,
      systemProgram: SYSTEM_PROGRAM_ID
    })
    .signers([calKeypair])
    .rpc();

    await readCalc();
  });

  it("Add 10 + 45", async () => { 
    await program.methods.add(new anchor.BN(10), new anchor.BN(45)).accounts({
      calcAcc: calKeypair.publicKey,
    })
    .rpc();

    await readCalc();

  });

    it("sub 10 - 45", async () => { 
    await program.methods.sub(new anchor.BN(10), new anchor.BN(45)).accounts({
      calcAcc: calKeypair.publicKey,
    })
    .rpc();

    await readCalc();

  });

    it("multiply 10 * 45", async () => { 
    await program.methods.multiply(new anchor.BN(10), new anchor.BN(45)).accounts({
      calcAcc: calKeypair.publicKey,
    })
    .rpc();

    await readCalc();

  });

    it("divide 10 / 45", async () => { 
    await program.methods.divide(new anchor.BN(10), new anchor.BN(45)).accounts({
      calcAcc: calKeypair.publicKey,
    })
    .rpc();

    await readCalc();

  });

    it("divide 10 / 0", async () => {
      
      try
      {
    await program.methods.divide(new anchor.BN(10), new anchor.BN(0)).accounts({
      calcAcc: calKeypair.publicKey,
    })
    .rpc();
      }

      catch (e)
      {
        console.log("Error caught: ", e.message);
      }
    await readCalc();

  });
});
