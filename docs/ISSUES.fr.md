# ANTENA — dix premières tâches

[English](ISSUES.md) · [Română](ISSUES.ro.md) · **Français** · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Créez-les sur GitHub avec `scripts/create-issues.sh OWNER/REPO`. Commentez un ticket pour le réclamer.

1. **Analyser les cartes restantes : CM, CE, GE, EX, FR, RP, EN** — `good first issue`, `area:parser`
   Une fonction par carte, dans le même style que `parse_gw` ; tests pour cartes valides et mal formées ; un type `Deck` contenant toutes les cartes dans l’ordre.

2. **Validation de la géométrie : longueur des segments, rayon, intersections de fils** — `good first issue`, `area:geometry`
   Avertir lorsqu’un segment dépasse λ/10 ou que le rayon > segment/8 (citer la source de la règle). Retourner des avertissements, ne jamais paniquer.

3. **Remplissage de la matrice d’impédance en espace libre (base d’impulsions, noyau de fil mince)** — `help wanted`, `area:solver`
   Documenter la formulation avec des renvois aux équations ; tester unitairement la symétrie et un cas à deux segments calculé à la main.

4. **Résolution linéaire (LU maison avec pivot partiel ou un crate permissif)** — `help wanted`, `area:solver`
   Matrices complexes ; garde sur le nombre de conditionnement ; test sur un système complexe 3×3 connu.

5. **Impédance d’entrée du dipôle demi-onde : valider avec des valeurs publiées** — `help wanted`, `tests`, `area:solver`
   Dipôle fin, 21 segments : partie réelle ≈ 73 Ω ± tolérance énoncée et justifiée ; citer la référence.

6. **Diagramme de rayonnement (carte RP) : champ lointain, gain en dBi** — `help wanted`, `area:solver`
   Balayage en azimut/élévation ; comparer la forme du diagramme du dipôle avec la forme analytique.

7. **Sol parfait par la méthode des images** — `help wanted`, `area:ground`
   Monopôle vertical sur sol parfait ≈ 36 Ω ; sol de Sommerfeld/Norton dans un ticket ultérieur.

8. **Sortie : rapport texte de style NEC-2 et JSON** — `good first issue`, `area:output`
   Schéma JSON stable dans `docs/` ; tests avec fichiers de référence.

9. **Build WebAssembly et visualiseur canvas pour la géométrie et les diagrammes 2D** — `help wanted`, `area:wasm`
   Page statique sous `docs/demo/` ; coller un deck, voir les fils et le diagramme.

10. **Contacter l’Open Research Institute au sujet d’Arcanum : modèles de test partagés et licences** — `help wanted`, `docs`
   Ouvrir une discussion en amont, résumer le résultat dans `docs/COORDINATION.md` (ce qui est partagé, ce qui reste séparé).
