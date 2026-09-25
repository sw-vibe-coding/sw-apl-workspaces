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

`course-11-14.apl` and `course-11-14-75.apl` read lessons 11 to 14.
Lesson 11 sends the reader to the prompt to define CUBE, and
`LESSON 11 2` brings them back to its second section; MEAN is
displayed and edited at the prompt after the lesson. The example
functions are called at the pauses: COUNT, FACT and SUMLOOP, ASK
answered inside the pause's own quad, GREET, and GUESS won on the
second try because the workspace's random link is fixed. Lesson 14's
errors are tried at a pause, OOPS is abandoned there and then left
suspended at the prompt and resumed with a branch. Each quiz has one
wrong answer.

`course-15-16.apl` and `course-15-16-75.apl` read lessons 15 and 16.
At lesson 15's pauses the workspace is renamed, saved into library 0,
listed, HELLO is erased and copied back from library 2, and the
libraries are listed; after the lesson the saved copy is loaded and
dropped. The (B) twin saves under another name, so the two can run at
once. Lesson 16 tries each mode's own features, so half of them are
errors whichever mode it runs in, and the date is labelled (VARIES).

`course-all.apl` and `course-all-75.apl` read the whole course:
START, GO at every pause, every quiz with the right answers, and NEXT
between lessons until it has nowhere to go. They are Milestone 1 of
`docs/plan.md`.

`drill-show.apl` loads DRILL and prints a hundred exercises with
their answers, the same on every run because the workspace's random
link is fixed. `drill-show-75.apl` does the same in (B) and then
defines VERIFY, with execute, in the sample rather than the
workspace: it makes a hundred more exercises, evaluates the
characters of each, and counts how many agree with the value the
tables gave. A hundred do.

`drill-easy.apl`, `drill-drill.apl` and `drill-hard.apl`, with their
(B) twins, sit the drill at each level: EASY on ARITHMETIC, with a
topic that is refused, right and wrong answers, an answer typed as
an expression, three wrong in a row worked out, a one-element vector
where a scalar was wanted, and a second EASY stopped at the topic
prompt; DRILL on ALL, with vector answers; HARD on REDUCE, where
three wrong in a row show the inner step and then the reduction.
Every one ends with STOP and the score.
