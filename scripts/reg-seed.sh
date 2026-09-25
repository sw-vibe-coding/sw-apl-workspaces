#!/usr/bin/env bash
# Seed one reg-rs transcript test per samples/*.apl. Existing tests
# are skipped; rebase intentionally with scripts/reg.sh rebase.
#
# A sample names its mode as a workspace does, on its first line: one
# that begins `⍝!MODES (B)` runs in (B) '75; any other runs in (A)
# '70. Every test runs sw-apl through scripts/sw-apl.sh with ws/ as
# library 2, EXTENDED (--lib, sw-apl a718f19 or later), so )LOAD 2
# NAME finds a workspace here, and with the normalize filter, so a
# sample may print a value that cannot come back the same by
# labelling it (VARIES).
#
# Usage: scripts/reg-seed.sh [pattern]
#        scripts/reg-seed.sh --all      recreate every test
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export REG_RS_DATA_DIR="$PWD/tests/reg-rs"
mkdir -p "$REG_RS_DATA_DIR"
lib="--lib 2=ws,EXTENDED"
pattern="${1:-}"
recreate=""
[ "$pattern" = "--all" ] && { recreate="yes"; pattern=""; }
filter="bash scripts/normalize-apl-output.sh"
mode_of() { head -1 "$1" | grep -q '^⍝!MODES (B)$' && echo " --mode 75" || true; }
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
        -c "scripts/sw-apl.sh$(mode_of "$f") $lib -f $f" \
        --timeout 120 --desc "Transcript of $f" --preprocess "$filter"
done
reg-rs run -q && echo "ALL PASS" || echo "SOME FAILURES"
