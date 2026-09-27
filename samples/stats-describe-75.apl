⍝!MODES (B)
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
⍝ Two variables. On the line Y=1+2X the fit is exact: intercept 1,
⍝ slope 2, correlation 1, and residuals that are rounding, of the
⍝ order of 1E¯15, so their sum of magnitudes is shown against a
⍝ bound rather than printed.
X←1 2 3 4 5
Y←3 5 7 9 11
X REGRESS Y
X CORR Y
X RSQ Y
1E¯10>+/|X RESID Y
(X REGRESS Y) PREDICT 6 7
⍝ With scatter, Y←2 4 5 4 5: the means are 3 and 4; the products of
⍝ deviations sum to 6 and the squares of X's to 10, so the slope is
⍝ 0.6 and the intercept 4-0.6×3, 2.2. The covariance is 6÷4, 1.5.
⍝ Y's squared deviations sum to 6, so its sample sd is 1.5*0.5 and
⍝ the correlation 6÷60*0.5, 0.7745966692. The fitted values are
⍝ 2.8 3.4 4 4.6 5.2, the residuals sum of squares 2.4, and R
⍝ squared 1-2.4÷6, 0.6, the correlation squared.
Y←2 4 5 4 5
X COV Y
X CORR Y
X REGRESS Y
X RESID Y
X RSQ Y
(X REGRESS Y) PREDICT 6
⍝ Two regressors as the columns of a matrix, Y←1+2×X1+3×X2 exactly.
X2←5 2⍴1 1 2 0 3 1 4 0 5 1
Y2←1+(2×X2[;1])+3×X2[;2]
Y2
X2 REGRESS Y2
(X2 REGRESS Y2) PREDICT 2 2⍴6 1 7 0
HOWREGRESS
⍝ Tests. One sample, 2 4 4 4 5 5 7 9 against a mean of 4: the mean
⍝ is 5, the sample sd 2.138089935, the standard error that over the
⍝ root of 8, 0.755928946, so t is 1.322875656 on 7 degrees of
⍝ freedom. Two samples, 1 2 3 4 5 and 2 4 5 4 5: means 3 and 4,
⍝ variances 2.5 and 1.5, pooled variance (4×2.5+4×1.5)÷8, 2, so t is
⍝ ¯1 over the root of 2×0.4, ¯1.118033989, on 8. A two-by-two table
⍝ 10 20 / 30 40: row totals 30 70, column totals 40 60, expected
⍝ 12 18 / 28 42, and the statistic 4÷12 plus 4÷18 plus 4÷28 plus
⍝ 4÷42, 0.7936507937, on 1.
4 TTEST1 V
1 2 3 4 5 TTEST2 2 4 5 4 5
CHISQ 2 2⍴10 20 30 40
⍝ Their probabilities: none of the three is significant. Then the
⍝ tabulated points: a t of 2 on 10 degrees of freedom is 0.0734
⍝ two-sided and 2.228 is the 5 per cent point; a chi-square of 3.841
⍝ on 1 degree and 5.991 on 2 are the 5 per cent points; on 1 degree
⍝ a t of 1 is 0.5, the Cauchy; and as the degrees grow TPROB tends to
⍝ PNORM.
R←4 TTEST1 V
R[1] TPROB R[2]
R←1 2 3 4 5 TTEST2 2 4 5 4 5
R[1] TPROB R[2]
R←CHISQ 2 2⍴10 20 30 40
R[1] CHIPROB R[2]
2 TPROB 10
2.228 TPROB 10
3.841 CHIPROB 1
5.991 CHIPROB 2
1 TPROB 1
1.96 TPROB 1000
PNORM 1.96
⍝ The normal distribution at 0, 1.96, ¯1 and 3, tabulated as 0.5,
⍝ 0.9750021, 0.1586553 and 0.9986501; the approximation is within
⍝ 7.5E¯8 of each, and prints its own last digits. PNORM 1.96 is the
⍝ familiar 0.05, less the same error.
NORMAL 0 1.96 ¯1 3
PNORM 1.96
⍝ Random. The values are the same on every run from a fresh load,
⍝ since the workspace's random link is fixed; two thousand normal
⍝ deviates have a mean near 0 and a standard deviation near 1.
RANDU 3
10 20 RANDIN 3
RANDN 3
Z←RANDN 2000
MEAN Z
SD Z
3 SAMPLE ⍳10
SHUFFLE 'APL'
HOWTEST
HOWRANDOM
)OFF
