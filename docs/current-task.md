# Current Task — L7 Security (next)

## Last Completed: L6 ticket-registry

### Programme Anchor complet
- **Location**: `my-own/L6/ticket-registry/anchor/programs/ticket-registry/src/`
- **3 instructions**: initialize, buy, withdraw
- **3 tests passing**: `my-own/L6/ticket-registry/anchor/tests/ticket-registry.ts`
- **Build**: OK (`anchor build` from `my-own/L6/ticket-registry/anchor/`)

### Frontend (skipped)
- Next.js boilerplate was partially set up at `my-own/L6/ticket-registry/src/`
- npm dependencies installed
- Missing: Codama client generation + 3 ticketregistry components (data-access, ui, feature)
- Student decided to skip frontend — too much boilerplate setup for learning value

### Key field name differences from temoin
Student's program uses different names than the temoin reference:
- `ticket_cost` (temoin: `ticket_price`)
- `ticket_available` (temoin: `available_tickets`)
- `owner` (temoin: `event_organizer`)
- `start_time` (temoin: `start_date`)
- `desc` param (temoin: `description`)
- `t_cost` param (temoin: `ticket_price`)
- `start_t` param (temoin: `start_date`)

## Next Up: L7 Security

### What is it
- Theory lesson — no code to write
- `temoin/7.lesson/README.md` — Runtime Policy summary
- Main resource: https://github.com/Ackee-Blockchain/solana-common-attack-vectors

### Solana Runtime Policy (from README)
1. Immutability: programs immutable when upgrade authority = null
2. Data: only owner can modify account data
3. Ownership: only owner can reassign ownership
4. Transaction: atomic, balances conserved
5. Data Allocation: only owner can resize, new data zeroed
6. Balance: only owner can subtract lamports, anyone can add

### What to do
- Read through the Ackee attack vectors repo
- Quiz the student on key concepts
- Update progress.md and concepts.md when done

## After L7: Pinocchio (main objective)
- Student's primary learning goal
- Zero experience — starting from scratch
- Check temoin/bonus/ for Pinocchio exercises
