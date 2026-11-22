# ANTENA — NEC-2-compatible wire antenna solver (method of moments) with a browser front end.

**English** · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Version: 0.1.0-alpha.1 · Last updated: 2026-11-22

🌐 https://antena.chiuta.ro

## Status

Early skeleton (pre-alpha): the workspace builds and the tests pass in CI. Every planned feature is an open task in `docs/issues/`.

## Planned scope

- Parsing of the remaining NEC-2 cards (CM, CE, GE, EX, FR, RP, EN) and geometry validation.
- Method of moments for thin wires: impedance matrix fill and a linear solver.
- Half-wave dipole input impedance checked against published values.
- Radiation pattern (RP card), gain in dBi, and a perfect ground by the image method.
- NEC-2-style text report plus JSON, and a WebAssembly build with a canvas viewer for geometry and 2D patterns.

None of this is implemented yet: each item is an open task in `docs/issues/`.

## Build and test

```bash
cargo test --workspace
cargo run -p antena-cli -- dipole 14.2
```

## Contribute

Read `CONTRIBUTING.md`, pick a task labelled `good first issue` from `docs/issues/` and open a pull request. No contribution is too small: a table row, a test or a translation counts.

## Independent implementation

Written from public specifications and textbooks; no third-party source code is copied (see `CLEANROOM.md`).

## License

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Support

Trade-free: no tracking, no ads. If it is useful, you can support the author: https://buymeacoffee.com/echo.of.the.strings
