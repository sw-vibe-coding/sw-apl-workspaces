⍝ COURSE lessons 7 to 10: matrices, products, order and counting,
⍝ characters. Each lesson is read with its pauses answered by GO, a
⍝ few of them trying what the lesson showed, and each quiz is taken
⍝ with one wrong answer among the right ones. The roll and the deal
⍝ come back the same because a loaded workspace starts its random
⍝ sequence where the file says.
)LOAD 2 COURSE
LESSON 7
M←3 4⍴⍳12
M[1 3;2 4]
GO
+⌿M
GO
GO
QUIZ 7
2 3
4
6 15
5 7 8
3 2
NEXT
(⍳5)∘.×⍳5
GO
(⍳4)∘.≥⍳4
GO
(2 3⍴⍳6)+.×3 2⍴⍳6
GO
QUIZ 8
32
2 3
6
0
1 2+.=1 3
NEXT
V←31 4 15 9
V[⍋V]
GO
?6 6
5?10
GO
24 60 60⊤3661
GO
QUIZ 9
2 3 1
2 5 8
5
1 1 0
2 2
NEXT
'IT''S'
GO
(~'HELLO'∊'AEIOU')/'HELLO'
GO
NAMES←3 4⍴'ANN BOB CARL'
NAMES[2;]
GO
QUIZ 10
3
'N'
+/'BANANA'='A'
'LPA'
1 0 1
⍝ Lesson 11 is not written yet.
NEXT
)OFF
