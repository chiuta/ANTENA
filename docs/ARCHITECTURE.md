# ANTENA — architecture

`antena-core`: card parsing (`GW`, later `GE`, `EX`, `FR`, `RP`, `EN`), geometry checks, then the solver (matrix fill, LU, currents, patterns).
Validation first: every solver step is compared with published NEC-2 results (half-wave dipole ≈ 73 Ω, Yagi gain, etc.); cite each reference in the test.
The goal is a *reference-grade, readable* NEC-2 equivalent, not a new numerical method (see LANDSCAPE.md for a project that does that).
