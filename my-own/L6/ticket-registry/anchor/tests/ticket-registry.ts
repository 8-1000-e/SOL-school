import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { assert } from "chai";
import { TicketRegistry } from "../target/types/ticket_registry";

describe("ticket-registry", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ticketRegistry as Program<TicketRegistry>;

  const organizer = Keypair.generate();
  const buyer = Keypair.generate();

  const eventName = "SolanaConf";
  const eventDesc = "A cool Solana event";
  const ticketPrice = new anchor.BN(0.5 * LAMPORTS_PER_SOL);
  const availableTickets = new anchor.BN(100);
  // 1 hour from now
  const startDate = new anchor.BN(Math.floor(Date.now() / 1000) + 3600);

  // Derive Event PDA
  const [eventPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("event"), Buffer.from(eventName), organizer.publicKey.toBuffer()],
    program.programId
  );

  // Derive Ticket PDA
  const [ticketPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("ticket"), eventPDA.toBuffer(), buyer.publicKey.toBuffer()],
    program.programId
  );

  before(async () => {
    // Airdrop SOL to organizer and buyer
    const orgAirdrop = await provider.connection.requestAirdrop(
      organizer.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(orgAirdrop);

    const buyerAirdrop = await provider.connection.requestAirdrop(
      buyer.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(buyerAirdrop);
  });

  it("Initialize an event", async () => {
    await program.methods
      .initialize(eventName, eventDesc, ticketPrice, availableTickets, startDate)
      .accounts({
        eventOrganizer: organizer.publicKey,
      })
      .signers([organizer])
      .rpc();

    const eventAccount = await program.account.event.fetch(eventPDA);
    assert.equal(eventAccount.name, eventName);
    assert.equal(eventAccount.description, eventDesc);
    assert.equal(eventAccount.ticketCost.toNumber(), ticketPrice.toNumber());
    assert.equal(eventAccount.ticketAvailable.toNumber(), 100);
    assert.equal(eventAccount.owner.toBase58(), organizer.publicKey.toBase58());
    console.log("Event created:", eventAccount.name);
  });

  it("Buy a ticket", async () => {
    const eventBefore = await program.account.event.fetch(eventPDA);
    const ticketsBefore = eventBefore.ticketAvailable.toNumber();

    await program.methods
      .buy()
      .accounts({
        ticketPayer: buyer.publicKey,
        event: eventPDA,
      })
      .signers([buyer])
      .rpc();

    const eventAfter = await program.account.event.fetch(eventPDA);
    assert.equal(eventAfter.ticketAvailable.toNumber(), ticketsBefore - 1);

    const ticketAccount = await program.account.ticket.fetch(ticketPDA);
    assert.equal(ticketAccount.owner.toBase58(), buyer.publicKey.toBase58());
    assert.equal(ticketAccount.event.toBase58(), eventPDA.toBase58());
    assert.equal(ticketAccount.price.toNumber(), ticketPrice.toNumber());
    console.log("Ticket bought! Remaining:", eventAfter.ticketAvailable.toNumber());
  });

  it("Withdraw funds", async () => {
    const organizerBalanceBefore = await provider.connection.getBalance(organizer.publicKey);
    const withdrawAmount = new anchor.BN(0.25 * LAMPORTS_PER_SOL);

    await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        owner: organizer.publicKey,
        event: eventPDA,
      })
      .signers([organizer])
      .rpc();

    const organizerBalanceAfter = await provider.connection.getBalance(organizer.publicKey);
    assert.isAbove(organizerBalanceAfter, organizerBalanceBefore);
    console.log(
      "Withdrawn:",
      withdrawAmount.toNumber() / LAMPORTS_PER_SOL,
      "SOL"
    );
  });
});
