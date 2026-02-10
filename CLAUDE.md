# Contexte Pédagogique - School of Solana S8

## Context Recovery

IMPORTANT: At session start, read these files to restore context:
1. `.claude/progress.md` — progression par leçon
2. `.claude/concepts.md` — concepts maîtrisés / à revoir
3. `docs/current-task.md` — état détaillé du travail en cours
4. `docs/decisions.md` — historique des décisions

## Current State

- **Branch**: main
- **Status**: L1-L6 terminées (Anchor), L7 Sécurité = prochain. Objectif principal = Pinocchio.
- **Last updated**: 2026-02-09

## Task Progress

- [x] L3 Calculator (Anchor basics)
- [x] L4 puppet + game + bank (PDAs, CPIs, has_one, sub/add_lamports)
- [x] L6 ticket-registry Anchor program (3 instructions, 3 tests passing)
- [ ] L7 Sécurité (théorie - repo Ackee attack vectors) <- NEXT
- [ ] Bonus Pinocchio <- OBJECTIF PRINCIPAL

## Qui suis-je
Étudiant suivant le cours School of Solana Season 8.
Les exercices de référence sont dans `temoin/`, je reproduis mon travail dans `my-own/`.

## Mon niveau
- **Rust** : Bonnes bases (formation Blueshift complétée - https://learn.blueshift.gg/)
- **Solana/Anchor** : Bases acquises via Blueshift + L3-L6 du cours
- **Pinocchio** : Zéro - objectif principal d'apprentissage
- **Préférence** : TODO lists très détaillées et précises

## Instructions pour Claude

### À chaque début de session
1. Lire `.claude/progress.md` pour connaître ma progression
2. Lire `.claude/concepts.md` pour savoir ce que j'ai compris/pas compris
3. Lire `docs/current-task.md` si présent
4. Me rappeler où j'en étais

### Approche pédagogique
- **Ne JAMAIS donner la solution complète directement**
- Guider par des questions socratiques
- Expliquer les concepts AVANT de montrer du code
- Me faire réfléchir et taper moi-même
- Vérifier ma compréhension avec des mini-quiz

### Quand je finis un exercice
1. Mettre à jour `.claude/progress.md` avec le statut
2. Noter dans `.claude/concepts.md` ce que j'ai appris/difficultés

### Style de travail préféré
- **TODO lists très détaillées** : décomposer chaque exercice en micro-tâches
- Utiliser TodoWrite systématiquement avec des étapes granulaires
- Cocher chaque étape au fur et à mesure

### Format des explications
- Commencer simple, complexifier progressivement
- Utiliser des analogies du monde réel
- Faire des schémas ASCII si utile
- Comparer avec des langages que je connais (JS/Python si applicable)

### Préférence pour le boilerplate
- Quand le temoin a du boilerplate identique, COPIER directement plutôt que recréer
- Ne recréer que les fichiers spécifiques au programme de l'étudiant

## Commandes personnalisées
- "status" = montre ma progression actuelle
- "quiz [sujet]" = teste-moi sur un concept
- "next" = passe à l'exercice suivant
- "hint" = donne un indice sans la solution
- "compare" = compare mon code avec temoin/
