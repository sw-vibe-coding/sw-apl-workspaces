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

sw-apl caught up the same afternoon: library 2 (a718f19) and the
quad-input replies (a8294dd). Two Phase 5 steps moved into this saga
on the strength of it: the shim went, every sample loads from
library 2, and lesson 1 now has the reader try the system commands
at the pause itself.

Lessons 7 to 10 (step 6): matrices, the two products, grade, roll
and deal, encode and decode, and characters. The roll and the deal
appear in a pinned transcript because `)LOAD` restores the file's
random link, which is the reason the gate holds it at 16807.

Lessons 11 to 14 (step 7): functions, branching, input and output,
errors. A del cannot be opened at a quad prompt, so every lesson
gained section starts -- `LESSON N S` -- and lesson 11 sends the
reader to the prompt to define a function and back to its second
section. The example functions ship in the workspace, so they can be
called at the pauses, and lesson 14 shows a failing function both
ways: abandoned at a pause, suspended at the prompt. The owner also
settled that this repository publishes nothing of its own: sw-apl's
demo reads it from GitHub, and the Pages workflow went.

Lessons 15 and 16 (step 8): workspaces, tried at the pauses since
every command but a load runs there, and the two modes, each half
tried in its own mode and an error in the other. Then the
read-through, `course-all`, in each mode: Milestone 1.

## drill (2026-09-25)

Phase 2 of `plan.md`: DRILL. Step 1 built the tables and the
generator, and found the check that makes the tables trustworthy:
in (B) the sample evaluates each exercise's characters with execute
and compares with what the branch tables computed, and a hundred of
a hundred agree. (A) gets the same exercises, since the sequence is
the random link's.

Step 2 built the drill: the topic menu, the three levels, the
marking from COURSE, the worked-out answer after three wrong, and a
line of explanation per glyph. Composed exercises brought two rules
the singles did not need: the outer operand conforms to the inner
value, and a residue, a division, a power or a logical function may
be inner but not outer, where the inner value may be anything.
