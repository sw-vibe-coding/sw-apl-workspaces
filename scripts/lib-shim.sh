#!/usr/bin/env bash
# A library directory for sw-apl with this repository's ws/ as its
# library 1, until sw-apl can be given a numbered library of its own
# (its Phase 10 step library-config; then this is --lib 2=ws and the
# samples say )LOAD 2). sw-apl's --library DIR wants DIR/work for
# library 0 and DIR/ws/lib1 for library 1, so the shim is those two,
# with lib1 a link to ws/. Library 0 lands in target/, which is
# ignored, so a sample that )SAVEs writes nowhere that is tracked.
#
# Prints the directory, so a script can say --library "$(scripts/lib-shim.sh)".
set -euo pipefail
root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
shim="$root/target/shim"
mkdir -p "$shim/work" "$shim/ws"
ln -sfn "$root/ws" "$shim/ws/lib1"
echo "$shim"
