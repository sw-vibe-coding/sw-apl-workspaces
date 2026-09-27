//! CALC against closed forms. Integrals of polynomials against their
//! antiderivatives, of the table functions against the primitives;
//! derivatives against the analytic ones; roots against known ones;
//! the series against the primitives within the tolerance asked, or
//! within the 5E¯10 that ten printed digits allow when that is looser.
//! Simpson is exact for a cubic and compared at `REL`; a derivative
//! by central difference over 1E¯5 is good to about 1E¯8, so `DERIV`;
//! Newton and bisection reach `ROOT`.

mod common;

use apl_oracle::{Mode, apl as lit};
use common::{check, load};
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

const REL: f64 = 1e-8;
const DERIV: f64 = 1e-6;
const ROOT: f64 = 1e-8;

fn horner(p: &[f64], x: f64) -> f64 {
    p.iter().fold(0.0, |acc, c| acc * x + c)
}

fn antiderivative(p: &[f64]) -> Vec<f64> {
    let n = p.len();
    p.iter()
        .enumerate()
        .map(|(i, c)| c / (n - i) as f64)
        .chain([0.0])
        .collect()
}

fn derivative(p: &[f64]) -> Vec<f64> {
    let n = p.len();
    p[..n - 1]
        .iter()
        .enumerate()
        .map(|(i, c)| c * (n - 1 - i) as f64)
        .collect()
}

#[test]
fn integrals() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = load(mode, "CALC");
        let mut rng = ChaCha8Rng::seed_from_u64(41);
        for _ in 0..30 {
            let p: Vec<f64> = (0..rng.random_range(1..=4))
                .map(|_| f64::from(rng.random_range(-5..=5)))
                .collect();
            let (a, b) = (
                f64::from(rng.random_range(-3..=0)),
                f64::from(rng.random_range(1..=4)),
            );
            let f = antiderivative(&p);
            let want = horner(&f, b) - horner(&f, a);
            check(
                &mut apl,
                &format!("{} INTEG {}", lit(&[a, b, 10.0]), lit(&p)),
                &[want],
                REL,
            );
        }
        check(&mut apl, "(0,(○1),200) INTEG 'S'", &[2.0], 1e-8);
        check(
            &mut apl,
            "0 1 200 INTEG '*'",
            &[std::f64::consts::E - 1.0],
            1e-8,
        );
        check(
            &mut apl,
            "1 2 200 INTEG '÷'",
            &[std::f64::consts::LN_2],
            1e-8,
        );
        check(&mut apl, "0.5 INTEGV 0 0.25 1 2.25 4", &[8.0 / 3.0], REL);
        check(&mut apl, "1 INTEGV 0 1 2 3", &[4.5], REL);
    }
}

#[test]
fn derivatives_and_roots() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = load(mode, "CALC");
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        for _ in 0..30 {
            let p: Vec<f64> = (0..rng.random_range(2..=4))
                .map(|_| f64::from(rng.random_range(-5..=5)))
                .collect();
            let x = f64::from(rng.random_range(-4..=4));
            check(
                &mut apl,
                &format!("{} DERIV {}", lit(&p), lit(&[x])),
                &[horner(&derivative(&p), x)],
                DERIV,
            );
        }
        for x in [-2.0, -0.5, 0.0, 0.7, 2.0] {
            check(
                &mut apl,
                &format!("'S' DERIV {}", lit(&[x])),
                &[x.cos()],
                DERIV,
            );
            check(
                &mut apl,
                &format!("'*' DERIV {}", lit(&[x])),
                &[x.exp()],
                DERIV,
            );
        }
        for _ in 0..30 {
            let r = f64::from(rng.random_range(-6..=6));
            let s = r + f64::from(rng.random_range(1..=5));
            let p = [1.0, -(r + s), r * s];
            let start = r - 0.4;
            check(
                &mut apl,
                &format!("{} NEWTON {}", lit(&p), lit(&[start])),
                &[r],
                ROOT,
            );
            check(
                &mut apl,
                &format!("{} BISECT {}", lit(&p), lit(&[r - 0.7, r + 0.4])),
                &[r],
                ROOT,
            );
        }
        check(
            &mut apl,
            "'C' NEWTON 1",
            &[std::f64::consts::FRAC_PI_2],
            ROOT,
        );
        check(&mut apl, "'S' BISECT 3 4", &[std::f64::consts::PI], ROOT);
    }
}

#[test]
fn series_reach_the_primitives() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = load(mode, "CALC");
        for t in [1e-3_f64, 1e-6, 1e-10] {
            // Ten printed digits allow about 5E¯10 whatever T asks.
            let shown = t.max(2e-9);
            let e = apl
                .numbers(&format!("ESERIES {}", lit(&[t])))
                .expect("ESERIES");
            assert!(
                (e[0] - std::f64::consts::E).abs() < shown,
                "ESERIES {t}: {e:?}"
            );
            let pi = apl
                .numbers(&format!("PISERIES {}", lit(&[t])))
                .expect("PISERIES");
            assert!(
                (pi[0] - std::f64::consts::PI).abs() < 2.0 * shown,
                "PISERIES {t}: {pi:?}"
            );
            for x in [0.5, 1.0, 2.0] {
                let line = format!("{} SINSERIES {}", lit(&[x]), lit(&[t]));
                let s = apl.numbers(&line).expect("SINSERIES");
                assert!((s[0] - x.sin()).abs() < shown, "{line}: {s:?}");
            }
        }
    }
}
