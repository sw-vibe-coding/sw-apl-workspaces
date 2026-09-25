# sw-apl-workspaces has nothing to build: the recipes here run the
# workspaces through sw-apl and check the repository's rules. The
# binary is SW_APL if set, else the release build in the sw-apl
# checkout beside this one, else sw-apl on the path (scripts/sw-apl.sh).

default:
    @just --list

# Repo-wide standards gates. README and other top-level markdown are
# ASCII-only; docs/*.md may contain glyphs. CLAUDE.md carries an
# agentrail-managed block with em dashes (upstream), so it is checked
# only outside that block by eye.
gates:
    sw-markdown-checker -f README.md
    sw-markdown-checker -f "samples/*.md"
    ./scripts/check-provenance.sh
    ./scripts/gen-index.sh --check

# Every workspace, in every mode it claims, loads without an error
# report, and every line it prints fits 64 columns.
check:
    ./scripts/check-ws.sh

# The full pre-commit gate, in order.
precommit: gates check

# Run one workspace file through sw-apl in a mode (70 or 75) and
# print the transcript: `just run ws/COURSE.apl.ws 75`.
run file mode="70":
    ./scripts/sw-apl.sh --mode {{mode}} --lib 2=ws,EXTENDED -f {{file}}

# A session with this library as library 2, in a mode: `just apl 75`.
apl mode="70":
    ./scripts/sw-apl.sh --mode {{mode}} --lib 2=ws,EXTENDED

# Print every sample's transcript, each in the mode its first line
# names, with ws/ as library 2 so )LOAD 2 NAME finds a workspace.
samples pattern="":
    ./scripts/run-samples.sh {{pattern}}

# Transcript regressions: one reg-rs test per sample. `just reg` runs
# them; scripts/reg-seed.sh creates a test for a new sample.
reg:
    ./scripts/reg.sh run -q

# Regenerate ws/library.json, the index a browser reads to find
# this library; tracked, and gated to be current.
index:
    ./scripts/gen-index.sh

# Regenerate CHANGES.md from git log.
changes:
    ./scripts/gen-changes.sh
