#!/usr/bin/env bash
# Run every samples/*.apl through sw-apl with the shim library and
# print the transcripts. Usage: scripts/run-samples.sh [pattern]
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
shim="$(./scripts/lib-shim.sh)"
pattern="${1:-}"
# A sample whose first line is `⍝!MODES (B)` runs in (B) '75.
mode_of() { head -1 "$1" | grep -q '^⍝!MODES (B)$' && echo "--mode 75" || true; }
shopt -s nullglob
for f in samples/*.apl; do
    base="$(basename "$f" .apl)"
    [ -n "$pattern" ] && [[ "$base" != *"$pattern"* ]] && continue
    echo "==== $base"
    # shellcheck disable=SC2046  # the mode is zero or two words
    ./scripts/sw-apl.sh $(mode_of "$f") --library "$shim" -f "$f" || echo "(exit $?)"
done
