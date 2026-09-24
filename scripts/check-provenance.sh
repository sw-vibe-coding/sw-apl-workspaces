#!/usr/bin/env bash
# Every workspace tracked in ws/ must say it is ours, be named for
# the modes its modes line claims, and carry the random link a clear
# workspace starts from.
#
# The point of the SOURCE line is not the line; it is what committing
# without it would mean. The historical workspaces this library is in
# the spirit of -- APLCOURSE, TYPEDRILL, STATPAK and the rest of
# APL\360's library 1 -- are IBM or STSC material of unclear
# copyright, and converting one is easy enough that it could land in
# ws/ by accident. This gate makes that a deliberate false claim
# rather than an oversight. Anything from elsewhere belongs in work/,
# which is gitignored.
#
# `)SAVE` deliberately does NOT write the SOURCE line. A mark a program
# stamps on everything asserts nothing. It does move the random link,
# which is why the link is put back by hand and checked here: a
# transcript that uses ? is reproducible from )LOAD only while every
# tracked file's link is the clear workspace's, 16807.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
mark='⍝!SOURCE sw-apl-workspaces'
status=0
tracked="$(git ls-files 'ws/**/*.apl.ws' 'ws/*.apl.ws')"
for f in $tracked; do
    if grep -qF "$mark" "$f"; then
        echo "  ok   $f"
    else
        echo "  FAIL $f: no '$mark' line"
        status=1
    fi
done
# A workspace's file name says the modes it runs in, so a list of the
# files shows it: NAME.a-70.apl.ws for (A) only, NAME.b-75.apl.ws for
# (B) only, NAME.apl.ws for both. The modes line inside is the
# authority, and the two must agree, or sw-apl lists a workspace in a
# mode its name denies.
for f in $tracked; do
    case "$f" in
        *.a-70.apl.ws) want='(A)' ;;
        *.b-75.apl.ws) want='(B)' ;;
        *) want='(A)(B)' ;;
    esac
    have="$(grep -m1 '^⍝!MODES ' "$f" | cut -d' ' -f2- || true)"
    if [ "$have" = "$want" ]; then
        echo "  ok   $f runs in $want"
    else
        echo "  FAIL $f: named for $want, but its modes line says '${have:-nothing}'"
        status=1
    fi
done
for f in $tracked; do
    link="$(grep -m1 '^⍝!LINK ' "$f" | cut -d' ' -f2 || true)"
    if [ "$link" = "16807" ]; then
        echo "  ok   $f link 16807"
    else
        echo "  FAIL $f: random link is '${link:-missing}', not 16807 (put it back after )SAVE)"
        status=1
    fi
done
# An untracked workspace under ws/ is the mistake this guards against
# one step earlier: it is on its way to being added.
untracked="$(git ls-files --others --exclude-standard 'ws/**/*.apl.ws' 'ws/*.apl.ws')"
if [ -n "$untracked" ]; then
    echo "  FAIL untracked workspaces under ws/ (material from elsewhere belongs in work/):"
    echo "$untracked" | sed 's/^/    /'
    status=1
fi
[ "$status" = 0 ] && echo "check-provenance: ws/ is ours, named for its modes, and reproducible"
exit "$status"
