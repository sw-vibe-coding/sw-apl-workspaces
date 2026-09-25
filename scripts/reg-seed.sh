#!/usr/bin/env bash
# Seed one reg-rs transcript test per samples/*.apl. Existing tests
# are skipped; rebase intentionally with scripts/reg.sh rebase.
#
# Every test runs scripts/run-sample.sh NAME, which picks the mode
# from the sample's first line, gives sw-apl ws/ as library 2 and a
# fresh library 0 of the sample's own, and runs through the normalize
# filter, so a sample may print a value that cannot come back the
# same by labelling it (VARIES).
#
# Usage: scripts/reg-seed.sh [pattern]
#        scripts/reg-seed.sh --all      recreate every test
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export REG_RS_DATA_DIR="$PWD/tests/reg-rs"
mkdir -p "$REG_RS_DATA_DIR"
pattern="${1:-}"
recreate=""
[ "$pattern" = "--all" ] && { recreate="yes"; pattern=""; }
filter="bash scripts/normalize-apl-output.sh"
shopt -s nullglob
for f in samples/*.apl; do
    base="$(basename "$f" .apl)"
    [ -n "$pattern" ] && [[ "$base" != *"$pattern"* ]] && continue
    name="ws-sample-$base"
    if [ -f "$REG_RS_DATA_DIR/$name.rgt" ]; then
        [ -z "$recreate" ] && { echo "  skip $name"; continue; }
        # The pattern is a substring of the file name; the dot keeps
        # course-1 from also removing course-1-75.
        reg-rs remove -p "$name." >/dev/null
    fi
    echo "  create $name"
    reg-rs create -t "$name" \
        -c "scripts/run-sample.sh $base" \
        --timeout 120 --desc "Transcript of $f" --preprocess "$filter"
done
reg-rs run -q && echo "ALL PASS" || echo "SOME FAILURES"
