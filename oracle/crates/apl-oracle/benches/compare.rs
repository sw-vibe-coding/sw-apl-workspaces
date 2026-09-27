//! The same subject timed two ways: through the in-process sw-apl
//! session, one line evaluated in a workspace loaded outside the
//! timed loop, and through the oracle's Rust reference on the same
//! data. Rust wins everywhere; the number worth having is the ratio
//! per subject, which says where an APL idiom costs little (an array
//! primitive that is one loop inside sw-apl) and where it costs much
//! (a loop written in APL, or the N-by-N sieve). `just bench`.

use std::hint::black_box;
use std::path::{Path, PathBuf};

use apl_oracle::{Apl, Mode, apl as lit};
use criterion::{Criterion, criterion_group, criterion_main};
use nalgebra::{DMatrix, DVector};
use statrs::distribution::{ChiSquared, ContinuousCDF, StudentsT};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn session(mode: Mode, name: &str) -> Apl {
    let mut apl = Apl::new(mode, &root());
    apl.load(name).expect("load");
    apl
}

/// A thousand whole numbers, the same every run.
fn thousand() -> Vec<f64> {
    (0..1000).map(|i| f64::from(i * 7919 % 41 - 20)).collect()
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn sd(v: &[f64]) -> f64 {
    let m = mean(v);
    (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (v.len() - 1) as f64).sqrt()
}

fn vector_primitives(c: &mut Criterion) {
    let v = thousand();
    let mut a = session(Mode::A, "STATS");
    let mut b = session(Mode::B, "STATS");
    a.eval(&format!("V←{}", lit(&v))).expect("V");
    b.eval(&format!("V←{}", lit(&v))).expect("V");
    let mut g = c.benchmark_group("mean-of-1000");
    g.bench_function("apl-a", |t| t.iter(|| a.numbers("MEAN V").expect("MEAN")));
    g.bench_function("apl-b", |t| t.iter(|| b.numbers("MEAN V").expect("MEAN")));
    g.bench_function("rust", |t| t.iter(|| mean(black_box(&v))));
    g.finish();
    let mut g = c.benchmark_group("sd-of-1000");
    g.bench_function("apl-a", |t| t.iter(|| a.numbers("SD V").expect("SD")));
    g.bench_function("rust", |t| t.iter(|| sd(black_box(&v))));
    g.finish();
}

fn factors(mut n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut d = 2;
    while n > 1 {
        while n.is_multiple_of(d) {
            out.push(d);
            n /= d;
        }
        d += 1;
    }
    out
}

fn collatz(mut n: u64) -> usize {
    let mut steps = 1;
    while n != 1 {
        n = if n.is_multiple_of(2) {
            n / 2
        } else {
            3 * n + 1
        };
        steps += 1;
    }
    steps
}

fn apl_loops(c: &mut Criterion) {
    let mut a = session(Mode::A, "MATH");
    let mut g = c.benchmark_group("factors-of-360360");
    g.bench_function("apl-a", |t| {
        t.iter(|| a.numbers("FACTORS 360360").expect("FACTORS"))
    });
    g.bench_function("rust", |t| t.iter(|| factors(black_box(360_360))));
    g.finish();
    let mut g = c.benchmark_group("gcd-1071-462");
    g.bench_function("apl-a", |t| {
        t.iter(|| a.numbers("1071 GCD 462").expect("GCD"))
    });
    g.bench_function("rust", |t| {
        t.iter(|| num_integer::gcd(black_box(1071u64), 462))
    });
    g.finish();
    let mut g = c.benchmark_group("collatz-27");
    g.bench_function("apl-a", |t| {
        t.iter(|| a.numbers("COLLATZ 27").expect("COLLATZ"))
    });
    g.bench_function("rust", |t| t.iter(|| collatz(black_box(27))));
    g.finish();
}

fn sieve(n: u64) -> Vec<u64> {
    let mut is = vec![true; (n + 1) as usize];
    let mut out = Vec::new();
    for k in 2..=n as usize {
        if is[k] {
            out.push(k as u64);
            let mut m = k * k;
            while m <= n as usize {
                is[m] = false;
                m += k;
            }
        }
    }
    out
}

fn sieves(c: &mut Criterion) {
    let mut a = session(Mode::A, "MATH");
    for n in [100u64, 300] {
        let mut g = c.benchmark_group(format!("primes-to-{n}"));
        let line = format!("PRIMES {n}");
        g.bench_function("apl-a", |t| t.iter(|| a.numbers(&line).expect("PRIMES")));
        g.bench_function("rust", |t| t.iter(|| sieve(black_box(n))));
        g.finish();
    }
}

fn fit(x: &[f64], y: &[f64]) -> Vec<f64> {
    let a = DMatrix::from_fn(x.len(), 2, |i, j| if j == 0 { 1.0 } else { x[i] });
    let b = DVector::from_column_slice(y);
    a.svd(true, true)
        .solve(&b, 1e-12)
        .expect("fit")
        .iter()
        .copied()
        .collect()
}

fn domino(c: &mut Criterion) {
    let x: Vec<f64> = (0..200).map(f64::from).collect();
    let y: Vec<f64> = x
        .iter()
        .map(|v| 3.0 + 0.5 * v + f64::from((*v as i32 * 31 % 7) - 3))
        .collect();
    let mut a = session(Mode::A, "STATS");
    let mut b = session(Mode::B, "STATS");
    for s in [&mut a, &mut b] {
        s.eval(&format!("X←{}", lit(&x))).expect("X");
        s.eval(&format!("Y←{}", lit(&y))).expect("Y");
    }
    let mut g = c.benchmark_group("regress-200-points");
    g.bench_function("apl-a", |t| {
        t.iter(|| a.numbers("X REGRESS Y").expect("REGRESS"))
    });
    g.bench_function("apl-b", |t| {
        t.iter(|| b.numbers("X REGRESS Y").expect("REGRESS"))
    });
    g.bench_function("rust", |t| t.iter(|| fit(black_box(&x), black_box(&y))));
    g.finish();
    // Diagonally dominant, so it is far from singular in every mode.
    let cells: Vec<f64> = (0..25)
        .map(|i| f64::from(i * 37 % 11 - 5) + if i % 6 == 0 { 30.0 } else { 0.0 })
        .collect();
    let m = DMatrix::from_row_slice(5, 5, &cells);
    let mut a = session(Mode::A, "MATRIX");
    a.eval(&format!("M←5 5⍴{}", lit(&cells))).expect("M");
    let mut g = c.benchmark_group("det-5x5");
    g.bench_function("apl-a", |t| t.iter(|| a.numbers("DET M").expect("DET")));
    g.bench_function("rust", |t| t.iter(|| black_box(&m).determinant()));
    g.finish();
    let mut g = c.benchmark_group("inv-5x5");
    g.bench_function("apl-a", |t| t.iter(|| a.eval("INV M").expect("INV")));
    g.bench_function("rust", |t| t.iter(|| black_box(&m).clone().try_inverse()));
    g.finish();
}

fn distributions(c: &mut Criterion) {
    let mut a = session(Mode::A, "STATS");
    let t10 = StudentsT::new(0.0, 1.0, 10.0).expect("t");
    let chi3 = ChiSquared::new(3.0).expect("chi");
    let mut g = c.benchmark_group("tprob-2-on-10");
    g.bench_function("apl-a", |t| {
        t.iter(|| a.numbers("2 TPROB 10").expect("TPROB"))
    });
    g.bench_function("rust", |t| t.iter(|| 2.0 * (1.0 - t10.cdf(black_box(2.0)))));
    g.finish();
    let mut g = c.benchmark_group("chiprob-5-on-3");
    g.bench_function("apl-a", |t| {
        t.iter(|| a.numbers("5 CHIPROB 3").expect("CHIPROB"))
    });
    g.bench_function("rust", |t| t.iter(|| 1.0 - chi3.cdf(black_box(5.0))));
    g.finish();
}

fn horner(p: &[f64], x: f64) -> f64 {
    p.iter().fold(0.0, |acc, c| acc * x + c)
}

/// Real roots by the same plan as PROOTS: sample, bisect each sign
/// change; Newton is left out, the quartic having simple roots.
fn roots(p: &[f64]) -> Vec<f64> {
    let bound = 1.0 + p[1..].iter().map(|c| (c / p[0]).abs()).fold(0.0, f64::max);
    let xs: Vec<f64> = (0..=400)
        .map(|i| bound * (f64::from(i) - 200.0) / 200.0)
        .collect();
    let ys: Vec<f64> = xs.iter().map(|x| horner(p, *x)).collect();
    let mut out = Vec::new();
    for i in 0..400 {
        if ys[i] * ys[i + 1] < 0.0 {
            let (mut lo, mut hi) = (xs[i], xs[i + 1]);
            for _ in 0..60 {
                let mid = (lo + hi) / 2.0;
                if horner(p, mid).signum() == horner(p, lo).signum() {
                    lo = mid
                } else {
                    hi = mid
                }
            }
            out.push((lo + hi) / 2.0);
        }
    }
    out
}

fn polynomial_roots(c: &mut Criterion) {
    let p = [2.0, -14.0, -12.0, 224.0, -320.0];
    let mut a = session(Mode::A, "POLY");
    let line = format!("PROOTS {}", lit(&p));
    let mut g = c.benchmark_group("proots-quartic");
    g.bench_function("apl-a", |t| t.iter(|| a.numbers(&line).expect("PROOTS")));
    g.bench_function("rust", |t| t.iter(|| roots(black_box(&p))));
    g.finish();
}

criterion_group!(
    compare,
    vector_primitives,
    apl_loops,
    sieves,
    domino,
    distributions,
    polynomial_roots
);
criterion_main!(compare);
