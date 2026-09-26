# drill

Phase 2 of docs/plan.md: DRILL, an exercise generator in the spirit
of APLCOURSE's TEACH and EASYDRILL, written fresh. It makes an
expression at random, prints it, reads the reader's answer with quad,
marks it, and keeps score; the reader chooses the functions drilled
and the level. Tables, not execute: in (A) a character vector cannot
be evaluated, so a glyph chosen at random is applied by a dispatch
function that branches to its line, and the expression is displayed
by catenating characters. Every printed line within 64 columns,
uppercase, STOP at any prompt, the random link left at 16807 so a
transcript reproduces from )LOAD.

docs/plan.md, "Phase 2: DRILL", holds the design. Every step: the
sample and its expected transcript first, then the APL; `just gates`,
`just check` and `just reg` before the commit; commit, push, report.
Milestone 2: DRILL runs in both modes and the transcripts are pinned.

## Steps

1. drill-tables -- the glyph tables, the two dispatch functions, the
   operand generator, the expression's display, SHOW N, and a sample
   that shows a hundred generated expressions with their values in
   both modes; in (B) the sample also evaluates each display with
   execute and counts how many agree with the dispatched value.
2. drill-session -- EASY, DRILL and HARD, the topic menu, the loop,
   marking, the worked answer after three wrong, the score, STOP,
   DESCRIBE and HOWDRILL; samples with scripted answers per level.
3. drill-topics -- INDEX and STRUCTURE topics and a reduction in HARD;
   the samples extended. Milestone 2.
4. drill-write -- the (B) pair: WRITE, the split into .a-70 and .b-75
   files, the shared-function gate, a (B) sample.
