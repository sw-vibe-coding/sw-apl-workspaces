#!/usr/bin/env bash
# Every line of output in every pinned sample transcript is at most 64
# characters, (B)'s clear-workspace width, so nothing a workspace
# prints wraps on the narrower mode. An echoed input line -- six
# spaces of indent, or a bracketed line number from the del editor --
# is the sample's own and is not counted; a sample's comments may run
# longer than what APL prints.
#
# Characters, not bytes: perl -CSD reads UTF-8 as characters, where
# awk's length would count a glyph as three.
#
# Usage: scripts/check-width.sh
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
status=0
shopt -s nullglob
for f in tests/reg-rs/ws-sample-*.out; do
    if perl -CSD -ne '
        next if /^      / or /^\[\d+\]/;
        if (length($_) - 1 > 64) { printf "  FAIL %s:%d: %d characters\n", $ARGV, $., length($_) - 1; $bad = 1 }
        END { exit($bad ? 1 : 0) }' "$f"; then
        :
    else
        status=1
    fi
done
[ "$status" = 0 ] && echo "check-width: every pinned transcript prints within 64 columns"
exit "$status"
