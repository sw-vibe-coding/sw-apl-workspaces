# Workspaces

The conventions every workspace in this repository follows. The file
format itself -- del definitions, assignments, the `⍝!` directives,
`)WSID` -- is sw-apl's, in its `docs/workspaces.md`; this is what a
workspace here does with it.

## The files

Every workspace is one plain UTF-8 text file under `ws/`, named for
the modes it runs in:

| Runs in | File | Modes line |
|---|---|---|
| (A) and (B) | `NAME.apl.ws` | `⍝!MODES (A)(B)` |
| (A) only | `NAME.a-70.apl.ws` | `⍝!MODES (A)` |
| (B) only | `NAME.b-75.apl.ws` | `⍝!MODES (B)` |

sw-apl lists and loads a workspace only in the modes its line names,
so an (A) session never sees a (B)-only workspace. The line inside is
the authority and `scripts/check-provenance.sh` fails when the name
disagrees with it.

The head of every file:

```apl
⍝ sw-apl workspace. Re-executable APL: loading it runs it.
⍝!MODES (A)(B)
⍝!SOURCE sw-apl-workspaces -- written for this repository. MIT, (c) 2026 Michael A Wright.
⍝!SAVED 12.00.00 09/24/26
⍝!LINK 16807
⍝!ORIGIN 1
⍝!DIGITS 10
⍝!WIDTH 120
)WSID NAME
```

- **SOURCE** says the workspace was written here. `)SAVE` does not
  write it; it is put in by hand, once, and the gate checks it is
  there. What committing without it would mean is in the README.
- **LINK** is 16807, the random link of a clear workspace. `)SAVE`
  writes the link where it stood, so after saving a tracked file the
  line is put back, and the gate checks it. Every transcript that
  uses `?` is reproducible from `)LOAD` because of this.
- **DIGITS** and **WIDTH** are what the file was saved under. (B)
  reads them too, so a workspace saved in (A) prints as it did there
  even where (B)'s clear workspace would be narrower.

## Both modes wherever possible

The shared core of (A) '70 and (B) '75 is nearly the whole language:
every primitive, the operators, the del editor, quad and quote-quad,
domino, roll and deal, mixed output by semicolon. A workspace that
uses only that is `(A)(B)` and is kept once.

What makes a file one mode's, and is therefore kept out of a
both-mode file:

| (A) only | (B) only |
|---|---|
| I-beams (`⌶20` and the rest) | Execute `⍎` and format `⍕` |
| `)ORIGIN`, `)DIGITS`, `)WIDTH` | Any quad name: `⎕IO`, `⎕LX`, `⎕FX`, `⎕RL`, ... |
| Groups (`)GROUP`, `)GRP`) | The screen editor's `[n⎕]` and `[∆n]` |

So a both-mode workspace reads its input with `⎕` and `⍞`, prints
with mixed output, and reaches a primitive chosen at run time by a
table -- a character vector of glyphs and a branch to the line for
each, as sw-apl's (A) BIRDS does -- rather than by execute.

**When (B) is needed**, the workspace is `NAME.b-75.apl.ws` and says
`(B)`. Where the same workspace exists in both but is richer in (B),
it is a pair, `NAME.a-70.apl.ws` beside `NAME.b-75.apl.ws`, both
called `NAME`, and the functions the two share are textually
identical. A (B)-only file may set `⎕LX←'DESCRIBE'`; a both-mode file
never sets `⎕LX`, which would make it (B) only.

## What every workspace has

- **`DESCRIBE`**: a niladic function that says what the workspace is,
  names the functions to run, and says what to type next. `)LOAD`
  prints only the SAVED line -- APL\360 had no run-on-load and sw-apl
  keeps to that -- so DESCRIBE is the greeting, typed by the reader.
- **`HOWNAME`**: a niladic function for each public function worth
  more than one line of DESCRIBE: its syntax, what it does, and an
  example.
- **A sample** in `samples/` that loads it, runs DESCRIBE, and
  exercises every public function with scripted input, one sample
  per mode it runs in, and a paragraph in `samples/README.md`.

## How a workspace talks

- Printed lines are at most 64 characters, the width of (B)'s clear
  workspace and of the 5110's screen, so nothing wraps in the
  narrower mode. `scripts/check-ws.sh` checks what a load prints;
  what a function prints when run is checked by its sample.
