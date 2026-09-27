#!/usr/bin/env -S scripts/sw-apl.sh --no-echo --lib 2=ws,EXTENDED -f
⍝ STATS through the library directory, as the demo and the CLI reach
⍝ it: run from the repository root. One of the oracle's cases.
)LOAD 2 STATS
DESCRIBE
SUMMARY 2 4 4 4 5 5 7 9
)OFF
