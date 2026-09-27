# oracle

Phase 7 of docs/plan.md: an independent oracle. A Rust cargo
workspace under oracle/ that drives sw-apl in process through
apl-session's API, runs the workspaces on many seeded random inputs
in both modes, and compares every number with a reference that
shares nothing with them: statrs for the distributions, nalgebra for
linear algebra, closed forms elsewhere. The checks are cargo
integration tests, one file per workspace; a deterministic report is
pinned by reg-rs; a few shebang .apl cases cover the library path.
The sibling's Rust gates apply: fmt, clippy -D warnings, sw-checklist
sizes. `just oracle` runs it.

## Steps

1. oracle-scaffold -- the workspace, the library crate (console, host,
   parser), `just oracle`, and STATS checked end to end, TPROB and
   CHIPROB swept against statrs.
2. oracle-math-matrix -- MATH and MATRIX.
3. oracle-poly-calc -- POLY and CALC, the pinned reports, the shebang
   cases, docs/testing.md. Milestone 5.
