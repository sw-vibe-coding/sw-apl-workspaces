# bench

Phase 8 of docs/plan.md: speed, APL beside Rust. criterion benches in
the oracle crate time the same subject through the in-process sw-apl
session and through the oracle's Rust reference: array primitives,
APL-level loops, domino, the continued fractions, PROOTS, and the
sieve at two sizes. One run's table and its reading go in
docs/testing.md, labelled with machine, date and sw-apl commit; the
numbers are not pinned. `just bench` runs it.

## Steps

1. bench-compare -- the benches, `just bench`, the table and its
   reading, and notes in docs/workspaces.md where a workspace's idiom
   is the slow one.
