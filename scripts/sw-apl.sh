#!/usr/bin/env bash
# The sw-apl binary the tests run: SW_APL if set, else the release
# build in the sw-apl checkout beside this repository, else sw-apl on
# the path. Every script and recipe goes through here so that which
# interpreter ran is one decision, made in one place.
set -euo pipefail
root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
if [ -n "${SW_APL:-}" ]; then
    bin="$SW_APL"
elif [ -x "$root/../sw-apl/target/release/sw-apl" ]; then
    bin="$root/../sw-apl/target/release/sw-apl"
elif command -v sw-apl >/dev/null; then
    bin="$(command -v sw-apl)"
else
    echo "sw-apl.sh: no sw-apl binary (set SW_APL, build ../sw-apl, or install one)" >&2
    exit 2
fi
exec "$bin" "$@"
