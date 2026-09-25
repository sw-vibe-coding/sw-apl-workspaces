#!/usr/bin/env bash
# Print every sample's transcript, each run as its test runs it
# (scripts/run-sample.sh). Usage: scripts/run-samples.sh [pattern]
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
pattern="${1:-}"
shopt -s nullglob
for f in samples/*.apl; do
    base="$(basename "$f" .apl)"
    [ -n "$pattern" ] && [[ "$base" != *"$pattern"* ]] && continue
    echo "==== $base"
    ./scripts/run-sample.sh "$base" || echo "(exit $?)"
done
