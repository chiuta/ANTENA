# ANTENA — NEC-2-kompatibilis huzalantenna-megoldó (momentum-módszer) böngészős felülettel.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · **Magyar**

Verzió: 0.1.0-alpha.1 · Utolsó frissítés: 2026-11-22

🌐 https://antena.chiuta.ro

## Állapot

Korai váz (pre-alfa): a workspace lefordul, a tesztek a CI-ben lefutnak. Minden tervezett funkció nyitott feladat a `docs/issues/` mappában.

## Tervezett terjedelem

- A hiányzó NEC-2 kártyák (CM, CE, GE, EX, FR, RP, EN) beolvasása és a geometria ellenőrzése.
- Momentum-módszer vékony huzalokra: impedanciamátrix-kitöltés és lineáris egyenletmegoldó.
- A félhullámú dipól bemeneti impedanciájának ellenőrzése közölt értékekkel.
- Sugárzási diagram (RP kártya), nyereség dBi-ben és tökéletes talaj a tükrözési módszerrel.
- NEC-2 stílusú szöveges jelentés és JSON, valamint WebAssembly-build vászon-nézegetővel a geometriához és a 2D-diagramokhoz.

Ezek közül még semmi sincs megvalósítva: minden pont nyitott feladat a `docs/issues/` mappában.

## Fordítás és tesztelés

```bash
cargo test --workspace
cargo run -p antena-cli -- dipole 14.2
```

## Közreműködés

Olvasd el a `CONTRIBUTING.md` fájlt, válassz egy `good first issue` címkéjű feladatot a `docs/issues/` mappából, és nyiss pull requestet. Nincs túl kicsi hozzájárulás: egy táblázatsor, egy teszt vagy egy fordítás is számít.

## Független megvalósítás

Nyilvános specifikációk és tankönyvek alapján íródott; harmadik fél forráskódját nem másoltuk (lásd `CLEANROOM.md`).

## Licenc

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Támogatás

Trade-free: nincs követés, nincs reklám. Ha hasznos számodra, támogathatod a szerzőt: https://buymeacoffee.com/echo.of.the.strings
