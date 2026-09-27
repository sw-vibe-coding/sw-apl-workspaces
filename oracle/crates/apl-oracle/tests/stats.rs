//! STATS against references that share nothing with it: closed forms
//! written here, `nalgebra` for least squares, `statrs` for the
//! distributions. Every case runs in (A) and in (B).
//!
//! Tolerances. STATS prints ten significant digits, so a value read
//! back carries a relative error of up to 5E¯10 before any
//! arithmetic; `REL`, 1E¯8 relative with a floor of 1E¯8 absolute,
//! covers that and the rounding of a few operations. The
//! distributions are compared at `DIST`, 1E¯8 absolute, since STATS's
//! continued fractions stop at 1E¯12 and `statrs` is good to machine
//! precision; NORMAL is a rational approximation good to 7.5E¯8 and is
//! compared at `APPROX`.

mod common;

use apl_oracle::{Apl, Mode, apl as lit, rows};
use common::{check, load};
use nalgebra::{DMatrix, DVector};
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use statrs::distribution::{ChiSquared, ContinuousCDF, Normal, StudentsT};

const REL: f64 = 1e-8;
const DIST: f64 = 1e-8;
const APPROX: f64 = 7.5e-8;
const CASES: usize = 40;

fn stats(mode: Mode) -> Apl {
    load(mode, "STATS")
}

fn ints(rng: &mut ChaCha8Rng, n: usize, lo: i32, hi: i32) -> Vec<f64> {
    (0..n)
        .map(|_| f64::from(rng.random_range(lo..=hi)))
        .collect()
}

/// A vector of 2 to 12 whole numbers that is not constant.
fn sample(rng: &mut ChaCha8Rng) -> Vec<f64> {
    loop {
        let n = rng.random_range(2..=12);
        let v = ints(rng, n, -20, 20);
        if v.iter().any(|x| *x != v[0]) {
            return v;
        }
    }
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn var(v: &[f64], population: bool) -> f64 {
    let m = mean(v);
    let ss: f64 = v.iter().map(|x| (x - m) * (x - m)).sum();
    ss / (v.len() - usize::from(!population)) as f64
}

fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    let n = s.len();
    (s[(n - 1) / 2] + s[n / 2]) / 2.0
}

/// Hyndman and Fan's type 7.
fn quantile(v: &[f64], p: f64) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    let h = p * (s.len() - 1) as f64;
    let lo = h.floor() as usize;
    let hi = h.ceil() as usize;
    s[lo] + (h - h.floor()) * (s[hi] - s[lo])
}

/// Average ranks for ties.
fn ranks(v: &[f64]) -> Vec<f64> {
    v.iter()
        .map(|x| {
            let below = v.iter().filter(|y| **y < *x).count() as f64;
            let equal = v.iter().filter(|y| **y == *x).count() as f64;
            below + (equal + 1.0) / 2.0
        })
        .collect()
}

fn modes(v: &[f64]) -> Vec<f64> {
    let count = |x: f64| v.iter().filter(|y| **y == x).count();
    let most = v.iter().map(|x| count(*x)).max().unwrap_or(0);
    let mut seen = Vec::new();
    for x in v {
        if count(*x) == most && !seen.contains(x) {
            seen.push(*x);
        }
    }
    seen
}

#[test]
fn descriptives() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = stats(mode);
        let mut rng = ChaCha8Rng::seed_from_u64(1);
        for _ in 0..CASES {
            let v = sample(&mut rng);
            let t = lit(&v);
            check(&mut apl, &format!("MEAN {t}"), &[mean(&v)], REL);
            check(&mut apl, &format!("MEDIAN {t}"), &[median(&v)], REL);
            check(&mut apl, &format!("MODE {t}"), &modes(&v), REL);
            check(
                &mut apl,
                &format!("RANGE {t}"),
                &[v.iter().cloned().fold(f64::MIN, f64::max)
                    - v.iter().cloned().fold(f64::MAX, f64::min)],
                REL,
            );
            check(&mut apl, &format!("VAR {t}"), &[var(&v, false)], REL);
            check(&mut apl, &format!("SD {t}"), &[var(&v, false).sqrt()], REL);
            check(&mut apl, &format!("PVAR {t}"), &[var(&v, true)], REL);
            check(&mut apl, &format!("PSD {t}"), &[var(&v, true).sqrt()], REL);
            check(&mut apl, &format!("RANK {t}"), &ranks(&v), REL);
            let z: Vec<f64> = v
                .iter()
                .map(|x| (x - mean(&v)) / var(&v, false).sqrt())
                .collect();
            check(&mut apl, &format!("ZSCORE {t}"), &z, REL);
        }
    }
}

