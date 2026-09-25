⍝!MODES (B)
⍝ COURSE: the shape of the course, and lesson 1. This repository is
⍝ library 2, EXTENDED, given to sw-apl with --lib 2=ws,EXTENDED.
⍝ DESCRIBE is the greeting, CONTENTS the list, START the first lesson.
)LOAD 2 COURSE
DESCRIBE
CONTENTS
⍝ Lesson 1. At the first pause two expressions are tried and printed;
⍝ GO goes on from every pause.
START
2+2
3×4
GO
GO
GO
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
⍝ An error at a pause stops the lesson with its report; a bare branch
⍝ clears it, and the lesson can be read again.
LESSON 1
1 2 3+4 5
→
LESSON 1
STOP
⍝ A quiz for a lesson not yet written, and a lesson that is not one.
QUIZ 9
LESSON 17
)OFF
