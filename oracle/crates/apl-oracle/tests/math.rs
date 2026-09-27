//! MATH against `num-integer` and definitions written here. Whole
//! numbers throughout, so every comparison is exact.

mod common;

use apl_oracle::{Mode, apl as lit, rows};
use common::{check, load};
use num_integer::Integer;
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

const EXACT: f64 = 0.0;

fn is_prime(n: u64) -> bool {
    n >= 2
        && (2..n)
            .take_while(|d| d * d <= n)
            .all(|d| !n.is_multiple_of(d))
}

fn sieve(n: u64) -> Vec<f64> {
    (2..=n).filter(|k| is_prime(*k)).map(|k| k as f64).collect()
}

fn factors(mut n: u64) -> Vec<f64> {
    let mut out = Vec::new();
    let mut d = 2;
    while n > 1 {
        while n.is_multiple_of(d) {
            out.push(d as f64);
            n /= d;
        }
        d += 1;
    }
    out
}

fn totient(n: u64) -> f64 {
    (1..=n).filter(|k| k.gcd(&n) == 1).count() as f64
}

#[test]
fn primes_and_factors() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = load(mode, "MATH");
        let mut rng = ChaCha8Rng::seed_from_u64(11);
        for _ in 0..40 {
            let n = rng.random_range(2..=150);
            check(&mut apl, &format!("PRIMES {n}"), &sieve(n), EXACT);
            check(
                &mut apl,
                &format!("ISPRIME {n}"),
                &[f64::from(u8::from(is_prime(n)))],
                EXACT,
            );
            check(&mut apl, &format!("FACTORS {n}"), &factors(n), EXACT);
            let divisors: Vec<f64> = (1..=n).filter(|d| n % d == 0).map(|d| d as f64).collect();
            check(&mut apl, &format!("DIVISORS {n}"), &divisors, EXACT);
            check(&mut apl, &format!("TOTIENT {n}"), &[totient(n)], EXACT);
        }
    }
}

#[test]
fn gcd_lcm_and_sequences() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = load(mode, "MATH");
        let mut rng = ChaCha8Rng::seed_from_u64(12);
        for _ in 0..40 {
            let (a, b): (u64, u64) = (rng.random_range(1..=500), rng.random_range(1..=500));
            check(
                &mut apl,
                &format!("{a} GCD {b}"),
                &[a.gcd(&b) as f64],
                EXACT,
            );
            check(
                &mut apl,
                &format!("{a} LCM {b}"),
                &[a.lcm(&b) as f64],
                EXACT,
            );
            let n = rng.random_range(1..=30);
            let mut fib = vec![1.0, 1.0];
            while fib.len() < n {
                fib.push(fib[fib.len() - 1] + fib[fib.len() - 2]);
            }
            fib.truncate(n);
            check(&mut apl, &format!("FIB {n}"), &fib, EXACT);
            let start = rng.random_range(1..=200);
            let mut collatz = vec![start as f64];
            let mut k = start;
            while k != 1 {
                k = if k % 2 == 0 { k / 2 } else { 3 * k + 1 };
                collatz.push(k as f64);
            }
            check(&mut apl, &format!("COLLATZ {start}"), &collatz, EXACT);
        }
    }
}

fn binomial(n: u64, k: u64) -> f64 {
    (0..k)
        .fold(1.0, |acc, i| acc * (n - i) as f64 / (i + 1) as f64)
        .round()
}

#[test]
fn pascal_perms_and_combs() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = load(mode, "MATH");
        for n in 0..=8u64 {
            let want: Vec<Vec<f64>> = (0..=n)
                .map(|i| {
                    (0..=n)
                        .map(|j| if j <= i { binomial(i, j) } else { 0.0 })
                        .collect()
                })
                .collect();
            assert_eq!(
                rows(&apl.eval(&format!("PASCAL {n}")).expect("PASCAL")),
                want,
                "PASCAL {n}"
            );
        }
        for n in 1..=5u64 {
            let perms = rows(&apl.eval(&format!("PERMS {n}")).expect("PERMS"));
            let mut sorted = perms.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).expect("order"));
            sorted.dedup();
            assert_eq!(
                perms.len(),
                (1..=n).product::<u64>() as usize,
                "PERMS {n} rows"
            );
            assert_eq!(sorted.len(), perms.len(), "PERMS {n} distinct");
            for row in &perms {
                let mut r = row.clone();
                r.sort_by(|a, b| a.partial_cmp(b).expect("order"));
                assert_eq!(
                    r,
                    (1..=n).map(|k| k as f64).collect::<Vec<_>>(),
                    "PERMS {n} row {row:?}"
                );
            }
        }
        for n in 1..=7u64 {
            for k in 1..=n {
                let combs = rows(&apl.eval(&format!("{k} COMBS {n}")).expect("COMBS"));
                assert_eq!(combs.len(), binomial(n, k) as usize, "{k} COMBS {n} rows");
                let mut sorted = combs.clone();
                sorted.dedup();
                assert_eq!(sorted, combs, "{k} COMBS {n} in order and distinct");
                assert!(
                    combs.iter().all(|r| r.windows(2).all(|w| w[0] < w[1])
                        && r.iter().all(|x| *x >= 1.0 && *x <= n as f64)),
                    "{k} COMBS {n} rows ascending"
                );
            }
        }
    }
}

fn roman(mut n: u32) -> String {
    let table = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut out = String::new();
    for (value, symbol) in table {
        while n >= value {
            out.push_str(symbol);
            n -= value;
        }
    }
    out
}

#[test]
fn bases_digits_and_roman() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = load(mode, "MATH");
        let mut rng = ChaCha8Rng::seed_from_u64(13);
        for _ in 0..60 {
            let base: u64 = rng.random_range(2..=16);
            let n: u64 = rng.random_range(0..=100_000);
            let mut digits = Vec::new();
            let mut k = n;
            loop {
                digits.push((k % base) as f64);
                k /= base;
                if k == 0 {
                    break;
                }
            }
            digits.reverse();
            check(&mut apl, &format!("{base} BASE {n}"), &digits, EXACT);
            check(
                &mut apl,
                &format!("{base} UNBASE {}", lit(&digits)),
                &[n as f64],
                EXACT,
            );
            if base == 10 {
                check(&mut apl, &format!("DIGITS {n}"), &digits, EXACT);
            }
        }
        for n in 1..=3999u32 {
            let got = apl.eval(&format!("ROMAN {n}")).expect("ROMAN").join("");
            assert_eq!(got.trim(), roman(n), "ROMAN {n}");
        }
    }
}
