# ANTENA — primi dieci compiti

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · **Italiano** · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Creale su GitHub con `scripts/create-issues.sh OWNER/REPO`. Commenta una issue per prenderla in carico.

1. **Analizza le schede rimanenti: CM, CE, GE, EX, FR, RP, EN** — `good first issue`, `area:parser`
   Una funzione per scheda, nello stesso stile di `parse_gw`; test per schede valide e malformate; un tipo `Deck` che contiene tutte le schede in ordine.

2. **Validazione della geometria: lunghezza dei segmenti, raggio, intersezioni dei fili** — `good first issue`, `area:geometry`
   Avvisa quando un segmento supera λ/10 o il raggio > segmento/8 (cita la fonte della regola). Restituisci avvisi, non andare mai in panic.

3. **Riempimento della matrice di impedenza in spazio libero (base a impulsi, nucleo a filo sottile)** — `help wanted`, `area:solver`
   Documenta la formulazione con riferimenti alle equazioni; testa la simmetria e un caso a due segmenti calcolato a mano.

4. **Risoluzione lineare (LU propria con pivoting parziale o un crate permissivo)** — `help wanted`, `area:solver`
   Matrici complesse; controllo del numero di condizionamento; test su un sistema complesso 3×3 noto.

5. **Impedenza d’ingresso del dipolo a mezz’onda: valida con valori pubblicati** — `help wanted`, `tests`, `area:solver`
   Dipolo sottile, 21 segmenti: parte reale ≈ 73 Ω ± tolleranza dichiarata e motivata; cita il riferimento.

6. **Diagramma di radiazione (scheda RP): campo lontano, guadagno in dBi** — `help wanted`, `area:solver`
   Scansione in azimut/elevazione; confronta la forma del diagramma del dipolo con quella analitica.

7. **Terreno perfetto con il metodo delle immagini** — `help wanted`, `area:ground`
   Monopolo verticale su terreno perfetto ≈ 36 Ω; terreno Sommerfeld/Norton in una issue successiva.

8. **Output: rapporto testuale in stile NEC-2 e JSON** — `good first issue`, `area:output`
   Schema JSON stabile in `docs/`; test con file di riferimento.

9. **Build WebAssembly e visualizzatore su canvas per geometria e diagrammi 2D** — `help wanted`, `area:wasm`
   Pagina statica in `docs/demo/`; incolla un deck, vedi i fili e il diagramma.

10. **Contatta l’Open Research Institute su Arcanum: modelli di test condivisi e licenze** — `help wanted`, `docs`
   Apri una discussione a monte, riassumi l’esito in `docs/COORDINATION.md` (cosa è condiviso, cosa resta separato).
