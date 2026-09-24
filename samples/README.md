# Samples

One transcript sample per workspace, in glyph-form APL, each ending
with `)OFF`. A sample loads its workspace from the shim library
(`)LOAD 1 NAME` until sw-apl's numbered libraries land, `)LOAD 2
NAME` after), runs `DESCRIBE`, and exercises every public function
with every input scripted, so that the transcript sw-apl prints for
it is the same every time. The transcripts are the tests: reg-rs
pins one per sample under `tests/reg-rs/` (see `docs/testing.md`).

A sample runs in (A) '70 unless its first line is the modes line a
workspace uses, naming (B): a file beginning with the lamp, `!MODES
(B)` runs with `--mode 75`. A workspace that runs in both modes has a
sample for each, so that both transcripts are pinned.

Every input a sample supplies to a quad or quote-quad prompt is a
line of the file, exactly as it would be typed at the terminal, so a
sample reads as a session: what was typed, indented six spaces, and
what came back.

The paragraphs below say what each sample shows.

`course-1.apl` and `course-1-75.apl`, one per mode, load COURSE,
print DESCRIBE and CONTENTS, read lesson 1 trying two expressions at
the first pause and GO at the rest, and take quiz 1 twice: once with
a right answer, a wrong one, an expression that makes the right one,
and STOP; once through to the score, with a one-element vector where
a scalar was wanted. Then NEXT reaches a lesson not yet written, an
error typed at a pause stops the lesson and a bare branch clears it,
STOP at a pause leaves, and a quiz and a lesson number out of range
are refused.

`course-2-6.apl` and `course-2-6-75.apl` read lessons 2 to 6, trying
at the pauses what each section showed, including an assignment that
outlasts its lesson, and take each quiz with one wrong answer among
the right ones, one answer given as an expression.
