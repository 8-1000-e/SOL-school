import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Puppet } from "../target/types/puppet";
import { PuppetMaster } from "../target/types/puppet_master";
import { assert } from "chai";

describe("puppet", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const puppetProgram = anchor.workspace.Puppet as Program<Puppet>;
  const puppetMasterProgram = anchor.workspace.PuppetMaster as Program<PuppetMaster>;
  const dataKeypair = anchor.web3.Keypair.generate();

  it("Initialize puppet", async () => 
    {
    // Créer le compte data via puppet.initialize

    const tx = await puppetProgram.methods.initialize()
    .accounts({
      payer: provider.wallet.publicKey,
      data: dataKeypair.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .signers([dataKeypair])
    .rpc();
    // → passer payer, data (keypair), systemProgram
  });

  it("CPI: pullStrings sets data to 42", async () => 
    {
      const tx = await puppetMasterProgram.methods.pullStrings(new anchor.BN(42))
      .accounts({
        data: dataKeypair.publicKey,
        puppetProgram: puppetProgram.programId
      })
      .rpc();
    // Appeler puppetMasterProgram.methods.pullStrings(new anchor.BN(42))
    // → passer data et puppetProgram comme comptes
    const account = await puppetProgram.account.data.fetch(dataKeypair.publicKey);
    console.log("value: ", account.value.toNumber());
    // → fetch et vérifier data.value == 42
  });
});
