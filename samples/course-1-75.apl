⍝!MODES (B)
⍝ COURSE: the shape of the course, and lesson 1. Loaded from library
⍝ 1 through the shim library, until sw-apl has a numbered library of
⍝ its own and this says )LOAD 2. DESCRIBE is the greeting, CONTENTS
⍝ the list, START the first lesson.
)LOAD 1 COURSE
DESCRIBE
CONTENTS
⍝ Lesson 1, every pause answered with an empty line.
START




⍝ The quiz: a right answer, a wrong one, an expression that makes the
⍝ right one, and STOP, which leaves without a score.
QUIZ 1
4
1
1+1
STOP
⍝ The whole quiz, a vector where a scalar was wanted.
QUIZ 1
4
0
2
,16
⍝ NEXT goes on from the last lesson read; lesson 2 is not written.
NEXT
⍝ STOP at a pause leaves a lesson.
LESSON 1
STOP
⍝ A quiz for a lesson not yet written, and a lesson that is not one.
QUIZ 3
LESSON 17
)OFF