#[test]
fn quantiles_and_frequencies() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = stats(mode);
        let mut rng = ChaCha8Rng::seed_from_u64(2);
        for _ in 0..CASES {
            let v = sample(&mut rng);
            let t = lit(&v);
            let ps = [0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0];
            let want: Vec<f64> = ps.iter().map(|p| quantile(&v, *p)).collect();
            check(&mut apl, &format!("{} QUANTILE {t}", lit(&ps)), &want, REL);
            let table = rows(&apl.eval(&format!("FREQ {t}")).expect("FREQ"));
            let mut distinct: Vec<f64> = v.clone();
            distinct.sort_by(f64::total_cmp);
            distinct.dedup();
            let want: Vec<Vec<f64>> = distinct
                .iter()
                .map(|d| vec![*d, v.iter().filter(|x| *x == d).count() as f64])
                .collect();
            assert_eq!(table, want, "FREQ {t}");
        }
    }
}

/// Least squares by SVD: the intercept, then a coefficient per column.
fn fit(xs: &[Vec<f64>], y: &[f64]) -> Vec<f64> {
    let n = y.len();
    let k = xs.len();
    let a = DMatrix::from_fn(n, k + 1, |i, j| if j == 0 { 1.0 } else { xs[j - 1][i] });
    let b = DVector::from_column_slice(y);
    a.svd(true, true)
        .solve(&b, 1e-12)
        .expect("least squares")
        .iter()
        .copied()
        .collect()
}

#[test]
fn two_variables() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = stats(mode);
        let mut rng = ChaCha8Rng::seed_from_u64(3);
        for _ in 0..CASES {
            let n = rng.random_range(3..=10);
            let (x, y) = loop {
                let x = ints(&mut rng, n, -9, 9);
                let y = ints(&mut rng, n, -9, 9);
                if var(&x, true) > 0.0 && var(&y, true) > 0.0 {
                    break (x, y);
                }
            };
            let (tx, ty) = (lit(&x), lit(&y));
            let cov = x
                .iter()
                .zip(&y)
                .map(|(a, b)| (a - mean(&x)) * (b - mean(&y)))
                .sum::<f64>()
                / (n - 1) as f64;
            check(&mut apl, &format!("{tx} COV {ty}"), &[cov], REL);
            check(
                &mut apl,
                &format!("{tx} CORR {ty}"),
                &[cov / (var(&x, false) * var(&y, false)).sqrt()],
                REL,
            );
            let b = fit(std::slice::from_ref(&x), &y);
            check(&mut apl, &format!("{tx} REGRESS {ty}"), &b, REL);
            let resid: Vec<f64> = x.iter().zip(&y).map(|(a, c)| c - b[0] - b[1] * a).collect();
            check(&mut apl, &format!("{tx} RESID {ty}"), &resid, 1e-6);
            let ss: f64 = resid.iter().map(|r| r * r).sum();
            let st: f64 = y.iter().map(|c| (c - mean(&y)).powi(2)).sum();
            check(&mut apl, &format!("{tx} RSQ {ty}"), &[1.0 - ss / st], 1e-6);
            check(
                &mut apl,
                &format!("({tx} REGRESS {ty}) PREDICT 2 5"),
                &[b[0] + 2.0 * b[1], b[0] + 5.0 * b[1]],
                REL,
            );
        }
    }
}

#[test]
fn two_regressors() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = stats(mode);
        let mut rng = ChaCha8Rng::seed_from_u64(4);
        for _ in 0..CASES {
            let n = rng.random_range(4..=9);
            let x1 = ints(&mut rng, n, -9, 9);
            let x2 = ints(&mut rng, n, -9, 9);
            let y = ints(&mut rng, n, -9, 9);
            let cells: Vec<f64> = x1.iter().zip(&x2).flat_map(|(a, b)| [*a, *b]).collect();
            let m = DMatrix::from_fn(n, 3, |i, j| [1.0, x1[i], x2[i]][j]);
            if m.rank(1e-9) < 3 {
                continue;
            }
            let line = format!("({n} 2⍴{}) REGRESS {}", lit(&cells), lit(&y));
            check(&mut apl, &line, &fit(&[x1.clone(), x2.clone()], &y), 1e-6);
        }
    }
}