- Text is uppercase, as APL\360 printed it.
- A prompt for an APL value -- a number, a vector, an expression --
  is read with `⎕`, which prints its own `⎕:` prompt. A prompt for
  text -- a choice from a menu, a word -- is printed with `⍞←` and
  read with `⍞`, so the answer is typed on the same line.
- **`STOP`**, typed at any prompt, returns to immediate execution. A
  function that reads input says so the first time it asks. For a
  `⎕` prompt, `STOP` is a name the workspace defines to a value the
  function recognises; for a `⍞` prompt, it is the word.
- A wrong answer to a question is explained, and the right one shown.

## Names

- APL names, uppercase, no underscores. Public names are short enough
  that `)FNS` lines up: eight characters where it can be.
- Subfunctions a reader is not meant to call start with `∆`
  (`∆ASK`, `∆CHECK`, `∆PAUSE`), so `)FNS` shows the public names first
  and the machinery after.
- Locals are declared in the header. A function that keeps state
  between calls (COURSE's `LAST`, a drill's score) keeps it in a
  global with a comment saying so, so that `)SAVE` keeps it too.

## The workspaces

### COURSE

Both modes, `ws/COURSE.apl.ws`. APL in sixteen lessons, with a quiz
after each; `CONTENTS` lists them.

| Function | What it does |
|---|---|
| `DESCRIBE` | What COURSE is and what to type |
| `CONTENTS` | The sixteen lessons, by number and title |
| `START` | Lesson 1 |
| `LESSON N` | Reads lesson N, in sections, with a pause after each; `LESSON N S` starts at section S |
| `NEXT` | The lesson after the last one read (`LAST`, a global that `)SAVE` keeps) |
| `QUIZ N` | Questions on lesson N, each answer read with `⎕` and marked; a score at the end |
| `EXAMPLES` | The example functions lessons 11 to 14 bring with them: `SQUARE`, `HYP`, `HELLO`, `MEAN`, `COUNT`, `FACT`, `SUMLOOP`, `ASK`, `GREET`, `GUESS`, `OOPS` |

A section ends with `(TRY IT. TYPE GO TO GO ON, OR STOP TO LEAVE.)`
and a `⎕:` prompt. Anything typed there is evaluated and printed, a
system command included, so the reader tries what the section showed
where they read it; an assignment made there is a global, and
outlasts the lesson. `GO` goes on and `STOP` leaves the lesson. An
error in what is typed is reported and the prompt comes back. A
definition, or a display with `∇NAME[⎕]∇`, cannot be typed at a
pause, so lesson 11 sends the reader to the prompt for it, and
`LESSON 11 2` brings them back to the section after. A
quiz question prints its text and then
the `⎕:` prompt; the answer is APL, so `4`, `2+2` and `,4` are all
read, and the last is remarked on as a vector where a scalar was
wanted. `STOP` at a question leaves the quiz. A wrong answer shows
the right one.

The machinery: `∆TRY` (the pause line), `∆PAUSE` (the loop that
reads, tests for GO and STOP, and prints), `∆READ` (a quiz answer),
`∆IS` (was the answer the word on the left; the variables `GO` and
`STOP` hold their own names, which is what makes typing them at a
`⎕` prompt work), `∆CHECK` (wanted on the left, given on the right;
same count, same values, same rank, with a remark for a scalar
against a one-element vector), and `∆L1`, `∆Q1` and so on, one per
lesson and quiz. Every local on the way to a pause is a delta name,
so what the reader types there sees their own names. A number that is
not a lesson's is refused.

The lessons: 1 Where you are, 2 Numbers, 3 Vectors, 4 Comparison
and logic, 5 Reduction and scan, 6 Selection, 7 Matrices, 8 Products,
9 Order and counting, 10 Characters, 11 Defining functions, 12
Branching, 13 Input and output, 14 Errors, 15 Workspaces, 16 The two
modes. Lesson 15 has the reader save, list, copy and load at the
pauses; lesson 16 shows what each mode has alone, and the reader
tries their own mode's half.

Samples: `course-1.apl`, `course-2-6.apl`, `course-7-10.apl`,
`course-11-14.apl` and `course-15-16.apl` in (A), their `-75` twins
in (B), and `course-all.apl` with its twin, which read the whole
course through with every quiz answered.

### DRILL

