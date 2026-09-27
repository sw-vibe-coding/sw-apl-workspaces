# math

Phase 4 of docs/plan.md: the other mathematics workspaces, each
small, both-mode, standing alone: MATH (number theory and
combinatorics), PLOT (character plots to 64 columns), MATRIX (linear
algebra over domino), POLY (polynomials as coefficient vectors), CALC
(numerical calculus). A function two workspaces need is written once
and copied, with a comment saying where the original is. Each has a
DESCRIBE, HOWs where a line is not enough, a sample per mode with the
values worked by hand, and every printed line within 64 columns.

docs/plan.md, "Phase 4", holds the function lists. Every step: the
sample first; `just gates`, `just check` and `just reg`; commit,
push, report. Milestone 4: the five workspaces in both modes, pinned.

## Steps

1. math-math -- MATH: PRIMES, ISPRIME, FACTORS, DIVISORS, GCD, LCM,
   TOTIENT, FIB, PASCAL, PERMS, COMBS, BASE, UNBASE, DIGITS, ROMAN,
   COLLATZ.
2. math-plot -- PLOT: PLOT Y, X PLOT Y, BAR, SCATTER, axes marked by
   the least and greatest values, drawn with the outer product.
3. math-matrix -- MATRIX: ID, DET, INV, SOLVE, TRACE, MPOW, NORM,
   ISSYM, GRAM, HOWDOMINO.
4. math-poly -- POLY: PEVAL, PADD, PMUL, PDIV, PDERIV, PINT, PROOTS,
   PSHOW.
5. math-calc -- CALC: INTEG, DERIV, NEWTON, BISECT, SERIES. Milestone 4.
