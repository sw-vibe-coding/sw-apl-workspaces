# Samples

One transcript sample per workspace, in glyph-form APL, each ending
with `)OFF`. A sample loads its workspace from library 2, this
repository's `ws/` given to sw-apl as `--lib 2=ws,EXTENDED`
(`)LOAD 2 NAME`), runs `DESCRIBE`, and exercises every public function
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
print DESCRIBE and CONTENTS, read lesson 1 trying at each pause what
the section showed (two expressions, then the system commands, which
run at the quad prompt and hand it back), and take quiz 1 twice:
once with a right answer, a wrong one, an expression that makes the
right one, and STOP; once through to the score, with a one-element
vector where a scalar was wanted. Then NEXT reaches a lesson not yet
written, an error typed at a pause is reported and the prompt comes
back, STOP at a pause leaves, and a quiz and a lesson number out of
range are refused. The two modes' transcripts differ where `)DIALECT`
and `)HELP` answer for the mode.

`course-2-6.apl` and `course-2-6-75.apl` read lessons 2 to 6, trying
at the pauses what each section showed, including an assignment that
outlasts its lesson, and take each quiz with one wrong answer among
the right ones, one answer given as an expression.

`course-7-10.apl` and `course-7-10-75.apl` read lessons 7 to 10 the
same way: a matrix indexed and summed down its columns, the
multiplication table and a matrix product, a sort by grade, a roll
and a deal that come back the same because a loaded workspace starts
its random sequence where the file says, seconds into hours, minutes
and seconds, a histogram drawn from an outer product, and characters
compared, compressed and printed beside numbers. Each quiz has one
wrong answer, and quiz 10 is answered partly in quotes.
