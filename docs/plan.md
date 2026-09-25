# sw-apl-workspaces Master Plan

This is the single source of planning truth for sw-apl-workspaces.
Every instruction from the project owner lands here first; agentrail
sagas and steps are derived from the phases below, never invented
ad hoc. When direction changes, edit this file, then re-plan the
saga (`agentrail plan --update`, `agentrail insert`).

Companion docs, written by the phases that need them:
`workspaces.md` (the conventions every workspace here follows),
`testing.md` (transcripts and gates), `saga.md` (long-form notes per
saga), `citations.md` (every source consulted, none copied),
`library.md` (how sw-apl reaches this library, at the CLI, the
service and the browser). The interpreter's own documents are in the
sw-apl repository beside this one; `docs/workspaces.md` there is the
file format and `docs/language.md` there is what each mode has.

## Goal

A library of APL workspaces for sw-apl, written from scratch here and
kept apart from the interpreter: a course that teaches APL from
nothing, a drill that makes exercises and marks the answers, a
statistics workspace, and a set of mathematics workspaces. Each is a
plain UTF-8 text file that sw-apl loads by name, runs in both of
sw-apl's modes wherever both can run it, and carries a `DESCRIBE`
that says what it holds.

This repository is what sw-apl's plan calls "the owner's other
repository" (sw-apl `docs/plan.md`, Phase 10, owner direction
2026-09-24): library 2 in the numbering that `)LIBS` reports as
`0 USER, 1 CORE, 2 EXTENDED`. sw-apl ships its own library 1 (LIFE,
RACE, EDIT, BIRDS, TTTML and LEARN); everything beyond that lives
here, so that `)LIB 2` lists it and `)LOAD 2 COURSE` loads it once
sw-apl can be pointed at a directory or a URL.

Order of work (owner direction 2026-09-24): COURSE and DRILL first,
then STATS, then the other mathematics workspaces.

## What this repository is not

