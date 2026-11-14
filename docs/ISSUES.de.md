# ANTENA — die ersten zehn Aufgaben

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · **Deutsch** · [Magyar](ISSUES.hu.md)

Lege sie auf GitHub mit `scripts/create-issues.sh OWNER/REPO` an. Kommentiere ein Issue, um es zu übernehmen.

1. **Die übrigen Karten parsen: CM, CE, GE, EX, FR, RP, EN** — `good first issue`, `area:parser`
   Eine Funktion pro Karte, im selben Stil wie `parse_gw`; Tests für gültige und fehlerhafte Karten; ein Typ `Deck`, der alle Karten der Reihe nach hält.

2. **Geometrieprüfung: Segmentlänge, Radius, Drahtüberschneidungen** — `good first issue`, `area:geometry`
   Warnen, wenn ein Segment λ/10 überschreitet oder Radius > Segment/8 (Quelle der Regel nennen). Warnungen zurückgeben, nie in Panik geraten.

3. **Füllen der Freiraum-Impedanzmatrix (Pulsbasis, Dünndraht-Kern)** — `help wanted`, `area:solver`
   Die Formulierung mit Gleichungsverweisen dokumentieren; Symmetrie und einen von Hand gerechneten Zwei-Segment-Fall per Unit-Test prüfen.

4. **Lineare Lösung (eigene LU mit Teilpivotierung oder ein permissives Crate)** — `help wanted`, `area:solver`
   Komplexe Matrizen; Schutz über die Konditionszahl; Test an einem bekannten komplexen 3×3-System.

5. **Eingangsimpedanz des Halbwellendipols: gegen veröffentlichte Werte validieren** — `help wanted`, `tests`, `area:solver`
   Dünner Dipol, 21 Segmente: Realteil ≈ 73 Ω ± angegebene und begründete Toleranz; Referenz nennen.

6. **Strahlungsdiagramm (RP-Karte): Fernfeld, Gewinn in dBi** — `help wanted`, `area:solver`
   Azimut-/Elevationsdurchlauf; die Form des Dipoldiagramms mit der analytischen vergleichen.

7. **Perfekter Boden nach der Spiegelungsmethode** — `help wanted`, `area:ground`
   Vertikaler Monopol über perfektem Boden ≈ 36 Ω; Sommerfeld-/Norton-Boden als späteres Issue.

8. **Ausgabe: Textbericht im NEC-2-Stil und JSON** — `good first issue`, `area:output`
   Stabiles JSON-Schema in `docs/`; Golden-File-Tests.

9. **WebAssembly-Build und ein Canvas-Viewer für Geometrie und 2D-Diagramme** — `help wanted`, `area:wasm`
   Statische Seite unter `docs/demo/`; ein Deck einfügen, die Drähte und das Diagramm sehen.

10. **Open Research Institute zu Arcanum kontaktieren: gemeinsame Testmodelle und Lizenzen** — `help wanted`, `docs`
   Upstream eine Diskussion eröffnen, das Ergebnis in `docs/COORDINATION.md` zusammenfassen (was geteilt wird, was getrennt bleibt).
