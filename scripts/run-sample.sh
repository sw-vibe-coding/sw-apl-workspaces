#!/usr/bin/env bash
# Run one sample through sw-apl, the way every test runs it: in the
# mode its first line names, with ws/ as library 2 (EXTENDED), and
# with a library 0 of its own under target/, emptied first. A sample
# that )SAVEs therefore starts from nothing every time and leaves
# nothing for another sample to list, so two samples can run at once
# and a transcript is the same on every run. --library also moves
# library 1 there, where there is none; no sample uses library 1.
#
# Usage: scripts/run-sample.sh NAME        (samples/NAME.apl)
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
name="$1"
file="samples/$name.apl"
[ -f "$file" ] || { echo "run-sample.sh: no $file" >&2; exit 2; }
lib0="target/lib0/$name"
rm -rf "$lib0"
mkdir -p "$lib0/work"
mode=""
head -1 "$file" | grep -q '^⍝!MODES (B)$' && mode="--mode 75"
# shellcheck disable=SC2086  # the mode is zero or two words
exec ./scripts/sw-apl.sh $mode --library "$lib0" --lib 2=ws,EXTENDED -f "$file"
