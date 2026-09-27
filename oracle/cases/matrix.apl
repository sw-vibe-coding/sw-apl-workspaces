#!/usr/bin/env -S scripts/sw-apl.sh --no-echo --lib 2=ws,EXTENDED -f
⍝ MATRIX through the library directory, as the demo and the CLI reach
⍝ it: run from the repository root. One of the oracle's cases.
)LOAD 2 MATRIX
DESCRIBE
DET 2 2⍴4 7 2 6
)OFF
