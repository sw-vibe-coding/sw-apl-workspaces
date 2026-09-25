#!/usr/bin/env bash
# Run every samples/*.apl through sw-apl with ws/ as library 2 and
# print the transcripts. Usage: scripts/run-samples.sh [pattern]
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
lib="--lib 2=ws,EXTENDED"
pattern="${1:-}"
# A sample whose first line is `⍝!MODES (B)` runs in (B) '75.
mode_of() { head -1 "$1" | grep -q '^⍝!MODES (B)$' && echo "--mode 75" || true; }
shopt -s nullglob
for f in samples/*.apl; do
    base="$(basename "$f" .apl)"
    [ -n "$pattern" ] && [[ "$base" != *"$pattern"* ]] && continue
    echo "==== $base"
    # shellcheck disable=SC2046,SC2086  # the mode is zero or two words; the flag is two
    ./scripts/sw-apl.sh $(mode_of "$f") $lib -f "$f" || echo "(exit $?)"
done