A pair: `ws/DRILL.a-70.apl.ws` for (A) and `ws/DRILL.b-75.apl.ws` for
(B), both called DRILL. Exercises in APL made at random and marked,
in the spirit of APLCOURSE's TEACH and EASYDRILL and written fresh.
The (B) file is the source: it has everything, and `WRITE`, which
needs execute; `scripts/gen-pair.sh` derives the (A) file from it by
dropping the functions whose first comment says `(B) ONLY`, the
lines that only call one, and the `⎕LX` line, and `just gates` fails
when the derived file is stale. So the two never drift, and a change
is made once, in the (B) file, then `just pair DRILL`.

| Function | What it does |
|---|---|
| `DESCRIBE` | What DRILL is and what to type |
| `HOWDRILL` | The topics, the numbers, the marking |
| `EASY` | One function on single numbers |
| `DRILL` | One function on vectors, or two |
| `HARD` | Two functions: a chain, in parentheses, or under a reduction |
| `WRITE` | (B) only. An exercise with its functions hidden as `?`, and its value; the reader writes the expression, which is run and marked |
| `SHOW N` | Prints N exercises, each followed by its answer |

Each level first asks the topic, read with `⍞`: ARITHMETIC (`+ - × ÷
| *`, and `- × |` on one argument), COMPARE (`< ≤ = ≥ > ≠ ∧ ∨`),
MAXMIN (`⌈ ⌊`), REDUCE (`+/ ×/ ⌈/ ⌊/`, over vectors and over `+ - ×`),
INDEX (`(V)[I]`, `V⍳X`, `X∊V`), STRUCTURE (`↑ ↓ ⌽ /` with a left
argument, and `⍳ ⍴ ⌽` on one), or ALL; an empty line is ALL, a word
that is none of these is asked again, and STOP leaves. Then it prints an exercise and the `⎕:`
prompt, reads the answer as APL, and marks it as COURSE's quizzes do:
value, count and rank, with a remark for a scalar against a
one-element vector; a wrong answer shows the right one; three wrong
in a row show the exercise worked out, the inner step first when
there is one, and a line on each function. STOP prints the score.

The numbers are small and whole, chosen so that the answer is whole
where the function allows: a multiple for division, a small power, a
divisor of 1 to 5 for residue, 1 and 0 for and and or; two vectors
are the same length, and the operand outside a composed exercise
conforms to the inner value. A comparison or a residue may be the
inner function but not the outer, where the inner value may be
anything. An indexing or structural exercise is always valid and
never empty: indices within range, a take, drop or rotation within
the length, a compression that keeps something; at HARD it sits
under a reduction. The random link is the file's, so a level and a
topic give the same exercises on every load.

`WRITE` turns the drill round. After the topic, it prints an exercise
at the DRILL level with every function hidden (`3 1 4?2 GIVES`, then
`6 2 8`), reads a line with `⍞`, refuses the value itself and an
empty line, runs the line with execute, and marks the result as the
other levels do, three wrong in a row revealing the exercise.
Indexing and compression have no glyph to hide and are left out. An
expression that is an error stops WRITE with the report, since the
language has no way to catch one; HOWDRILL says to type `→` and
WRITE again. In (B) the file's `⎕LX` prints DESCRIBE on load.

The machinery: `∆RUN` is the loop; `∆MENU` sets the topic's tables
(`∆M` one-argument, `∆D` two-argument, `∆R` reductions, `∆C` allowed
inside a composed exercise, `∆O` allowed outside one); `∆KIND` picks
the shape by level and tables; `∆NEW` makes the exercise and sets the
caller's `∆E`, `∆V`, the inner step `∆E1` and `∆V1`, and the glyphs
for the hints; `∆PAIR`, `∆OUTER` and `∆SPAIR` draw operands, `∆SHOWS`
writes an indexing or structural exercise; `∆MON`, `∆DYA`, `∆REDUCE`
and `∆STR` are branch tables that apply a glyph, since (A) has no
execute, `∆DYA` and `∆STR` reading `∆X` and `∆Y` from their caller as
sw-apl's BIRDS reads `FN`; `∆CHARS` writes a number or vector as APL prints
it, without format, by encode; `∆CHECK`, `∆IS`, `∆WORK` and `∆HINT`
mark and explain.

Samples: `drill-easy.apl`, `drill-drill.apl` and `drill-hard.apl`,
one level each with scripted answers, `drill-index.apl` and
`drill-structure.apl` for the two topics, and `drill-show.apl`; each
has a `-75` twin, and the (B) `drill-show-75.apl` also defines VERIFY
with execute and counts how many of three hundred HARD exercises,
from every table, evaluate to the value the tables gave.
`drill-write-75.apl`, (B) only, plays WRITE.
