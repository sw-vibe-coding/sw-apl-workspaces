#!/usr/bin/env bash
# Every workspace in ws/, in every mode its modes line claims, runs
# through sw-apl without an error report -- a file is APL and loading
# it is running it -- and no line it prints is wider than 64 columns,
# (B)'s clear-workspace width, so nothing wraps on the narrower mode.
#
# Usage: scripts/check-ws.sh [pattern]
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
pattern="${1:-}"
status=0
errors='ERROR|INCORRECT COMMAND|NOT FOUND|WS FULL|INTERRUPT|NOT SAVED|NOT COPIED|NOT WITH OPEN DEFINITION'
shopt -s nullglob
for f in ws/*.apl.ws; do
    base="$(basename "$f" .apl.ws)"
    [ -n "$pattern" ] && [[ "$base" != *"$pattern"* ]] && continue
    modes="$(grep -m1 '^⍝!MODES ' "$f" | cut -d' ' -f2- || true)"
    [ -z "$modes" ] && modes='(A)'
    for m in A B; do
        [[ "$modes" == *"($m)"* ]] || continue
        [ "$m" = A ] && mode=70 || mode=75
        out="$(printf ')OFF\n' | ./scripts/sw-apl.sh --mode "$mode" --no-echo -f "$f" 2>&1 || true)"
        if grep -Eq "$errors" <<<"$out"; then
            echo "  FAIL $f in ($m): an error report while loading"
            grep -E "$errors" <<<"$out" | head -5 | sed 's/^/    /'
            status=1
        else
            echo "  ok   $f loads in ($m)"
        fi
        wide="$(awk 'length($0) > 64 { n++ } END { print n+0 }' <<<"$out")"
        if [ "$wide" != 0 ]; then
            echo "  FAIL $f in ($m): $wide printed line(s) wider than 64 columns"
            status=1
        fi
    done
done
[ "$status" = 0 ] && echo "check-ws: every workspace loads in every mode it claims, within 64 columns"
exit "$status"
