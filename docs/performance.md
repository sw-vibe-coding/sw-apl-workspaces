# Performance: APL beside Rust

What the benchmarks in the oracle crate measure, what one run showed,
why the numbers fall where they do, and what that means for how the
workspaces here are written. The numbers are one machine on one day
and are not pinned; the shape they make is the durable part.

## What was measured, and how

`just bench` runs `criterion` benches in `oracle/crates/apl-oracle/
benches/compare.rs`. Each subject is timed two ways on the same data:

- **APL**: one input line evaluated by sw-apl running in the same
  process, through `apl-session`, in a session whose workspace was
  loaded, and whose data was assigned, before the timed loop began.
  What is timed is what a reader would wait for after pressing
  return: the line parsed, the functions called, the result
  formatted to text, and, in the oracle's wrapper, that text read
  back into numbers.
- **Rust**: the oracle's reference for the same computation, which
  shares nothing with the APL: `nalgebra` for least squares and the
  inverse, `statrs` for the t and chi-square tails, `num-integer` for
  gcd, and plain loops written in the bench for the rest.

Criterion's measurement time was cut to three seconds a function, so
the whole run takes about two minutes. The middle estimate is
reported. The APL side carries a small fixed bias against it: the
formatting of the answer and the parsing back, a microsecond or two,
which matters for the fastest subjects and not for the rest.

One run, on an Apple M1 Max, 2026-09-27, sw-apl at c0e6e51:

| Subject | APL, (A) | APL, (B) | Rust | APL over Rust |
|---|---|---|---|---|
| MEAN of 1000 numbers | 24.2 us | 24.1 us | 831 ns | 29 |
| SD of 1000 numbers | 115.9 us | | 1.8 us | 63 |
| REGRESS on 200 points | 67.5 us | 66.9 us | 2.0 us | 34 |
| INV of a 5 by 5 | 18.2 us | | 379 ns | 48 |
| DET of the same 5 by 5 | 206.2 us | | 118 ns | 1,748 |
| FACTORS 360360 | 212.8 us | | 104 ns | 2,046 |
| 1071 GCD 462 | 103.9 us | | 10 ns | 10,662 |
| COLLATZ 27 | 1.48 ms | | 115 ns | 12,962 |
| PRIMES 100 | 297.2 us | | 261 ns | 1,139 |
| PRIMES 300 | 2.49 ms | | 605 ns | 4,112 |
| 2 TPROB 10 | 1.87 ms | | 229 ns | 8,183 |
| 5 CHIPROB 3 | 2.01 ms | | 176 ns | 11,447 |
| PROOTS of a quartic | 21.55 ms | | 5.0 us | 4,282 |

## The shape of the gap

Rust is faster in every row, as compiled code against an interpreter
must be. That is not the finding. The finding is that the ratio is
not one number but three, and which one a subject gets depends on
where its loop lives.

**Thirty to sixty: the work is a primitive.** MEAN, SD and REGRESS
are one or two lines each, and the line is one or two primitives on
a whole array: a reduction, a subtraction, domino. Inside sw-apl a
primitive is a Rust loop over the array, so a thousand elements cost
what a thousand elements cost, and what is paid on top is the
interpretation of the line: tokenising it, binding the names,
building the result as an APL value, and printing it. That overhead
is a few tens of microseconds and does not grow with the data. At a
thousand elements it is thirty times the work; at a million it would
be a rounding error. INV of a 5 by 5 is the same story with domino.

**Thousands to ten thousand: the loop is in APL.** GCD, COLLATZ and
FACTORS are loops of two to four lines, and each pass through the
loop is interpreted again: the branch evaluated, the line parsed, the
scalars boxed and unboxed. A step that is a nanosecond in Rust is ten
microseconds in APL, so a loop of a few hundred steps lands at ten
thousand times. TPROB and CHIPROB are the same: the continued
fractions are loops of forty to a hundred steps with a dozen scalar
operations each. PROOTS is bisection and Newton, sixty and fifty
steps, each step a PEVAL over four hundred points; the points are
vectorised, which is why it is not a hundred thousand, but the steps
are not.

**The cleanest illustration is DET against INV** on the same matrix.
INV is `⌹M`, one primitive, and runs 48 times slower than `nalgebra`.
DET is elimination written out in APL to show the method, five
passes of a few lines, and runs 1,748 times slower. Nothing about the
mathematics differs; only where the loop is.

**The sieve is quadratic, and says so in the ratio.** `PRIMES N` is
the classic one-liner, `(2=+⌿0=(⍳N)∘.|⍳N)/⍳N`, which builds an N by N
table. Rust's sieve is linear-ish. So the ratio grows with N: 1,139
at a hundred, 4,112 at three hundred. The one-liner is the right
thing to show a learner; it is also why MATH's DESCRIBE says "N by a
sieve" and stops there, and why `HOWCOMBS` warns about N beyond
twelve, which is the same table in another guise.

**The two modes are the same speed.** (A) and (B) differ by a few
per cent on MEAN and REGRESS, inside the run-to-run noise. The modes
are one interpreter with different tables, and the tables are not on
the hot path.

## What it means for writing these workspaces

1. **Reach for a primitive over a loop wherever one exists.** A
   reduction over a running total, `⌹` over elimination, an outer
   product over nested loops, `⍋` over a sort written by hand. The
   workspaces mostly do this already; DET is the deliberate
   exception, kept because seeing the method is its purpose, and
   HOWDET now says INV is the fast way.
2. **Vectorise across cases, not within a step.** PROOTS runs Newton
   from four hundred and one starts at once, so each step is one
   PEVAL rather than four hundred; what remains is the fifty steps.
   The continued fractions could be run on a vector of arguments the
   same way if anyone needs a thousand p-values at once.
3. **Know what an APL loop costs and spend it where there is no
   primitive.** Euclid's algorithm, trial division, a series summed
   to a tolerance: these are loops by nature, and ten microseconds a
   step is fine for the sizes a learner meets. They would not be
   fine inside another loop.
4. **Watch the tables.** An outer product is the idiom that makes
   APL short, and it is N squared in space and time. It is right up
   to a few hundred and wrong beyond, and the workspace should say
   which side of that line it is on, as MATH and HOWCOMBS do.

## What this comparison is not

It is not a benchmark of sw-apl against another APL. Compiled Rust
with no workspace, no bounds accounting and no printing is the wrong
opponent for judging an interpreter's quality; the right one would
be GNU APL or Dyalog on the same lines, which would show whether
sw-apl's thirty-times overhead on a primitive is ordinary for the
breed. Nothing here says it is or is not.

It is not a measurement of sw-apl's own hot paths either. The oracle
drives sw-apl through its public session API, so what is timed
includes the session's parsing, the value model, the workspace's
space accounting and the transcript, all of which the interpreter's
author might profile separately.

And the numbers are one run. Criterion reports a range for each, and
on this machine the fast subjects moved by ten to twenty per cent
between runs when other work was going on; the ratios between the
three classes did not.

## Running and extending it

```bash
just bench                                  # every subject
cd oracle && cargo bench --bench compare -- "regress|inv"   # a few
```

A subject is a criterion group with an `apl-a` function, an optional
`apl-b`, and a `rust` function. To add one: load the workspace and
assign the data outside the timed closure, evaluate one line inside
it, and write the Rust reference beside it. The `--warm-up-time` and
`--measurement-time` flags shorten a run; the table above used one
and three seconds.
