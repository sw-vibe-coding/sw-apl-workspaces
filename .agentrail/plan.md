# course

Phases 0 and 1 of docs/plan.md (owner direction 2026-09-24): the
rest of the process scaffold, then COURSE -- an APL course, lesson by
lesson, with quizzes marked as the reader goes, in the spirit of what
APLCOURSE was for and written fresh here. One file, (A)(B), every
printed line within 64 columns, uppercase, input by quad and
quote-quad, STOP at any prompt.

docs/plan.md, "Phase 1: COURSE", holds the lesson list, the shape
(DESCRIBE, CONTENTS, LESSON N, START, NEXT, QUIZ N) and the decisions
the steps take until the owner says otherwise. Every step: the sample
and its expected transcript first, then the APL; `just gates` and
`just check` and `scripts/reg.sh run` before the commit; commit, push,
report. Milestone 1: )LOAD 2 COURSE, START, and every lesson and quiz
runs in (A) and in (B).

## Steps

1. scaffold -- Phase 0: justfile recipes, check scripts, the library
   shim, reg-rs wrappers, the sample runner, the index generator and
   the Pages workflow, samples/README.md, tests/reg-rs/, work/README.md,
   and docs/workspaces.md, testing.md, saga.md, citations.md, library.md.
2. course-shape -- the workspace's skeleton and lesson 1 in full.
3. course-arrays -- lessons 2 to 6.
4. course-tables -- lessons 7 to 10.
5. course-programs -- lessons 11 to 14.
6. course-workspace -- lessons 15 and 16, the read-through in each
   mode, the docs. Milestone 1.
