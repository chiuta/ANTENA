# ANTENA — Solutore di antenne a filo compatibile NEC-2 (metodo dei momenti), con interfaccia nel browser.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · **Italiano** · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versione: 0.1.0-alpha.1 · Ultimo aggiornamento: 2026-11-14

🌐 https://antena.chiuta.ro

## Stato

Scheletro iniziale (pre-alfa): il workspace compila e i test passano in CI. Ogni funzione prevista è un'attività aperta in `docs/issues/`.

## Ambito previsto

- Lettura delle schede NEC-2 rimanenti (CM, CE, GE, EX, FR, RP, EN) e validazione della geometria.
- Metodo dei momenti per fili sottili: riempimento della matrice delle impedenze e solutore lineare.
- Impedenza d’ingresso del dipolo a mezz’onda verificata rispetto a valori pubblicati.
- Diagramma di radiazione (scheda RP), guadagno in dBi e terreno perfetto con il metodo delle immagini.
- Rapporto testuale in stile NEC-2 più JSON, e build WebAssembly con visualizzatore canvas per geometria e diagrammi 2D.

Nulla di tutto ciò è ancora implementato: ogni voce è un compito aperto in `docs/issues/`.

## Compilare e testare

```bash
cargo test --workspace
cargo run -p antena-cli -- dipole 14.2
```

## Contribuire

Leggi `CONTRIBUTING.md`, scegli un'attività con etichetta `good first issue` in `docs/issues/` e apri una pull request. Nessun contributo è troppo piccolo: una riga di tabella, un test o una traduzione contano.

## Implementazione indipendente

Scritta a partire da specifiche pubbliche e manuali; nessun codice sorgente di terzi è stato copiato (vedi `CLEANROOM.md`).

## Licenza

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Sostegno

Trade-free: niente tracciamento, niente pubblicità. Se ti è utile, puoi sostenere l'autore: https://buymeacoffee.com/echo.of.the.strings
