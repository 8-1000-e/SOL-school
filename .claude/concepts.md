# Concepts - School of Solana S8

> Suivi de ce que j'ai compris et ce qui reste flou

## Rust

### Maîtrisé ✅ (via Blueshift)
- Variables, mutabilité, shadowing
- Ownership, borrowing, lifetimes
- Structs, enums, pattern matching
- Option<T>, Result<T,E>
- Traits, generics
- Error handling

### En cours 🔄

### À revoir ❌

---

## Solana

### Maîtrisé ✅ (via Blueshift)
- Modèle de compte
- Transactions, instructions
- Programme basics

### En cours 🔄

### À revoir ❌
- À évaluer selon le cours

---

## Anchor Framework

### Maîtrisé ✅ (via Blueshift)
- Structure de base d'un programme Anchor
- Déclaration de comptes
- Instructions

### En cours 🔄
- PDAs : seeds derivation, `#[instruction(name: String)]` pour accéder aux params dans seeds
- CPIs : `CpiContext::new` + `system_program::Transfer` pour transférer des SOL
- `has_one` constraint : vérification automatique de ownership
- `.sub_lamports()` / `.add_lamports()` : manipulation directe de lamports (Anchor helpers)
- `checked_sub()` : arithmétique safe
- Système de modules Rust : `mod` vs `use`, `pub mod` + `pub use` dans mod.rs

### À revoir ❌
- Erreur récurrente : `#[accounts]` vs `#[account]` (singulier pour data structs, `#[derive(Accounts)]` pour contexte d'instruction)
- `#[max_len()]` exige des littéraux, pas des constantes
- Les timestamps Solana sont `i64` (pas `u64`)
- Noms de fonctions Rust : snake_case obligatoire

---

## Sécurité Solana

### Maîtrisé ✅
- Runtime Policy : 6 règles (immutability, data, ownership, transaction, data allocation, balance)
- Arbitrary CPI : vérifier le program ID appelé
- Duplicate Mutable Accounts : même compte passé 2x = double modification
- Re-initialization vs Frontrunning : distinction claire
- Account Reloading : données périmées après CPI

### En cours 🔄
- 11 attack vectors Ackee : concepts compris mais confusions entre mécanismes

### À revoir ❌
- **Discriminator vs Ownership** : discriminator = distinguer types (Event vs Ticket), ownership = vérifier que le compte appartient à ton programme. Deux checks différents !
- **Revival Attack** : scénario précis = close → attaquant renvoie lamports → compte ressuscite (règle 6 Balance)
- **Seeds PDA** : chaque élément rend le PDA unique. `[ticket, event, buyer]` = 1 ticket par buyer par event, pas "le ticket du créateur"
- **Type Cosplay vs Ownership Check** : type cosplay = mauvais type de compte, ownership = mauvais programme owner

---

## Notes de session

### 2025-02-04
- Début du cours
- Structure: temoin/ (référence) → my-own/ (mon travail)
- Background: Formation Blueshift complétée
- Objectif: Se concentrer sur Pinocchio

### 2026-02-09
- L4 complétée : puppet, game, bank (PDAs, CPIs, has_one, sub/add_lamports)
- L6 ticket-registry : programme Anchor complet (initialize, buy, withdraw) + 3 tests
- Erreurs fréquentes : #[accounts] vs #[account], PascalCase vs snake_case, oubli de `.key()` et `.to_account_info()`
- Frontend skippé (trop de boilerplate Next.js/Codama/Gill)

<!-- Ajouter des notes après chaque session -->
