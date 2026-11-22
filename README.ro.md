# ANTENA — Solver de antene filare compatibil NEC-2 (metoda momentelor), cu interfață în browser.

[English](README.md) · **Română** · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versiune: 0.1.0-alpha.1 · Ultima actualizare: 2026-11-22

🌐 https://antena.chiuta.ro

## Stare

Schelet timpuriu (pre-alfa): workspace-ul se compilează, iar testele trec în CI. Fiecare funcție planificată este o sarcină deschisă în `docs/issues/`.

## Domeniu planificat

- Citirea cardurilor NEC-2 rămase (CM, CE, GE, EX, FR, RP, EN) și validarea geometriei.
- Metoda momentelor pentru fire subțiri: umplerea matricei de impedanțe și un rezolvitor liniar.
- Impedanța de intrare a dipolului în semiundă, verificată față de valori publicate.
- Diagrama de radiație (cardul RP), câștig în dBi și sol perfect prin metoda imaginii.
- Raport text în stil NEC-2 plus JSON și o versiune WebAssembly cu vizualizator pe canvas pentru geometrie și diagrame 2D.

Nimic din asta nu este implementat încă: fiecare punct este o sarcină deschisă în `docs/issues/`.

## Compilare și testare

```bash
cargo test --workspace
cargo run -p antena-cli -- dipole 14.2
```

## Contribuie

Citește `CONTRIBUTING.md`, alege o sarcină etichetată `good first issue` din `docs/issues/` și deschide un pull request. Nicio contribuție nu e prea mică: un rând de tabel, un test sau o traducere contează.

## Implementare independentă

Scrisă pornind de la specificații publice și manuale; nu s-a copiat cod sursă de la terți (vezi `CLEANROOM.md`).

## Licență

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Sprijin

Trade-free: fără urmărire, fără reclame. Dacă ți-e de folos, poți sprijini autorul: https://buymeacoffee.com/echo.of.the.strings