#[test]
fn t_tests_and_chi_square() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = stats(mode);
        let mut rng = ChaCha8Rng::seed_from_u64(5);
        for _ in 0..CASES {
            let v = sample(&mut rng);
            let m = f64::from(rng.random_range(-5..=5));
            let n = v.len() as f64;
            let t = (mean(&v) - m) / (var(&v, false) / n).sqrt();
            check(
                &mut apl,
                &format!("{} TTEST1 {}", lit(&[m]), lit(&v)),
                &[t, n - 1.0],
                REL,
            );
            let w = sample(&mut rng);
            let (n1, n2) = (v.len() as f64, w.len() as f64);
            let sp = (((n1 - 1.0) * var(&v, false) + (n2 - 1.0) * var(&w, false))
                / (n1 + n2 - 2.0))
                .sqrt();
            let t2 = (mean(&v) - mean(&w)) / (sp * (1.0 / n1 + 1.0 / n2).sqrt());
            check(
                &mut apl,
                &format!("{} TTEST2 {}", lit(&v), lit(&w)),
                &[t2, n1 + n2 - 2.0],
                REL,
            );
            let (r, c) = (rng.random_range(2..=3), rng.random_range(2..=4));
            let table = ints(&mut rng, r * c, 1, 30);
            let (rs, cs, total): (Vec<f64>, Vec<f64>, f64) = (
                (0..r)
                    .map(|i| (0..c).map(|j| table[i * c + j]).sum())
                    .collect(),
                (0..c)
                    .map(|j| (0..r).map(|i| table[i * c + j]).sum())
                    .collect(),
                table.iter().sum(),
            );
            let chi: f64 = (0..r)
                .flat_map(|i| (0..c).map(move |j| (i, j)))
                .map(|(i, j)| {
                    let e = rs[i] * cs[j] / total;
                    (table[i * c + j] - e).powi(2) / e
                })
                .sum();
            check(
                &mut apl,
                &format!("CHISQ {r} {c}⍴{}", lit(&table)),
                &[chi, ((r - 1) * (c - 1)) as f64],
                REL,
            );
        }
    }
}

#[test]
fn normal_against_statrs() {
    let normal = Normal::new(0.0, 1.0).expect("normal");
    for mode in [Mode::A, Mode::B] {
        let mut apl = stats(mode);
        let zs: Vec<f64> = (-40..=40).map(|i| f64::from(i) / 10.0).collect();
        let want: Vec<f64> = zs.iter().map(|z| normal.cdf(*z)).collect();
        check(&mut apl, &format!("NORMAL {}", lit(&zs)), &want, APPROX);
        let want: Vec<f64> = zs
            .iter()
            .map(|z| 2.0 * (1.0 - normal.cdf(z.abs())))
            .collect();
        check(
            &mut apl,
            &format!("PNORM {}", lit(&zs)),
            &want,
            2.0 * APPROX,
        );
    }
}

#[test]
fn tprob_against_statrs() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = stats(mode);
        for df in [1.0, 2.0, 3.0, 5.0, 10.0, 30.0, 100.0, 500.0] {
            let t = StudentsT::new(0.0, 1.0, df).expect("t");
            for x in [0.0, 0.1, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 4.0, 6.0, 10.0, 30.0] {
                let want = 2.0 * (1.0 - t.cdf(x));
                check(
                    &mut apl,
                    &format!("{} TPROB {}", lit(&[x]), lit(&[df])),
                    &[want],
                    DIST,
                );
            }
        }
    }
}

#[test]
fn chiprob_against_statrs() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = stats(mode);
        for df in [1.0, 2.0, 3.0, 5.0, 10.0, 30.0, 100.0, 500.0] {
            let chi = ChiSquared::new(df).expect("chi-square");
            for x in [
                0.0, 0.01, 0.5, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0, 200.0, 600.0,
            ] {
                let want = 1.0 - chi.cdf(x);
                check(
                    &mut apl,
                    &format!("{} CHIPROB {}", lit(&[x]), lit(&[df])),
                    &[want],
                    DIST,
                );
            }
        }
    }
}

/// The oracle notices: a wrong expectation fails, so a pass means
/// something.
#[test]
#[should_panic(expected = "MEAN 1 2 3")]
fn a_wrong_expectation_fails() {
    let mut apl = stats(Mode::A);
    check(&mut apl, "MEAN 1 2 3", &[2.5], REL);
}
