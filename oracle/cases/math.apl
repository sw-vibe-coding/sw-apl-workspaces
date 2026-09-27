#!/usr/bin/env -S scripts/sw-apl.sh --no-echo --lib 2=ws,EXTENDED -f
⍝ MATH through the library directory, as the demo and the CLI reach
⍝ it: run from the repository root. One of the oracle's cases.
)LOAD 2 MATH
DESCRIBE
PRIMES 30
)OFF
