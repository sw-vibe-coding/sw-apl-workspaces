⍝ COURSE lessons 11 to 14: defining functions, branching, input and
⍝ output, errors. A definition cannot be typed at a pause, so lesson
⍝ 11 sends the reader to the prompt and LESSON 11 2 brings them back
⍝ to its second section. The example functions come with the
⍝ workspace; lesson 14's OOPS is left suspended at the prompt on
⍝ purpose, and resumed.
)LOAD 2 COURSE
EXAMPLES
LESSON 11
STOP
∇R←CUBE X
R←X×X×X
∇
CUBE 3
CUBE 1 2 3
LESSON 11 2
SQUARE 7
3 HYP 4
HELLO
GO
MEAN 2 4 6
N
GO
∇MEAN[⎕]∇
∇MEAN[2]R←(+/V)÷⍴V
∇
MEAN 2 4 6
QUIZ 11
1 4 9
13
'N'
1
MEAN 2 4 9
NEXT
COUNT 3
GO
FACT 5
GO
SUMLOOP 1 2 3 4
+/1 2 3 4
GO
QUIZ 12
1 2
24
0
15
3
NEXT
ASK
5
2×ASK
2+3
GO
GREET
ADA
GO
3×⎕←2+2
''
'SUM: ';+/⍳10
GO
GUESS
1
2
QUIZ 13
'⍞'
5
2
3
12
NEXT
1 2+1 2 3
2÷0
XYZ+1
GO
GO
OOPS
)SI
GO
OOPS
)SI
→3
)SI
QUIZ 14
'LENGTH ERROR'
'DOMAIN ERROR'
'VALUE ERROR'
')SI'
'RANK ERROR'
)OFF
