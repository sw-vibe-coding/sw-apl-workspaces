---
description: "Checkpoint: gates, docs, commit, push, CHANGES.md refresh"
---

# /mw-cp -- checkpoint process

Run the full pre-commit gate for the work in the current session,
then commit and push it. Order: gates -> docs -> commit -> push ->
CHANGES.md refresh -> report. If this session is an agentrail
step, `agentrail complete` comes AFTER the commit lands (the step
records HEAD), and nothing may change after `complete`.

## 1. Discover scope

```bash
git status --short
git diff --stat
```

List the workspaces under `ws/` that changed, and the samples that
load them. Everything in steps 2 and 3 is scoped to that list.

## 2. Gates (repo-wide, fast)

```bash
just gates
```

- `sw-markdown-checker` on README.md and every other `.md` outside
  `docs/`. `docs/*.md` may contain glyphs and is not gated. Known
  non-blocking failure: the agentrail-managed block in CLAUDE.md
  contains em dashes emitted by `agentrail instructions apply` (fix
  belongs upstream); everything you hand-edited must pass.
- `scripts/check-provenance.sh`: every tracked workspace carries the
  SOURCE line, its name agrees with its modes line, its random link
  is 16807, and nothing untracked sits under `ws/`.
- `scripts/check-ws.sh`: every workspace loads in every mode it
  claims without an error report, and every printed line fits 64
  columns.

## 3. Transcripts (scoped)

```bash
scripts/reg.sh run -q
```

reg-rs covers everything checked by running sw-apl: one test per
`samples/*.apl`, in the mode the sample's first line names. Rebase a
baseline only intentionally; name the test and the reason in the
commit message. A transcript that moves because the sw-apl binary
changed is a finding to record, not a baseline to rebase quietly.

## 4. Docs

- Update `docs/workspaces.md` when a workspace's public functions or
  conventions changed (what and how, no chronology).
- Update `docs/plan.md` when scope or ordering changed; the saga
  plan must match (`agentrail plan`).
- Add or update the workspace's `samples/*.apl` and its paragraph in
  `samples/README.md` when behaviour is transcript-visible.
- The README's workspace table says what exists.

## 5. Commit

Stage by explicit path (never `git add -A`). Include `.agentrail/`
metadata with the source commit. Message: what + why, saga and
step, which modes were run, reg-rs notes, and the trailer:

```
Co-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>
```

Never `--no-verify`. If a hook fails, fix the cause.

## 6. Push

```bash
git push
```

If pushing is impossible, say so explicitly in the handoff; never
leave commits silently stranded. Non-fast-forward on a personal
branch: `git pull --rebase`, never force-push shared branches.

## 7. CHANGES.md refresh

```bash
./scripts/gen-changes.sh
```

Commit as `docs(changes): refresh CHANGES.md to HEAD` (or fold
into the following agentrail-complete commit), then push.

## 8. Report

Tell the owner: what was pushed (commits, files, which modes ran),
the next step(s) from the saga, and any blockers.

## Never

- NEVER run `sw-install` as part of this flow.
- Never edit the sw-apl repository from a step here; a gap in the
  interpreter is reported to its plan with the sample that shows it.
- Never rebase a baseline to make a wrong transcript pass.
