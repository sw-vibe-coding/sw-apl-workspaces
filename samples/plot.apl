⍝ PLOT: pictures in characters. A straight line, a sine wave over six
⍝ radians joined into a curve, a parabola at a lower HEIGHT, the same
⍝ points given in an unsorted X both joined and alone, a bar chart,
⍝ and the edges: a constant Y, which is one row, and bars of nothing.
)LOAD 2 PLOT
DESCRIBE
PLOT 2×⍳10
PLOT 1○(⍳60)÷10
HEIGHT←8
PLOT (⍳15)*2
5 3 1 4 2 GRAPH 25 9 1 16 4
5 3 1 4 2 SCATTER 25 9 1 16 4
BAR 3 1 4 1 5 9 2 6
PLOT 7 7 7
BAR 0 0
HOWPLOT
)OFF
