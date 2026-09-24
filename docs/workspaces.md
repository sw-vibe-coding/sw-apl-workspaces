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

None is written yet; `docs/plan.md` says which comes first. Each
gains a section here when its step lands, saying what its public
functions are and what its sample shows.
