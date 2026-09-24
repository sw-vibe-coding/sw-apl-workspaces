⍝!MODES (B)
⍝ COURSE lessons 2 to 6: numbers, vectors, comparison and logic,
⍝ reduction and scan, selection. Each lesson is read with its pauses
⍝ answered by GO, a few of them trying what the lesson showed, and
⍝ each quiz is taken with one wrong answer among the right ones.
)LOAD 1 COURSE
LESSON 2
7÷2
2×3+4
GO
¯3+5
GO
3|17
GO
QUIZ 2
14
2
3
3
2*10
LESSON 3
1 2 3+4 5 6
GO
⍴3 4 5 6 7
GO
5⍴1 2
,7
GO
QUIZ 3
⍳4
5
4 6 8
1 2 1 2
1 2 3 4
NEXT
1 2 3=2
GO
~1 0 1
GO
(1 2 3>1)×10 20 30
GO
QUIZ 4
0 0 1
0
0 1 0
1
0 20 30
NEXT
-/1 2 3
GO
+/2 4 6 8>3
GO
+\1 2 3 4
GO
QUIZ 5
10
24
1 3 6
9
+/2 4 6 8>3
NEXT
V←10 20 30 40
V[4 3 2 1]
V[2]←25
V
GO
¯1↑V
GO
(V>15)/V
1 0 1 1\1 2 3
GO
QUIZ 6
5 6
6 7 8
3 2 1
7 8 9
2
⍝ V was set at a pause, so it is still here.
V
)OFF
