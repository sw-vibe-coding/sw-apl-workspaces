#!/usr/bin/env bash
# A workspace that is the same in both modes but richer in (B) is a
# pair, NAME.a-70.apl.ws beside NAME.b-75.apl.ws, both called NAME.
# The (B) file is the source; this derives the (A) file from it, so
# the functions the two share cannot drift apart:
#
#   - the modes line (B) becomes (A);
#   - a line setting quad-LX goes, (A) having no quad names;
#   - a function whose first body line begins "⍝ (B) ONLY" goes, and
#     so does any line that is only a call of such a function.
#
# Usage: scripts/gen-pair.sh NAME          write ws/NAME.a-70.apl.ws
#        scripts/gen-pair.sh --check       every pair is current
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
names_of() {
    # The names of the (B)-only functions, as bare call lines.
    awk '
        /^∇/ && !infn { infn = 1; hdr = $0; next }
        infn && !seen { seen = 1; if (index($0, "⍝ (B) ONLY") == 1) { sub(/^∇/, "", hdr); print hdr } next }
        /^∇$/ { infn = 0; seen = 0 }
    ' "$1"
}
one() {
    local name="$1" src="ws/$1.b-75.apl.ws" dst="ws/$1.a-70.apl.ws"
    [ -f "$src" ] || { echo "gen-pair.sh: no $src" >&2; exit 2; }
    # The names go in on one line, bar-separated: this awk will not
    # take a newline inside -v.
    local calls; calls="$(names_of "$src" | paste -sd '|' -)"
    awk -v list="$calls" '
        BEGIN { split(list, a, "|"); for (k in a) if (a[k] != "") bonly[a[k]] = 1; infn = 0 }
        /^⍝!MODES \(B\)$/ { print "⍝!MODES (A)"; next }
        /^⎕LX←/ { next }
        infn == 0 && /^∇/ { infn = 1; n = 0; delete buf; buf[++n] = $0; next }
        infn == 1 {
            buf[++n] = $0
            if ($0 == "∇") {
                drop = (n >= 3 && index(buf[2], "⍝ (B) ONLY") == 1)
                if (!drop) for (i = 1; i <= n; i++) if (!(buf[i] in bonly)) print buf[i]
                infn = 0
            }
            next
        }
        !($0 in bonly) { print }
    ' "$src"
}
if [ "${1:-}" = "--check" ]; then
    status=0
    shopt -s nullglob
    for src in ws/*.b-75.apl.ws; do
        name="$(basename "$src" .b-75.apl.ws)"
        dst="ws/$name.a-70.apl.ws"
        [ -f "$dst" ] || continue
        if diff -q <(one "$name") "$dst" >/dev/null; then
            echo "  ok   $dst is derived from $src"
        else
            echo "  FAIL $dst differs from what $src derives: run scripts/gen-pair.sh $name"
            status=1
        fi
    done
    exit "$status"
fi
one "$1" > "ws/$1.a-70.apl.ws"
echo "Wrote ws/$1.a-70.apl.ws"
