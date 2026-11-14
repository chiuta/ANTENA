# ANTENA — az első tíz feladat

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · **Magyar**

Hozd létre őket a GitHubon a `scripts/create-issues.sh OWNER/REPO` paranccsal. Szólj hozzá egy issue-hoz, hogy lefoglald.

1. **A hátralévő kártyák elemzése: CM, CE, GE, EX, FR, RP, EN** — `good first issue`, `area:parser`
   Kártyánként egy függvény, a `parse_gw` stílusában; tesztek érvényes és hibás kártyákra; egy `Deck` típus, amely az összes kártyát sorrendben tartja.

2. **Geometria-ellenőrzés: szakaszhossz, sugár, huzalmetszések** — `good first issue`, `area:geometry`
   Figyelmeztess, ha egy szakasz meghaladja a λ/10-et, vagy a sugár > szakasz/8 (hivatkozz a szabály forrására). Figyelmeztetéseket adj vissza, soha ne paníkolj.

3. **A szabadtéri impedanciamátrix kitöltése (impulzusbázis, vékonyhuzal-mag)** — `help wanted`, `area:solver`
   Dokumentáld a felírást egyenlethivatkozásokkal; egységteszttel ellenőrizd a szimmetriát és egy kézzel kiszámolt kétszakaszos esetet.

4. **Lineáris megoldás (saját LU részleges főelemkiválasztással vagy egy megengedő crate)** — `help wanted`, `area:solver`
   Komplex mátrixok; kondíciószám-védelem; teszt egy ismert 3×3-as komplex rendszeren.

5. **A félhullámú dipól bemeneti impedanciája: validálás közzétett értékekkel** — `help wanted`, `tests`, `area:solver`
   Vékony dipól, 21 szakasz: valós rész ≈ 73 Ω ± megadott és indokolt tűrés; hivatkozz a forrásra.

6. **Sugárzási diagram (RP kártya): távoltér, nyereség dBi-ben** — `help wanted`, `area:solver`
   Azimut/elevációs söprés; hasonlítsd össze a dipól diagramjának alakját az analitikussal.

7. **Tökéletes talaj a tükrözési módszerrel** — `help wanted`, `area:ground`
   Függőleges monopól tökéletes talaj felett ≈ 36 Ω; a Sommerfeld/Norton talaj egy későbbi issue-ban.

8. **Kimenet: NEC-2 stílusú szöveges jelentés és JSON** — `good first issue`, `area:output`
   Stabil JSON-séma a `docs/` alatt; golden-file tesztek.

9. **WebAssembly build és canvas-nézegető a geometriához és a 2D diagramokhoz** — `help wanted`, `area:wasm`
   Statikus oldal a `docs/demo/` alatt; illessz be egy deck-et, és lásd a huzalokat és a diagramot.

10. **Lépj kapcsolatba az Open Research Institute-tal az Arcanumról: közös tesztmodellek és licencek** — `help wanted`, `docs`
   Nyiss vitát upstream, az eredményt foglald össze a `docs/COORDINATION.md`-ben (mi közös, mi marad külön).
