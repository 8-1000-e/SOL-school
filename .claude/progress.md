# Progression - School of Solana S8

> Dernière mise à jour : 2026-02-09

## Background
- Formation Blueshift complétée (https://learn.blueshift.gg/)
- Bonnes bases Rust et Solana/Anchor
- **Objectif principal : Maîtriser Pinocchio**

## Leçon en cours
**Leçon 7** - Sécurité (prochain)

## Statut par leçon

| Leçon | Sujet | Statut | Date | Notes |
|-------|-------|--------|------|-------|
| 1 | Intro Solana + Setup | ⏭️ Probablement acquis | - | Via Blueshift |
| 2 | Fondamentaux Rust | ⏭️ Probablement acquis | - | Via Blueshift |
| 3 | Solana Programming I (Anchor) | ✅ Terminé | 2025-02-04 | Calculator complet |
| 4 | Solana Programming II (PDAs, CPIs) | ✅ Terminé | 2026-02-09 | puppet + game + bank |
| 5 | Tests & Debugging | ⏭️ Skippé | - | |
| 6 | Frontend dApps | ✅ Terminé (Anchor only) | 2026-02-09 | ticket-registry : programme Anchor complet + 3 tests OK, frontend skippé |
| 7 | Sécurité | ✅ Terminé | 2026-02-10 | Runtime Policy + 11 attack vectors, quiz 5/10 |
| Bonus | Tokens SPL | ⬜ Non commencé | - | |
| Bonus | Trident Fuzzing | ⬜ Non commencé | - | |
| **Bonus** | **Pinocchio** | ⭐ **PRIORITÉ** | - | Objectif principal |

### Leçon 7 ✅
- [x] Runtime Policy (6 règles)
- [x] 11 attack vectors Ackee (descriptions + mitigations)
- [x] Quiz sécurité (5/10 - confusions discriminator/ownership, revival attack, seeds PDA)

## Exercices complétés

### Leçon 1 ✅
- [x] Setup environnement (Rust, Solana CLI, Anchor)
- [x] Premier projet hello_world
- [x] Comprendre Proof of History (skip - connu via Blueshift)

### Leçon 2
- [ ] Quiz Rust pour évaluer niveau
- [ ] Exercices rust-by-example si nécessaire

### Leçon 2
- [ ] Variables et mutabilité
- [ ] Ownership et borrowing
- [ ] Structs et enums
- [ ] Option et Result
- [ ] Traits et generics

### Leçon 6 ✅ (Anchor seulement)
- [x] `anchor init ticket-registry`
- [x] state.rs : Event (name, description, ticket_cost, ticket_available, owner, start_time) + Ticket (owner, event, price)
- [x] errors.rs : EventError (5 variantes)
- [x] instructions/initialize.rs : PDA seeds [b"event", name, organizer], validations
- [x] instructions/buy.rs : CPI Transfer SOL vers Event PDA, Ticket PDA seeds [b"ticket", event, buyer]
- [x] instructions/withdraw.rs : sub_lamports/add_lamports, has_one = owner
- [x] instructions/mod.rs + lib.rs
- [x] anchor build OK
- [x] 3 tests passent (Initialize, Buy, Withdraw)
- [ ] Frontend skippé (Next.js + Codama + Gill trop de setup boilerplate)

<!-- Ajouter les exercices des autres leçons au fur et à mesure -->

## Tasks du cours (pour graduation)
- [ ] Task 1 (Leçon 1)
- [ ] Task 2 (Leçon 2)
- [ ] Task 3 (Leçon 3)
- [ ] Task 4 (Leçon 4)
- [ ] Task 5 (Leçon 5)
- [ ] Projet final

> Rappel: 3/5 tasks + projet final = NFT certification
