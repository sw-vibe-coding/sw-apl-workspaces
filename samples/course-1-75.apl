⍝!MODES (B)
⍝ COURSE: the shape of the course, and lesson 1. This repository is
⍝ library 2, EXTENDED, given to sw-apl with --lib 2=ws,EXTENDED.
⍝ DESCRIBE is the greeting, CONTENTS the list, START the first lesson.
)LOAD 2 COURSE
DESCRIBE
CONTENTS
⍝ Lesson 1. At each pause what the section showed is tried: two
⍝ expressions, then the system commands, which run at the quad prompt
⍝ and hand it back; GO goes on.
START
2+2
3×4
GO
)DIALECT
)WSID
GO
)LIBS
)LIB 2
GO
)HELP LIBS
GO
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
⍝ NEXT goes on from the last lesson read; lesson 7 is not written.
LESSON 6
STOP
NEXT
⍝ An error at a pause is reported and the prompt comes back; STOP
⍝ leaves the lesson.
LESSON 1
1 2 3+4 5
STOP
⍝ A quiz for a lesson not yet written, and a lesson that is not one.
QUIZ 9
LESSON 17
)OFF
