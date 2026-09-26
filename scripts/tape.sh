#!/usr/bin/env bash
# Render docs/tapes/NAME.tape with vhs against a local sw-apl service
# that has ws/ as library 2, then convert the gif to an animated webp
# beside it. Needs vhs, ttyd, ffmpeg and gif2webp, and sw-apl's
# release binaries (SW_APL_DIR, else ../sw-apl).
#
# Usage: scripts/tape.sh plot
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
name="${1:-plot}"
dir="${SW_APL_DIR:-../sw-apl}"
server="$dir/target/release/sw-apl-server"
export APLTERM="$dir/target/release/aplterm"
[ -x "$server" ] && [ -x "$APLTERM" ] || { echo "tape.sh: build sw-apl's release binaries first" >&2; exit 2; }
mkdir -p target/tape/work
# Ports of the tape's own, so a service the owner already has running
# on the usual ones is neither disturbed nor recorded by mistake.
export APLTERM_ADDR=127.0.0.1:2761
"$server" --mode 70 --library target/tape --lib "2=$PWD/ws,EXTENDED" \
    --listen "$APLTERM_ADDR" --http 127.0.0.1:8761 >target/tape/server.log 2>&1 &
pid=$!
trap 'kill "$pid" 2>/dev/null || true' EXIT
sleep 1
vhs "docs/tapes/$name.tape"
gif2webp -min_size "images/$name.gif" -o "images/$name.webp"
ls -la "images/$name.gif" "images/$name.webp"
