⍝ CALC: numerical calculus on a function named by a character of
⍝ FNS or given as a polynomial. Simpson is exact for X*2 and X*3, so
⍝ their integrals over 0 to 1 are a third and a quarter; the sine
⍝ over 0 to pi is 2, the exponential over 0 to 1 is e less 1. X*2
⍝ tabulated at 0 0.5 1 1.5 2 integrates to 8÷3. The derivative of
⍝ the sine at 0 and of the exponential at 0 is 1, of X*2 at 3 is 6.
⍝ The root of X*2-2 is the root of 2, of the cosine near 1 is half
⍝ pi, of the sine between 3 and 4 is pi; X*2+1 has no root to
⍝ bracket. The series reach the primitives as the tolerance falls.
)LOAD 2 CALC
DESCRIBE
'S' AT 1
1 0 0 AT 1 2 3
0 1 10 INTEG 1 0 0
0 1 4 INTEG 1 0 0 0
(0,(○1),20) INTEG 'S'
0 1 100 INTEG '*'
0.5 INTEGV 0 0.25 1 2.25 4
'S' DERIV 0
'*' DERIV 0
1 0 0 DERIV 3
1 0 ¯2 NEWTON 1
'C' NEWTON 1
1 0 ¯2 BISECT 0 2
'S' BISECT 3 4
1 0 1 BISECT 0 2
SERIES 1E¯6
SERIES 1E¯10
HOWCALC
)OFF
