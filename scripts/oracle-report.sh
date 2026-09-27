#!/usr/bin/env bash
# The oracle's verdict as a transcript reg-rs can pin: every test
# binary's result line, with the timing stripped, so a green run
# prints the same thing every time. cargo's own progress goes to
# stderr and is not part of it.
#
# Usage: scripts/oracle-report.sh
set -euo pipefail
cd "$(git rev-parse --show-toplevel)/oracle"
cargo test --workspace -q 2>/dev/null \
    | grep -E '^test result:' \
    | sed -E 's/; finished in [0-9.]+s//'
