//! POLY against direct computation: Horner for evaluation, the
//! definitions for sum, product, derivative and integral, the
//! division identity for quotient and remainder, and known roots for
//! PROOTS. Coefficients are whole numbers, so arithmetic is exact and
//! compared at `REL`; roots are found by Newton and compared at
//! `ROOT`, 1E¯5, since a root is rounded to six places.

mod common;

use apl_oracle::{Mode, apl as lit};
use common::{check, load};
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

const REL: f64 = 1e-8;
const ROOT: f64 = 1e-5;

/// Highest power first, as POLY writes them.
fn horner(p: &[f64], x: f64) -> f64 {
    p.iter().fold(0.0, |acc, c| acc * x + c)
}

fn trim(p: &[f64]) -> Vec<f64> {
    let first = p.iter().position(|c| *c != 0.0).unwrap_or(p.len() - 1);
    p[first..].to_vec()
}

fn multiply(p: &[f64], q: &[f64]) -> Vec<f64> {
    let mut out = vec![0.0; p.len() + q.len() - 1];
    for (i, a) in p.iter().enumerate() {
        for (j, b) in q.iter().enumerate() {
            out[i + j] += a * b;
        }
    }
    trim(&out)
}

fn poly(rng: &mut ChaCha8Rng) -> Vec<f64> {
    let n = rng.random_range(1..=5);
    let mut p: Vec<f64> = (0..n)
        .map(|_| f64::from(rng.random_range(-6..=6)))
        .collect();
    if p[0] == 0.0 {
        p[0] = 1.0;
    }
    p
}

#[test]
fn evaluate_add_multiply() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = load(mode, "POLY");
        let mut rng = ChaCha8Rng::seed_from_u64(31);
        for _ in 0..40 {
            let (p, q) = (poly(&mut rng), poly(&mut rng));
            let xs: Vec<f64> = (-3..=3).map(f64::from).collect();
            let want: Vec<f64> = xs.iter().map(|x| horner(&p, *x)).collect();
            check(
                &mut apl,
                &format!("{} PEVAL {}", lit(&p), lit(&xs)),
                &want,
                REL,
            );
            let n = p.len().max(q.len());
            let pad = |v: &[f64]| [vec![0.0; n - v.len()], v.to_vec()].concat();
            let sum: Vec<f64> = pad(&p).iter().zip(pad(&q)).map(|(a, b)| a + b).collect();
            check(
                &mut apl,
                &format!("{} PADD {}", lit(&p), lit(&q)),
                &trim(&sum),
                REL,
            );
            check(
                &mut apl,
                &format!("{} PMUL {}", lit(&p), lit(&q)),
                &multiply(&p, &q),
                REL,
            );
        }
    }
}

#[test]
fn divide_differentiate_integrate() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = load(mode, "POLY");
        let mut rng = ChaCha8Rng::seed_from_u64(32);
        for _ in 0..40 {
            let (p, q) = (poly(&mut rng), poly(&mut rng));
            if q.len() > p.len() {
                continue;
            }
            let quotient = apl
                .numbers(&format!("{} PDIV {}", lit(&p), lit(&q)))
                .expect("PDIV");
            let remainder = apl
                .numbers(&format!("{} PREM {}", lit(&p), lit(&q)))
                .expect("PREM");
            assert!(
                remainder.len() < q.len() || remainder == [0.0],
                "PREM degree: {p:?} by {q:?} gave {remainder:?}"
            );
            for x in [-2.0, -0.5, 0.0, 1.0, 3.0] {
                let back = horner(&q, x) * horner(&quotient, x) + horner(&remainder, x);
                assert!(
                    (back - horner(&p, x)).abs() <= 1e-6 * horner(&p, x).abs().max(1.0),
                    "{p:?} = {q:?} × {quotient:?} + {remainder:?} fails at {x}"
                );
            }
            let n = p.len();
            let deriv: Vec<f64> = p[..n - 1]
                .iter()
                .enumerate()
                .map(|(i, c)| c * (n - 1 - i) as f64)
                .collect();
            check(
                &mut apl,
                &format!("PDERIV {}", lit(&p)),
                &trim(if deriv.is_empty() { &[0.0] } else { &deriv }),
                REL,
            );
            let integ: Vec<f64> = p
                .iter()
                .enumerate()
                .map(|(i, c)| c / (n - i) as f64)
                .chain([0.0])
                .collect();
            check(&mut apl, &format!("PINT {}", lit(&p)), &integ, REL);
        }
    }
}

#[test]
fn roots_known_and_absent() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = load(mode, "POLY");
        let mut rng = ChaCha8Rng::seed_from_u64(33);
        for _ in 0..30 {
            let k = rng.random_range(1..=4);
            let mut roots: Vec<f64> = Vec::new();
            while roots.len() < k {
                let r = f64::from(rng.random_range(-5..=5));
                if !roots.contains(&r) {
                    roots.push(r);
                }
            }
            let lead = f64::from(rng.random_range(1..=3));
            let p = roots
                .iter()
                .fold(vec![lead], |acc, r| multiply(&acc, &[1.0, -r]));
            roots.sort_by(f64::total_cmp);
            check(&mut apl, &format!("PROOTS {}", lit(&p)), &roots, ROOT);
        }
        check(&mut apl, "PROOTS 1 0 1", &[], ROOT);
        check(&mut apl, "PROOTS 1 0 5 0 4", &[], ROOT);
        check(&mut apl, "PROOTS 1 ¯2 1", &[1.0], ROOT);
    }
}
