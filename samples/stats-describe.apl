⍝ STATS, the descriptive functions, on 2 4 4 4 5 5 7 9. Worked by
⍝ hand: the sum is 40 over 8, so the mean is 5; sorted, the middle
⍝ two are 4 and 5, so the median is 4.5; 4 occurs three times, the
⍝ mode; the range is 9-2. The deviations are ¯3 ¯1 ¯1 ¯1 0 0 2 4,
⍝ their squares sum to 32: the population variance is 32÷8, 4, and
⍝ its root 2; the sample variance is 32÷7, 4.571428571, and its root
⍝ 2.138089935. Quartiles at positions 1+0.25×7, 2.75, and 6.25 in
⍝ the sorted vector: 4 (between two 4s) and 5+0.25×2, 5.5. Ranks:
⍝ the three 4s share places 2 3 4, so 3; the two 5s share 5 6, so
⍝ 5.5. Four bins of width 1.75 from 2: 2 alone in the first, the 4s
⍝ and 5s in the second, 7 in the third, 9 in the last.
)LOAD 2 STATS
DESCRIBE
V←2 4 4 4 5 5 7 9
MEAN V
MEDIAN V
MODE V
RANGE V
VAR V
SD V
PVAR V
PSD V
0.25 0.5 0.75 QUANTILE V
0 1 QUANTILE V
SUMMARY V
ZSCORE V
RANK V
FREQ V
4 HIST V
⍝ A skewed set with a repeated non-integer: the mean is pulled to 4
⍝ by the 10, the median and mode stay at 2.5. Two modes when two
⍝ values tie for most frequent, in the order first met.
W←1.5 2.5 2.5 3.5 10
MEAN W
MEDIAN W
MODE W
MODE 1 2 2 3 3
FREQ W
3 HIST W
⍝ Edges and ends: an odd count has one middle value; a set of equal
⍝ values has one bin; a matrix is all its elements; a hundred
⍝ counting numbers fall twenty to a bin; a bin over sixty is scaled.
MEDIAN 3 1 2
RANK 3 1 2 3
1 HIST 7 7 7
MEAN 2 2⍴1 2 3 4
5 HIST ⍳100
2 HIST 200⍴1 1 2
HOWQUANTILE
HOWRANK
)OFF
