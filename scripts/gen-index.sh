#!/usr/bin/env bash
# Write ws/library.json: the index a browser reads to find this
# library, one entry per workspace file with its name, its file and
# the modes its modes line names. sw-apl's page fetches the index,
# then the files it lists, and hands them to the session as its
# library 1 is handed to it (sw-apl plan, Phase 10, browser-libraries).
#
# The field names are this repository's proposal until that step
# settles them; changing them is this script and nothing else. The
# index is tracked, so what is published is what was committed, and
# `just gates` fails when it is stale.
#
# Usage: scripts/gen-index.sh            write ws/library.json
#        scripts/gen-index.sh --check    fail if it is not current
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
out=ws/library.json
render() {
    local entries=()
    shopt -s nullglob
    for f in ws/*.apl.ws; do
        local base name modes
        base="$(basename "$f")"
        name="${base%%.*}"
        modes="$(grep -m1 '^⍝!MODES ' "$f" | cut -d' ' -f2- || true)"
        [ -z "$modes" ] && modes='(A)'
        entries+=("  {\"name\": \"$name\", \"file\": \"$base\", \"modes\": \"$modes\"}")
    done
    echo '{"name": "EXTENDED",'
    if [ "${#entries[@]}" = 0 ]; then
        echo ' "workspaces": []}'
        return
    fi
    echo ' "workspaces": ['
    local i
    for i in "${!entries[@]}"; do
        if [ "$i" -lt $(( ${#entries[@]} - 1 )) ]; then echo "${entries[$i]},"; else echo "${entries[$i]}"; fi
    done
    echo ' ]}'
}
if [ "${1:-}" = "--check" ]; then
    if [ -f "$out" ] && diff -q <(render) "$out" >/dev/null; then
        echo "  ok   $out is current"
    else
        echo "  FAIL $out is stale: run scripts/gen-index.sh"
        exit 1
    fi
else
    render > "$out"
    echo "Wrote $out"
fi
