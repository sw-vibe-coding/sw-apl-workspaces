⍝ MATRIX: linear algebra over domino. Worked by hand: 4 7 / 2 6 has
⍝ determinant 24-14, 10, and inverse 6 ¯7 / ¯2 4 over 10; solving
⍝ 4X+7Y=1, 2X+6Y=2 gives X ¯0.8, Y 0.6. The 3 by 3 with rows 1 2 3,
⍝ 0 1 4 and 5 6 0 has determinant 1 and the integer inverse ¯24 18 5
⍝ / 20 ¯15 ¯4 / ¯5 4 1; a matrix times its inverse is the identity
⍝ up to rounding of the order of 1E¯14, so the product is shown
⍝ rounded. 1 2 / 2 4 is singular. The Fibonacci matrix 1 1 / 1 0 to
⍝ the fifth power holds 8 5 / 5 3. The columns 1 1 0 and 1 0 1 made
⍝ orthonormal are 1 1 0 over the root of 2 and 1 ¯1 2 over the root
⍝ of 6, and their transpose times themselves is the identity.
)LOAD 2 MATRIX
DESCRIBE
ID 3
A←2 2⍴4 7 2 6
DET A
INV A
A SOLVE 1 2
A+.×A SOLVE 1 2
C←3 3⍴1 2 3 0 1 4 5 6 0
DET C
INV C
⌊0.5+C+.×INV C
TRACE C
DET 2 2⍴1 2 2 4
DET 3 3⍴0 1 2 1 0 3 4 5 6
(2 2⍴1 1 1 0) MPOW 5
(2 2⍴1 1 1 0) MPOW 0
NORM 3 4
NORM 2 2⍴1 1 1 1
ISSYM 2 2⍴1 2 2 1
ISSYM 2 2⍴1 2 3 1
ISSYM 2 3⍴1
Q←GRAM 3 2⍴1 1 1 0 0 1
Q
⌊0.5+(⍉Q)+.×Q
HOWDOMINO
HOWDET
)OFF
