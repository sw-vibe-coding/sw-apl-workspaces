# Testing

There is no Rust here and nothing to build. A workspace is tested by
running it through sw-apl, in every mode it claims, with every input
scripted, and pinning the transcript. Three things do that.

## The gates

`just gates` and `just check`, fast and repo-wide, run before every
commit (see `.claude/commands/mw-cp.md`):

| Gate | What it checks |
|---|---|
| `sw-markdown-checker` | README, `samples/README.md` and every other markdown outside `docs/` is ASCII-only |
| `scripts/check-provenance.sh` | Every tracked workspace carries the SOURCE line; its name agrees with its modes line; its random link is 16807; nothing untracked sits under `ws/` |
| `scripts/gen-index.sh --check` | `ws/library.json` matches the files |
| `scripts/gen-pair.sh --check` | Every `NAME.a-70.apl.ws` that has a `NAME.b-75.apl.ws` beside it is what the generator derives from the (B) file |
| `scripts/check-ws.sh` | Every workspace loads in every mode it claims without an error report, and prints nothing wider than 64 characters while loading |
| `scripts/check-width.sh` | Every output line of every pinned transcript under `tests/reg-rs/` is at most 64 characters; an echoed input line is the sample's own and is not counted. Both width checks count characters with perl, since awk counts bytes here and a glyph is three |

Each exits non-zero on failure; check the exit code, not the text.

## The transcripts

`samples/*.apl` are the tests. Each sample is a session: it loads a
workspace, runs `DESCRIBE`, and exercises every public function, with
every answer to a `⎕` or `⍞` prompt on a line of its own, exactly as
it would be typed. sw-apl prints the transcript -- what was typed,
indented, and what came back -- and reg-rs pins it.

A sample runs in (A) '70 unless its first line is `⍝!MODES (B)`, the
line a workspace names its modes on; then it runs with `--mode 75`.
A workspace that runs in both modes has a sample for each, so both
transcripts are pinned.

```bash
scripts/reg-seed.sh              # create a test for each new sample
scripts/reg.sh run               # run all, summary line
scripts/reg.sh run -vv -p course # full diff for one
just samples course              # print the transcripts, to look at
```

Tests are stored under `tests/reg-rs/` (`.rgt` spec and `.out`
baseline, tracked; the run database, not). Every test runs
`scripts/run-sample.sh NAME`, which runs sw-apl with `ws/` as
library 2 and a fresh library 0 of the sample's own under `target/`,
so a sample that `)SAVE`s starts from nothing every time and leaves
nothing for another to list, and through
`scripts/normalize-apl-output.sh`, which masks the one thing a
session prints that cannot come back the same, the sign-off's clock,
and any value a sample labels `(VARIES): ` in its own output.

**Rebase a baseline only on purpose**, and say why in the commit
message. A transcript that moved because the sw-apl binary changed is
a finding: record it in `docs/plan.md` with the sample, report it,
and do not rebase to hide it.

## Test-first

Every step follows red, green, refactor:

1. RED: write the sample, and the transcript it should produce, by
   hand, in every mode the workspace claims. Run it and watch it fail
   for the right reason.
2. GREEN: the least APL that produces the transcript.
3. REFACTOR: to the conventions in `workspaces.md`, transcript
   unchanged.

Then seed the reg-rs test and pin it.

## The binary

`scripts/sw-apl.sh` decides which interpreter runs, in one place:
`SW_APL` if set, else the release build in the sw-apl checkout beside
this repository (`../sw-apl/target/release/sw-apl`), else `sw-apl` on
the path. `scripts/sw-apl.sh --version` says which one; note it when a
transcript moves.

## The library

Every script gives sw-apl this repository's `ws/` as library 2,
EXTENDED, with `--lib 2=ws,EXTENDED`, so a sample says `)LOAD 2 NAME`
and `)LIBS` shows the library as a reader of the README would see it.
The flag arrived in sw-apl at commit a718f19; an older binary refuses
it, and `scripts/sw-apl.sh --help` shows whether the one in use has
it. A sample's library 0 is `target/lib0/NAME/work`, emptied before
each run; `just apl` uses `work/` under the repository, which is
ignored. Neither writes anything that is tracked.

## The oracle

The transcripts pin what a workspace did; the hand working in the
samples says why it is right, a few cases per function. The oracle
under `oracle/` is the third check: sw-apl run in process on many
seeded random inputs, every number compared with a reference that
shares nothing with the APL.

```bash
just oracle          # cargo test in oracle/, both modes
just oracle-gates    # fmt, clippy -D warnings, sw-checklist
```

It is a cargo workspace of its own with one crate, `apl-oracle`,
which depends on sw-apl's `apl-session` by path (`../sw-apl`): a
`Console` that feeds lines from a queue, a `Host` with `ws/` as
library 2 exactly as `--lib 2=ws,EXTENDED` builds it, a wrapper that
loads a workspace and evaluates a line, returning the numbers it
printed or the error it reported, and a parser from APL's printed
numbers (high minus, E notation, rows) to `f64`. The checks are the
integration tests in `oracle/crates/apl-oracle/tests/`, one file per
workspace. Each draws cases from a fixed seed, runs them in (A) and
in (B), and compares within a tolerance stated at the top of the
file: `statrs` for the normal, t and chi-square distributions,
`nalgebra` for least squares and linear algebra, closed forms written
in the test for the rest. A failure prints the APL line, so the case
can be typed at the prompt, and one test asserts that a wrong
expectation fails.

The oracle sits beside the gates, not in them: it needs the Rust
toolchain and the sibling checkout, and it compiles part of sw-apl
the first time. Run it when a workspace's numbers change.
