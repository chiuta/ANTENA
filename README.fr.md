# ANTENA — Solveur d'antennes filaires compatible NEC-2 (méthode des moments), avec interface navigateur.

[English](README.md) · [Română](README.ro.md) · **Français** · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Version : 0.1.0-alpha.1 · Dernière mise à jour : 2026-11-22

🌐 https://antena.chiuta.ro

## État

Squelette précoce (pré-alpha) : le workspace compile et les tests passent en CI. Chaque fonctionnalité prévue est une tâche ouverte dans `docs/issues/`.

## Périmètre prévu

- Lecture des cartes NEC-2 restantes (CM, CE, GE, EX, FR, RP, EN) et validation de la géométrie.
- Méthode des moments pour fils minces : remplissage de la matrice d’impédances et solveur linéaire.
- Impédance d’entrée du dipôle demi-onde vérifiée par rapport à des valeurs publiées.
- Diagramme de rayonnement (carte RP), gain en dBi et sol parfait par la méthode des images.
- Rapport texte de style NEC-2 plus JSON, et version WebAssembly avec visionneuse canvas pour la géométrie et les diagrammes 2D.

Rien de tout cela n’est encore implémenté : chaque point est une tâche ouverte dans `docs/issues/`.

## Compiler et tester

```bash
cargo test --workspace
cargo run -p antena-cli -- dipole 14.2
```

## Contribuer

Lisez `CONTRIBUTING.md`, choisissez une tâche étiquetée `good first issue` dans `docs/issues/` et ouvrez une pull request. Aucune contribution n'est trop petite : une ligne de table, un test ou une traduction comptent.

## Implémentation indépendante

Écrite à partir de spécifications publiques et de manuels ; aucun code source tiers n'a été copié (voir `CLEANROOM.md`).

## Licence

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Soutien

Trade-free : ni pistage ni publicité. Si cela vous est utile, vous pouvez soutenir l'auteur : https://buymeacoffee.com/echo.of.the.strings
