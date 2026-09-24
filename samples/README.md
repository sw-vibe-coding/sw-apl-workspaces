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
