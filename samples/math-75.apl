⍝!MODES (B)
⍝ MATH: number theory and combinatorics. Worked by hand: 360 is
⍝ 2×2×2×3×3×5; the divisors of 28 sum to 28, a perfect number; 48
⍝ and 18 share 6, so their least common multiple is 48×18÷6, 144;
⍝ of 1 to 36 the numbers coprime to 36 are 36×(1-1÷2)×(1-1÷3), 12,
⍝ and a prime P has P-1; PERMS 3 has !3 rows, 2 COMBS 4 has 2!4;
⍝ 13 is 1101 in binary and 255 is 15 15 in hexadecimal; 1984 is
⍝ MCMLXXXIV; 6 goes 3 10 5 16 8 4 2 1.
)LOAD 2 MATH
DESCRIBE
PRIMES 50
ISPRIME 97
ISPRIME 91
FACTORS 360
DIVISORS 28
48 GCD 18
48 LCM 18
TOTIENT 36
TOTIENT 97
FIB 10
PASCAL 5
PERMS 3
⍴PERMS 5
2 COMBS 4
3 COMBS 5
2 BASE 13
2 UNBASE 1 1 0 1
16 BASE 255
DIGITS 1984
ROMAN 1984
ROMAN 2024
COLLATZ 6
⍝ Edges: 1 is not prime and has no prime factors; its totient is 1;
⍝ 0 has one digit; the sequence from 27 is a long one.
ISPRIME 1
FACTORS 1
TOTIENT 1
2 BASE 0
⍴COLLATZ 27
HOWPERMS
HOWBASE
)OFF
