import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Bank } from "../target/types/bank";
import { expect } from "chai";

const BANK_ACCOUNT_SEED = "bank_account";

describe("bank", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Bank as Program<Bank>;
  const bankCreator = anchor.web3.Keypair.generate();
  const bankName = "School of Solana";

  // Helper : dériver le PDA du bank account
  function getBankAddress(creator: anchor.web3.PublicKey) {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(BANK_ACCOUNT_SEED),
        creator.toBuffer(),
      ],
      program.programId
    );
  }

  // Helper : airdrop des SOL à une adresse
  async function airdrop(address: anchor.web3.PublicKey, amount = 1_000_000_000) {
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(address, amount),
      "confirmed"
    );
  }

  it("Create Bank", async () => {
    // Airdrop au créateur pour qu'il puisse payer
    await airdrop(bankCreator.publicKey);

    const [bankPDA] = getBankAddress(bankCreator.publicKey);

    await program.methods
      .create(bankName)
      .accounts({
        payer: bankCreator.publicKey,
        bankAccount: bankPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([bankCreator])
      .rpc();

    const account = await program.account.bankAccount.fetch(bankPDA);
    expect(account.name).to.equal(bankName);
    expect(account.balance.toNumber()).to.equal(0);
    expect(account.owner).to.eql(bankCreator.publicKey);
    console.log("Bank created:", account.name, "owner:", account.owner.toBase58());
  });

  it("Deposit to Bank", async () => {
    const [bankPDA] = getBankAddress(bankCreator.publicKey);
    const depositAmount = new anchor.BN(500);

    await program.methods
      .deposit(depositAmount)
      .accounts({
        payer: bankCreator.publicKey,
        bankAccount: bankPDA,
      })
      .signers([bankCreator])
      .rpc();

    const account = await program.account.bankAccount.fetch(bankPDA);
    expect(account.balance.toNumber()).to.equal(500);
    console.log("Balance after deposit:", account.balance.toNumber());
  });

  it("Withdraw from Bank", async () => {
    const [bankPDA] = getBankAddress(bankCreator.publicKey);
    const withdrawAmount = new anchor.BN(258);

    const balanceBefore = (await program.account.bankAccount.fetch(bankPDA)).balance.toNumber();

    await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        payer: bankCreator.publicKey,
        bankAccount: bankPDA,
      })
      .signers([bankCreator])
      .rpc();

    const account = await program.account.bankAccount.fetch(bankPDA);
    expect(account.balance.toNumber()).to.equal(balanceBefore - 258);
    console.log("Balance after withdraw:", account.balance.toNumber());
  });
});