- **Not converted IBM material.** APLCOURSE, TYPEDRILL, PLOTFORMAT,
  ADVANCEDEX, STATPAK and the rest of APL\360's library 1 are IBM's
  (or STSC's) work of unclear copyright. Nothing here is transcribed,
  converted, decoded or quoted from them. Their manuals and the
  APL\360 User's Manual say what such a workspace *did*, and that is
  used the way a specification is used: COURSE teaches the same
  language, DRILL sets the same kind of exercise, STATS computes the
  same statistics, and every line of APL is written here.
- **Not an importer.** Decoding binary workspaces from tape images
  (the MTS D5 tapes, VS APL's 800-byte records) is a possible future
  goal, low priority, and not planned. If it is ever done, its output
  stays untracked in `work/`, as sw-apl's `aplcourse-how-to.md`
  already provides for.
- **Not an interpreter change.** A workspace that needs something
  sw-apl lacks is a finding to report to the sw-apl plan, not a
  reason to write around it here in a way that makes the workspace
  not-APL. Until sw-apl has it, the workspace does without or waits.
- **Not APL2, not Dyalog.** Flat arrays, no nesting, no each, no
  diamonds, no dfns, no lowercase names, no underscores in names.

## Guiding constraints

Inherited from sw-apl, and holding here:

- **A workspace is a text file APL can re-execute**: the format in
  sw-apl `docs/workspaces.md`. Del definitions for functions,
  assignments for variables, `⍝!` directives for the settings and the
  random link, `)WSID NAME` for the name. What `)SAVE` writes is what
  is tracked, edited by hand where the plan says so (the random link,
  below).
- **The modes line and the file name agree.** `⍝!MODES (A)(B)` in
  `NAME.apl.ws`; `(A)` in `NAME.a-70.apl.ws`; `(B)` in
  `NAME.b-75.apl.ws`. sw-apl lists and loads a workspace only in the
  modes its line names, so an (A) session never sees a (B)-only
  workspace, whatever it is called. A gate checks the two agree.
- **Both modes wherever possible.** The shared core of (A) '70 and
  (B) '75 is nearly the whole language: the primitives, the del
  editor, quad and quote-quad, domino, roll and deal, mixed output by
  semicolon. A workspace that uses only that is `(A)(B)` and kept
  once. What makes a file one mode's: an I-beam or a group makes it
  (A) only; execute, format, or any quad name (`⎕IO`, `⎕LX`, `⎕FX`)
  makes it (B) only. So a both-mode workspace reads its input with
  `⎕` and `⍞`, prints with mixed output, and dispatches over a table
  of primitives the way sw-apl's (A) BIRDS does, rather than by
  execute.
- **When (B) is needed, say so in the name.** A workspace that needs
  execute, format or the quad names is written as `NAME.b-75.apl.ws`
  with `⍝!MODES (B)`, and nothing else. Where the same workspace can
  exist in both but is richer in (B), it is a pair, `NAME.a-70.apl.ws`
  beside `NAME.b-75.apl.ws`, both called `NAME` (sw-apl's BIRDS is the
  precedent), and the functions the two share are kept textually
  identical, which a gate checks.
- **Lines fit 64 columns.** (B)'s clear workspace prints 64 wide,
  the 5110's screen. Every line of text a workspace prints is at most
  64 characters, so nothing wraps on the narrower mode. Text is
  uppercase, as APL\360 printed it and as every sw-apl workspace is
  written.
- **Provenance on every tracked workspace.** A `⍝!SOURCE` line naming
  this repository, and a gate (`scripts/check-provenance.sh`) that
  fails when one is missing, when a name and a modes line disagree,
  or when an untracked file appears under `ws/`. Material from
  elsewhere goes in `work/`, which is ignored.
- **Transcripts are the tests.** There is no Rust here. A workspace is
  tested by running it through sw-apl in every mode it claims, with
  every input scripted, and pinning the transcript with reg-rs, one
  test per `samples/*.apl`, exactly as sw-apl does. Rebase a baseline
  only with the reason in the commit message.
- **Reproducible randomness.** DRILL and STATS use `?`. sw-apl's
  random link starts at 16807 in a clear workspace and `)LOAD`
  restores the link a file carries, so a transcript is reproducible
  as long as every tracked file's `⍝!LINK` is 16807. `)SAVE` moves it;
  the gate checks it is put back.
- **README and top-level markdown are ASCII-only**; `docs/*.md` may
  use glyphs. `sw-markdown-checker` gates the former.
- **Process**: agentrail sagas derived from this plan; TDD (the
  sample and its expected transcript first, then the APL); the
  `/mw-cp` checkpoint before every commit; the step completion
  report; `CHANGES.md` regenerated from `git log`, never hand-edited.

## Conventions every workspace follows

- `DESCRIBE`: a niladic function, first in the file after the
  variables, that says what the workspace is, names the functions to
  run, and says what to type next. `)LOAD` prints only the SAVED
  line, so DESCRIBE is the greeting, typed by the reader.
- `HOWNAME`: a niladic function for each function worth more than one
  line of DESCRIBE, printing its syntax, what it does, and an example.
- Interactive functions read with `⎕` where the answer is APL (a
  number, a vector, an expression) and with `⍞` where it is text (a
  choice, `STOP`). A prompt printed with `⍞←` stays on the line.
- A drill or a lesson can always be left: the word `STOP` typed at
  any prompt returns to immediate execution, and a lesson says how.
- Names are APL names, uppercase, with no underscores, and at most
  eight characters where a listing lines up better for it. Subfunctions
  a reader is not meant to call start with `∆` (`∆ASK`, `∆SHOW`), so
  `)FNS` shows the public names first and the machinery after.
- No `⎕LX` in a both-mode file: it would make the file (B) only. A
  (B)-only file may set `⎕LX←'DESCRIBE'`.
- Every workspace has a sample in `samples/` that loads it, runs
  DESCRIBE, and exercises every public function with scripted input,
  and a section in `samples/README.md` saying what the transcript
  shows.

## How sw-apl reaches this library

sw-apl is building the mechanism (its Phase 10, steps `library-config`
and `browser-libraries`); this repository supplies the layout those
steps are written against, and tests against sw-apl as it lands.

- **At the CLI and the service** (sw-apl a718f19, 2026-09-24): a
  `--lib 2=PATH/ws,EXTENDED` flag, or the same in `sw-apl.toml`. Every
  script here runs sw-apl with `--lib 2=ws,EXTENDED` and the samples
  say `)LOAD 2 NAME`. (Before it landed, a shim made `ws/` library 1
  and the samples said `)LOAD 1`; the switch rebased every baseline
  once, 2026-09-24.)
- **In the browser** (the same sw-apl commit): the page reads
  `libraries.json` beside it and fetches each library's index and
  files. The published demo's library 2 is this repository's
  `ws/library.json` through raw.githubusercontent.com; it moves to
  this repository's GitHub Pages, whose responses carry
  `Access-Control-Allow-Origin: *`, once Pages is enabled for the
  repository. The index is `ws/library.json`:

  ```
  {"name": "EXTENDED",
   "workspaces": [
     {"name": "COURSE", "file": "COURSE.apl.ws", "modes": "(A)(B)"},
     {"name": "DRILL", "file": "DRILL.a-70.apl.ws", "modes": "(A)"},
     {"name": "DRILL", "file": "DRILL.b-75.apl.ws", "modes": "(B)"}]}
  ```

  generated by `scripts/gen-index.sh` from the files and their modes
  lines, tracked, and gated to be current. The page reads `file` from
  each entry and takes the modes from the file itself.
- **The binary** the tests run is `SW_APL` if set, else
  `../sw-apl/target/release/sw-apl`, else `sw-apl` on the path. The
  version it reports is recorded in each reg-rs run's notes so a
  transcript that moves can be told from an interpreter that did.

What this repository needs from sw-apl, and where it stands:

| Need | sw-apl step | Status here |
|---|---|---|
| `)LIB 2`, `)LOAD 2 NAME` from a configured directory | `library-config` | Landed; in use |
| `)LIBS` naming this library | `library-config` | Landed |
| Libraries by URL in the browser, an index file | `browser-libraries` | Landed; GitHub Pages for this repository still to be enabled |
| `)HELP`, `)DIALECT` for COURSE's opening lesson | `dialect`, `help` | Landed |
| System commands and errors in the reply to `⎕` | `quad-input-commands` | In progress; lesson 1 revised when it lands (`lib-course-opening`) |

## Phases

Each phase becomes one agentrail saga. Step slugs are stable
identifiers; prompts are written when the saga is planned.

### Phase 0: bootstrap (with the `course` saga)

Done at setup (2026-09-24): COPYRIGHT, LICENSE, `.gitattributes`,
`.gitignore`, the README, this plan, CLAUDE.md, the `/mw-cp` command,
the agentrail saga, the `justfile`, and the scripts: `sw-apl.sh`
(which binary runs), `check-provenance.sh` (the SOURCE line, the
name against the modes line, LINK 16807, no untracked files under
`ws/`), `check-ws.sh` (run every workspace in every mode it claims,
fail on any error report, and check the 64-column rule on what is
printed), `lib-shim.sh`, `reg.sh`, `reg-seed.sh`,
`normalize-apl-output.sh` and `run-samples.sh` (from sw-apl, with the
shim's library and a mode taken from a sample's first line), and
`gen-changes.sh`. The shim went with the `lib-cli` step, once sw-apl
had numbered libraries of its own.

1. `scaffold` -- the rest: `scripts/gen-index.sh` writing
   `ws/library.json` and a gate that the tracked index is current;
   the Pages workflow that publishes `ws/`; `samples/README.md`,
   `tests/reg-rs/`, `work/README.md`; and `docs/workspaces.md`,
   `docs/testing.md`, `docs/saga.md`, `docs/citations.md`,
   `docs/library.md`, so that every document the README names
   exists. A first trivial workspace is not needed: COURSE's shape is
   the next step and proves the scaffold.

### Phase 1: COURSE (saga `course`)

A course in APL for someone who has read sw-apl's LEARN, or has not.
LEARN is orientation and the essentials in nine short lessons
(sw-apl plan, Phase 10); COURSE is the language, lesson by lesson,
with exercises marked as they go. It is in the spirit of what
APLCOURSE was for -- learning APL at the terminal from the workspace
itself -- and written fresh: its lessons are ours, its exercises are
ours, and it teaches the APL of sw-apl's two modes.

Shape:

- `DESCRIBE`, then `CONTENTS` listing the lessons by number and
  title, `LESSON N` to read one, `START` for the first, and `NEXT`
  to go on from the last one read (the number kept in a variable the
  reader may `)SAVE` with the workspace).
- A lesson prints its text in short sections, each followed by a few
  expressions to try, and waits with `⍞` between sections so the
  reader can type them. At the end, `QUIZ N`: a handful of questions
  whose answers are read with `⎕`, checked for value and shape with
  tolerant equality, and explained when wrong. `STOP` at any prompt
  leaves.
- One file, `(A)(B)`, so the same course serves both modes; a lesson
  that must differ (the settings commands of (A) against the system
  variables of (B); the I-beams; execute and format) says so in its
  text and shows both, marked, rather than splitting the file. The
  reader is told which mode they are in by `)DIALECT` in lesson 1.
- Every printed line fits 64 columns. Text is uppercase.
- Budget: the whole workspace in well under sw-apl's default 1 MB; a
  5110-sized (64 KB) edition is not a goal.

Lessons, in order (the step that writes each may split or merge, and
records the final list in `docs/workspaces.md`):

1. Where you are: the prompt, `)DIALECT`, `)WSID`, `)LIBS`, `)LIB`,
   `)HELP`, how a lesson works, `STOP`, `NEXT`.
2. Numbers: the arithmetic functions, the high minus, right to left,
   parentheses, `⌈ ⌊ | * ⍟ ○ !` monadic and dyadic.
3. Vectors: strands, scalar extension, `⍳`, `⍴`, reshape, ravel,
   catenate, empty vectors.
4. Comparison and logic: `< ≤ = ≥ > ≠ ∧ ∨ ~`, Booleans as numbers,
   `⌈` and `⌊` as max and min, `≠` as exclusive or.
5. Reduction and scan: `+/ ×/ ⌈/ ∧/`, `+\`, the identity of an empty
   reduction, reductions as loops you do not write.
6. Selection: bracket indexing, indexed assignment, `↑ ↓`, `⌽ ⊖`,
   compress and expand, `∊`, `⍳` as index-of.
7. Matrices: rank, `⍴` again, indexing with two subscripts, `⍉`,
   reduction along an axis, `+/[1]`, display.
8. Products: outer product and its tables, inner product `+.×` and
   what else it can be, a multiplication table and a distance table.
9. Order and counting: `⍋ ⍒`, sorting with them, `?` and deal, `⊤ ⊥`,
   base conversion, a histogram of counts.
10. Characters: character vectors and matrices, quotes, `∊` and `⍳`
    on text, mixed output, a formatted line.
11. Defining functions: the del, headers of every valence, a result,
    locals, editing a line, displaying a function, `)FNS`, `)ERASE`.
12. Branching: `→`, labels, the conditional branch idiom `→(X)/L`,
    loops, `→0`, and why a reduction is usually better.
13. Input and output: `⎕` and `⍞`, prompts, a function that asks,
    checking an answer.
14. Errors: reading a report, SYNTAX VALUE DOMAIN LENGTH RANK INDEX,
    the state indicator, `)SI`, `→` to clear, `)SIV`.
15. Workspaces: `)SAVE`, `)LOAD`, `)COPY`, `)CLEAR`, `)DROP`, library
    0 and the libraries, `DESCRIBE` as the convention.
16. The two modes: what (A) has alone (I-beams, `)ORIGIN` `)DIGITS`
    `)WIDTH`, groups) and what (B) has alone (execute, format, the
    quad names), the same program in each, and where next: DRILL,
    STATS, sw-apl's docs.

Steps:

1. `scaffold` (Phase 0 above).
2. `course-shape` -- the workspace with DESCRIBE, CONTENTS, LESSON,
   START, NEXT, the section pause, QUIZ with `∆ASK` and `∆CHECK`, and
   lesson 1 written in full as the model for the rest. The sample
   walks lesson 1 and its quiz with a right answer, a wrong one and
   STOP; it runs in both modes.
3. `course-arrays` -- lessons 2 to 6.
4. `course-tables` -- lessons 7 to 10.
5. `course-programs` -- lessons 11 to 14.
6. `course-workspace` -- lessons 15 and 16, a read-through of the
   whole course in each mode, `docs/workspaces.md`'s entry, the README
   table row, and the sample's README paragraph. **Milestone 1:**
   `)LOAD 2 COURSE`, `START`, and every lesson and quiz runs in (A)
   and in (B).

### Phase 2: DRILL (saga `drill`)

An exercise generator in the spirit of APLCOURSE's TEACH and
EASYDRILL, written fresh. It makes an expression at random, prints
it, reads the reader's answer with `⎕`, checks it, and keeps score.
The reader chooses the functions drilled and the difficulty.

Design:

- **Tables, not execute.** In (A) there is no way to evaluate a
  character vector, so the generator works from tables: a character
  vector of the monadic glyphs it drills and one of the dyadic, a
  dispatch function per table that branches to the glyph's line
  (sw-apl's (A) BIRDS shows the idiom), and the expression displayed
  by catenating the glyph between the operands' characters. The value
  the reader is marked against is computed by the dispatch, never by
  reading the display back. Numbers are shown as APL shows them, so
  the display and the value agree.
- **Operands** are small integers, scalars and short vectors, chosen
  with `?` from ranges that keep the answer whole where the function
  allows (division and roots are drilled on numbers chosen to divide
  and to be squares; the reader is told that is so). Vectors are
  conformable or one is a scalar; a LENGTH ERROR is never the answer.
- **Checking** is tolerant equality on value and equality of shape.
  A scalar answer to a one-element vector is accepted and remarked
  on. A wrong answer shows the right one; three wrong answers in a
  row show the expression worked out.
- **Levels and topics.** `EASY`: one function, scalars. `DRILL`: one
  or two functions, vectors. `HARD`: two or three functions with
  parentheses and a reduction. Topics chosen at the start from a
  printed menu read with `⍞`: ARITHMETIC, COMPARE, MAXMIN, REDUCE,
  INDEX, STRUCTURE, ALL. `STOP` ends a session and prints the score.
- **Reproducible.** From a clear workspace, the same choices give the
  same drill; the sample pins one session per level with scripted
  answers. `⍝!LINK` stays 16807 in the tracked file.
- **The (B) pair.** Execute makes a second kind of drill possible:
  show a value and ask the reader to *write* an expression that
  produces it, then evaluate what they typed. That is `WRITE`, and it
  needs `⍎`, so it cannot be in the both-mode file. DRILL becomes a
  pair: `DRILL.a-70.apl.ws` holds the tables drill, `DRILL.b-75.apl.ws`
  holds the same functions unchanged plus `WRITE` and `⎕LX←'DESCRIBE'`.
  A gate diffs the shared functions between the two files.

Steps:

1. `drill-tables` -- the glyph tables, the two dispatch functions,
   the operand generator, the expression's display, and a sample that
   shows a hundred generated expressions with their values, in both
   modes, as the check that display and value agree.
2. `drill-session` -- EASY, DRILL and HARD, the menu, the loop,
   marking, the worked answer, the score, STOP, DESCRIBE and HOWDRILL;
   samples with scripted answers per level.
3. `drill-topics` -- INDEX and STRUCTURE topics (indexing, take, drop,
   reverse, rotate, compress, membership, index-of) and a vector
   reduction in HARD; the samples extended. **Milestone 2:** DRILL
   runs in both modes and the transcripts are pinned.
4. `drill-write` -- the (B) pair: WRITE, the split into `.a-70` and
   `.b-75` files, the shared-function gate, a (B) sample.

### Phase 3: STATS (saga `stats`)

A statistics workspace, clean-room. STATPAK's manual and the STATPACK
functions catalogued in the SHARP library say what such a package
offered; the mathematics is textbook, and every function here is
written from the definition, not from any listing.

Functions, each with a HOWNAME, on numeric vectors (and matrices
where a column is a variable):

- Description: `MEAN`, `MEDIAN`, `MODE`, `RANGE`, `VAR` and `SD`
  (sample, with `PVAR` and `PSD` for population), `QUANTILE`,
  `SUMMARY` (five-number summary and the mean, as mixed output),
  `ZSCORE`, `RANK`.
- Tables and pictures: `FREQ` (a frequency table of a vector), `HIST`
  (a character histogram: one row per bin, the count as a row of
  `*`, scaled to fit 64 columns), `STEM` if it fits.
- Two variables: `COV`, `CORR`, `REGRESS` (least squares by domino,
  returning intercept and slope, or the coefficients for a matrix of
  regressors), `RESID`, `RSQ`, `PREDICT`.
- Tests: `TTEST1` (one sample against a mean), `TTEST2` (two samples,
  pooled), `CHISQ` (a contingency table), each returning the statistic
  and degrees of freedom; `NORMAL` (the standard normal distribution
  function, by a rational approximation good to six places, its
  source cited) and `TDIST` if a short approximation exists, so a
  p-value can be printed.
- Generation: `RANDN` (normal deviates by Box-Muller, from `?` and
  `○`), `RANDU` (uniform on an interval), `SAMPLE` (deal without
  replacement), `SHUFFLE`.

Both modes, one file: everything above is arithmetic, domino and
mixed output. Printing precision is the workspace's `⍝!DIGITS`; a
reader wanting more sets it (`)DIGITS` in (A), `⎕PP` in (B)), which
DESCRIBE says.

Steps:

1. `stats-describe` -- the descriptive functions, FREQ and HIST,
   DESCRIBE and the HOWs; a sample on a fixed data set with values
   worked by hand in the sample's comments.
2. `stats-relate` -- COV, CORR, REGRESS and its relatives; the sample
   extended with a data set whose fit is known exactly.
3. `stats-test` -- the tests and NORMAL; RANDN, RANDU, SAMPLE, SHUFFLE
   with pinned transcripts. **Milestone 3.**

### Phase 4: the other mathematics workspaces (saga `math`)

Each is small, both-mode, and stands alone; the order is by how much
each is used by the ones after. A function needed by two workspaces
is written once and copied, with a comment saying where the original
is, since there is no cross-workspace reference in APL\360.

1. `MATH` -- number theory and combinatorics: `PRIMES N` (a sieve),
   `ISPRIME`, `FACTORS`, `GCD`, `LCM`, `TOTIENT`, `FIB N`, `PASCAL N`,
   `PERMS` (all permutations of `⍳N` as a matrix, small N),
   `COMBS` (combinations), `BASE` (any base to any base by `⊤` and
   `⊥`), `DIGITS`, `ROMAN`, `COLLATZ`.
2. `PLOT` -- character plots to 64 columns: `PLOT Y`, `X PLOT Y`,
   `BAR`, `SCATTER`, with axes marked by the smallest and largest
   value, as PLOTFORMAT's purpose was; drawn with `∘.=`, `⌈` and
   indexing. STATS's HIST may then be replaced by a call to the same
   idiom, copied.
3. `MATRIX` -- linear algebra over domino and the products: `ID N`,
   `DET` (by elimination, so it needs no primitive sw-apl lacks),
   `INV`, `SOLVE`, `TRACE`, `MPOW`, `NORM`, `ISSYM`, `GRAM` (Gram-
   Schmidt), and a lesson-like `HOWDOMINO` on what `⌹` is.
4. `POLY` -- polynomials as coefficient vectors: `PEVAL` (Horner by
   `⊥`), `PADD`, `PMUL` (by outer product and diagonal sums), `PDIV`,
   `PDERIV`, `PINT`, `PROOTS` (Newton from a spread of starts, real
   roots), `PSHOW` (a readable rendering).
5. `CALC` -- numerical calculus: `INTEG` (Simpson over a table of
   values, and over a function named in a table the way BIRDS reaches
   primitives), `DERIV`, `NEWTON`, `BISECT`, `SERIES` (e, pi, sin by
   their series, to a tolerance, compared with `*` and `○`).

Steps: one per workspace, `math-math`, `math-plot`, `math-matrix`,
`math-poly`, `math-calc`, each with DESCRIBE, the HOWs, a sample and
its README paragraph. **Milestone 4.**

### Phase 5: the library in sw-apl (saga `library`)

sw-apl's `library-config` and `browser-libraries` landed on
2026-09-24 (its commit a718f19), so this phase's first step moved up
into the `course` saga.

1. `lib-cli` -- done 2026-09-24, in the `course` saga: the samples
   switched from the shim's `)LOAD 1` to `)LOAD 2`, every baseline
   rebased with that one reason; the shim removed; `)LIBS` shows
   EXTENDED; the README's "Use it" says exactly what to type.
2. `lib-browser` -- GitHub Pages enabled for this repository and
   serving `ws/`, sw-apl's `libraries.json` pointed at it instead of
   raw.githubusercontent.com, and a check that the published demo
   lists and loads COURSE in each mode. **Milestone 5:** `)LOAD 2
   COURSE` and `START` in the browser.
3. `lib-course-opening` -- once sw-apl's `quad-input-commands` lands:
   lesson 1 lets the reader try `)DIALECT`, `)WSID`, `)LIBS` and
   `)HELP` at the pause itself, drops the paragraph on clearing an
   error with `→` (an error at a `⎕` prompt then re-prompts), checks
   the text against what sw-apl prints, and re-pins course-1.

## Decisions

- **Names**: COURSE, DRILL, STATS, then MATH, PLOT, MATRIX, POLY,
  CALC. The owner said STATS; sw-apl's plan used STATISTICS as an
  example only.
- **The random link** in every tracked file is 16807, put back by
  hand after a `)SAVE` and gated, so that transcripts using `?` are
  reproducible from `)LOAD`.
- **Uppercase, 64 columns**, both modes unless a feature forces one,
  and a (B)-only need goes in a `.b-75` file (a pair when it is the
  same workspace made richer, a workspace of its own when it stands
  alone).
- **Clean room**: sources are cited in `docs/citations.md`; no IBM or
  STSC text or code enters the repository; the historical workspaces
  are referred to by name as what a function is in the spirit of, and
  nothing more.
- **The interface with sw-apl** is the file layout of its library 1
  and an index file; anything else sw-apl needs is a request to its
  plan, recorded here under "How sw-apl reaches this library".
- **Interpreter gaps found while writing** (a primitive that
  misbehaves, a missing trouble report) are reported to the sw-apl
  plan with the sample that shows them, not worked around silently.
  The sample stays here as a record until sw-apl fixes it, marked in
  `samples/README.md`.

## Findings for sw-apl

Found while writing the workspaces; reported to the owner, recorded
here until sw-apl's plan takes them up. None is worked around in a
way that makes a workspace not-APL.

- **An error in the reply to `⎕` suspends the caller** (2026-09-24,
  COURSE step 2). A LENGTH ERROR typed at a `⎕` prompt inside
  `∆PAUSE` prints its report against `∆PAUSE[1]`, but `)SI` then
  shows the *calling* function suspended (`TEST[3]*`), and `→1`
  restarts that caller rather than the line that read. The two
  displays disagree; one of them is wrong. Whether the manuals have
  the input request re-issued after an error, rather than the
  function suspended, is worth checking at the same time. COURSE
  tells the reader to type `→` and read the lesson again, which is
  right whichever way this is settled.
- **A system command at a `⎕` prompt is a SYNTAX ERROR** (same
  day). `)SI` typed in reply to `⎕` is reported as a syntax error
  in the line that read. APL\360 accepted system commands in reply
  to quad input. Lesson 1 therefore asks the reader to try
  `)DIALECT`, `)WSID`, `)LIBS` and `)HELP` after the lesson, not at
  its pauses.
