⍝ sw-apl workspace. Re-executable APL: loading it runs it.
⍝!MODES (A)(B)
⍝!SOURCE sw-apl-workspaces -- written for this repository. MIT, (c) 2026 Michael A Wright.
⍝!ORIGIN 1
⍝!DIGITS 10
⍝!WIDTH 64
⍝!LINK 16807
)WSID EIGENCUBE
⍝ Small geometric kernel for the Eigencube demonstration. Points are
⍝ rows of a matrix; rotations are exact integer quarter-turn matrices.

∇DESCRIBE
'EIGENCUBE -- SMALL 3-D LINEAR-ALGEBRA KERNEL'
''
'ROT X       QUARTER-TURN MATRIX ABOUT X.'
'ROT Y       QUARTER-TURN MATRIX ABOUT Y.'
'ROT Z       QUARTER-TURN MATRIX ABOUT Z.'
'GEOTURN M P APPLY MATRIX M TO POINTS P.'
'CUBE        THE 26 NON-CENTRE CUBELET COORDINATES.'
'GEOM        SHOW THE SOLVED CUBELET COORDINATES.'
''
'THIS IS THE GEOMETRIC CORE USED BY RUBIK; ITS STATE IS NOT MOVE HISTORY.'
∇

∇R←ROT A
⍝ A is X, Y or Z. Exact quarter-turn matrices avoid floating point drift.
R←3 3⍴0
→('X'=A)/X
→('Y'=A)/Y
R←3 3⍴1 0 0 0 0 1 0 ¯1 0
→0
X:R←3 3⍴0 0 1 0 1 0 ¯1 0 0
→0
Y:R←3 3⍴0 0 ¯1 0 1 0 1 0 0
∇

∇R←M GEOTURN P
R←P+.×⍉M
∇

⍝ The 26 non-centre positions, as rows (x y z). Base-3 encode
⍝ makes all 27 points; compression removes the hidden centre.
CUBE←⍉((⍳27)≠14)/¯1+3 3 3⊤¯1+⍳27

∇GEOM
'EIGENCUBE COORDINATE DEMO'
'THE FULL CUBELET MODEL HAS 26 NON-CENTRE PIECES.'
CUBE
∇
