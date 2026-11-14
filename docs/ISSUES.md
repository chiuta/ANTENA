# ANTENA — first ten tasks

**English** · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Create them on GitHub with `scripts/create-issues.sh OWNER/REPO`. Comment on an issue to claim it.

1. **Parse the remaining cards: CM, CE, GE, EX, FR, RP, EN** — `good first issue`, `area:parser`
   One function per card, same style as `parse_gw`; tests for good and malformed cards; a `Deck` type holding all cards in order.

2. **Geometry validation: segment length, radius, wire intersections** — `good first issue`, `area:geometry`
   Warn when a segment exceeds λ/10 or radius > segment/8 (cite the rule source). Return warnings, never panic.

3. **Free-space impedance matrix fill (pulse basis, thin-wire kernel)** — `help wanted`, `area:solver`
   Document the formulation with equation references; unit-test symmetry and a two-segment case by hand.

4. **Linear solve (own LU with partial pivoting or a permissive crate)** — `help wanted`, `area:solver`
   Complex matrices; condition-number guard; test against a known 3×3 complex system.

5. **Half-wave dipole input impedance: validate against published values** — `help wanted`, `tests`, `area:solver`
   Thin dipole, 21 segments: real part ≈ 73 Ω ± tolerance stated and justified; cite the reference.

6. **Radiation pattern (RP card): far field, gain in dBi** — `help wanted`, `area:solver`
   Azimuth/elevation sweep; compare dipole pattern shape with the analytic one.

7. **Perfect ground by the image method** — `help wanted`, `area:ground`
   Vertical monopole over perfect ground ≈ 36 Ω; Sommerfeld/Norton ground as a later issue.

8. **Output: NEC-2-style text report and JSON** — `good first issue`, `area:output`
   Stable JSON schema in `docs/`; golden-file tests.

9. **WebAssembly build and a canvas viewer for geometry and 2D patterns** — `help wanted`, `area:wasm`
   Static page under `docs/demo/`; paste a deck, see the wires and the pattern.

10. **Contact Open Research Institute about Arcanum: shared test models and licences** — `help wanted`, `docs`
   Open a discussion upstream, summarise the outcome in `docs/COORDINATION.md` (what is shared, what stays separate).
