# sw-apl-workspaces Saga Log

Long-form notes per saga. `CHANGES.md` is the per-commit log;
`.agentrail/` holds the live saga; `.agentrail-archive/` holds
finished ones.

## course (2026-09-24)

Phases 0 and 1 of `plan.md`: the process scaffold, then COURSE. The
repository was bootstrapped in one commit with the process carried
over from sw-apl -- agentrail saga derived from the plan, the `/mw-cp`
checkpoint, gates in place of a build -- and step 1 added the index a
browser reads, the Pages workflow, and the documents the README
names. COURSE follows, lesson by lesson.

COURSE's shape (step 2): DESCRIBE, CONTENTS, START, LESSON, NEXT and
QUIZ, with a pause between sections read by quote-quad and answers
read by quad, STOP recognised at either kind of prompt without
execute, so that the same file runs in (A) and (B). Lesson 1 and its
quiz are the model the other lessons follow.

Lessons 2 to 6 (step 3) changed the pause: quote-quad could not
evaluate what a reader tried, so the pause reads with quad and
prints whatever is typed until GO or STOP, and every local on the
way to it became a delta name so the reader's own names are not
shadowed. Two interpreter findings went to plan.md.
