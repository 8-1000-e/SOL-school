# Decisions Log

## 2026-02-09 — Skip L5 (Tests & Debugging)
**Decision**: Skip lesson 5 entirely
**Rationale**: Student wanted to move faster towards Pinocchio (main objective)

## 2026-02-09 — L6 ticket-registry: different field names from temoin
**Decision**: Student used their own field names instead of copying temoin exactly
**Rationale**: Better for learning. Names: ticket_cost, ticket_available, owner, start_time, desc, t_cost, start_t
**Impact**: Codama-generated TypeScript client would have different types than temoin's

## 2026-02-09 — Skip L6 frontend
**Decision**: Skip the Next.js/Codama/Gill frontend part of L6
**Rationale**: Too much boilerplate setup (Next.js + Codama + Gill + Shadcn + wallet-ui) with low learning value for Solana. Student prefers to focus on on-chain programming.

## 2026-02-09 — Copy boilerplate from temoin
**Decision**: For future frontend work, copy non-specific boilerplate files directly from temoin instead of recreating them
**Rationale**: Student pointed out this is faster and less error-prone. Only adapt files specific to the student's program (field names, instruction names).

## 2026-02-09 — Recurring student errors to watch for
**Context**: Across L4 and L6, student makes these errors repeatedly:
- `#[accounts]` instead of `#[account]` (singular for data structs)
- Forgetting `.key()` on Pubkey assignments
- Forgetting `.to_account_info()` on account references in CPI contexts
- Using `u64` instead of `i64` for timestamps
- PascalCase for function names (should be snake_case)
**Action**: Proactively remind before these come up in future exercises
