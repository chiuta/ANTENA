# ANTENA — primele zece sarcini

[English](ISSUES.md) · **Română** · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Creează-le pe GitHub cu `scripts/create-issues.sh OWNER/REPO`. Comentează la un issue ca să-l revendici.

1. **Analizează cardurile rămase: CM, CE, GE, EX, FR, RP, EN** — `good first issue`, `area:parser`
   O funcție per card, în același stil ca `parse_gw`; teste pentru carduri corecte și malformate; un tip `Deck` care ține toate cardurile în ordine.

2. **Validarea geometriei: lungimea segmentului, raza, intersecții de fire** — `good first issue`, `area:geometry`
   Avertizează când un segment depășește λ/10 sau raza > segment/8 (citează sursa regulii). Returnează avertismente, nu intra niciodată în panică.

3. **Umplerea matricei de impedanță în spațiu liber (bază de impulsuri, nucleu de fir subțire)** — `help wanted`, `area:solver`
   Documentează formularea cu trimiteri la ecuații; testează unitar simetria și un caz cu două segmente calculat de mână.

4. **Rezolvare liniară (LU propriu cu pivotare parțială sau un crate permisiv)** — `help wanted`, `area:solver`
   Matrice complexe; gardă pentru numărul de condiționare; test pe un sistem complex 3×3 cunoscut.

5. **Impedanța de intrare a dipolului în semiundă: validează cu valori publicate** — `help wanted`, `tests`, `area:solver`
   Dipol subțire, 21 de segmente: partea reală ≈ 73 Ω ± toleranța declarată și justificată; citează referința.

6. **Diagrama de radiație (cardul RP): câmp îndepărtat, câștig în dBi** — `help wanted`, `area:solver`
   Baleiaj în azimut/elevație; compară forma diagramei dipolului cu cea analitică.

7. **Sol perfect prin metoda imaginii** — `help wanted`, `area:ground`
   Monopol vertical peste sol perfect ≈ 36 Ω; solul Sommerfeld/Norton într-un issue ulterior.

8. **Ieșire: raport text în stil NEC-2 și JSON** — `good first issue`, `area:output`
   Schemă JSON stabilă în `docs/`; teste cu fișiere de referință.

9. **Build WebAssembly și un vizualizator pe canvas pentru geometrie și diagrame 2D** — `help wanted`, `area:wasm`
   Pagină statică în `docs/demo/`; lipești un deck și vezi firele și diagrama.

10. **Contactează Open Research Institute despre Arcanum: modele de test comune și licențe** — `help wanted`, `docs`
   Deschide o discuție în amonte, rezumă rezultatul în `docs/COORDINATION.md` (ce se împarte, ce rămâne separat).
