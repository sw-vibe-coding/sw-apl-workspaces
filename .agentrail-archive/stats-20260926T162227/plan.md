# stats

Phase 3 of docs/plan.md: STATS, a statistics workspace, clean-room.
STATPAK's manual and the STATPACK functions in the SHARP catalogue
say what such a package offered; the mathematics is textbook, and
every function here is written from the definition. Both modes, one
file: arithmetic, domino and mixed output are all it needs. Each
function has a HOW where one line of DESCRIBE is not enough; every
printed line within 64 columns; the random link left at 16807.

docs/plan.md, "Phase 3: STATS", holds the function list. Every step:
the sample and its expected transcript first, on a data set whose
values are worked by hand in the sample's comments; `just gates`,
`just check` and `just reg` before the commit; commit, push, report.
Milestone 3: STATS complete in both modes, transcripts pinned.

## Steps

1. stats-describe -- MEAN, MEDIAN, MODE, RANGE, VAR, SD, PVAR, PSD,
   QUANTILE, SUMMARY, ZSCORE, RANK, FREQ, HIST; DESCRIBE and the HOWs;
   a sample on a fixed data set with the values worked by hand.
2. stats-relate -- COV, CORR, REGRESS (by domino, one regressor or a
   matrix of them), RESID, RSQ, PREDICT; the sample extended with a
   data set whose fit is known exactly.
3. stats-test -- TTEST1, TTEST2, CHISQ, NORMAL (a cited rational
   approximation), TDIST if a short one exists; RANDN by Box-Muller,
   RANDU, SAMPLE, SHUFFLE, with pinned transcripts. Milestone 3.
