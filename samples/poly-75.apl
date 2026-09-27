⍝!MODES (B)
⍝ POLY: polynomials as coefficient vectors, highest power first.
⍝ P is X*2-3X+2, which is (X-1)(X-2): it is 2 0 0 2 at 0 1 2 3 and
⍝ 72 at 10. Adding 5 gives X*2-3X+7; adding its negation gives 0.
⍝ Dividing by X-1 leaves X-2 exactly; X*3+1 over X+1 is X*2-X+1
⍝ exactly, and X*2+1 over X+1 is X-1 remainder 2. The derivative is
⍝ 2X-3, whose integral is X*2-3X. The roots are 1 and 2; X*2-2 has
⍝ plus and minus the root of 2; the cubic with roots 1 2 3 is
⍝ X*3-6X*2+11X-6; X*2+1 has none; (X-1)*2 has 1 once.
)LOAD 2 POLY
DESCRIBE
P←1 ¯3 2
P PEVAL 0 1 2 3
P PEVAL 10
P PADD 0 0 5
P PADD ¯1 3 ¯2
1 ¯1 PMUL 1 ¯2
P PDIV 1 ¯1
P PREM 1 ¯1
1 0 0 1 PDIV 1 1
1 0 0 1 PREM 1 1
1 0 1 PDIV 1 1
1 0 1 PREM 1 1
PDERIV P
PINT PDERIV P
PROOTS P
PROOTS 1 0 ¯2
PROOTS 1 ¯6 11 ¯6
PROOTS 1 0 1
PROOTS 1 ¯2 1
PSHOW 2 0 ¯3 1
PSHOW P
PSHOW 1.25 ¯2.5 3
PSHOW ¯1 0 0
PSHOW 0 0
HOWPOLY
)OFF
