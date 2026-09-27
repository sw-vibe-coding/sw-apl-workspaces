//! MATRIX against `nalgebra`, on random whole-number matrices that
//! are not singular. The determinant and the trace are exact in
//! principle and compared at `REL`; an inverse and a solve are only as
//! good as the matrix's conditioning, so matrices with a condition
//! number over 1E4 are skipped and the rest are compared at `INV`.

mod common;

use apl_oracle::{Apl, Mode, apl as lit, rows};
use common::{check, close, load};
use nalgebra::DMatrix;
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

const REL: f64 = 1e-8;
const INV: f64 = 1e-6;

fn square(rng: &mut ChaCha8Rng, n: usize) -> DMatrix<f64> {
    DMatrix::from_fn(n, n, |_, _| f64::from(rng.random_range(-5..=5)))
}

/// A matrix as APL reads it: `(N N⍴...)`.
fn literal(m: &DMatrix<f64>) -> String {
    let cells: Vec<f64> = (0..m.nrows())
        .flat_map(|i| (0..m.ncols()).map(move |j| (i, j)))
        .map(|(i, j)| m[(i, j)])
        .collect();
    format!("({} {}⍴{})", m.nrows(), m.ncols(), lit(&cells))
}

fn well_conditioned(m: &DMatrix<f64>) -> bool {
    match m.clone().try_inverse() {
        Some(inv) => m.norm() * inv.norm() < 1e4,
        None => false,
    }
}

fn same_matrix(apl: &mut Apl, line: &str, want: &DMatrix<f64>, tol: f64) {
    let got = rows(&apl.eval(line).unwrap_or_else(|e| panic!("APL error:\n{e}")));
    let ok = got.len() == want.nrows()
        && got.iter().enumerate().all(|(i, r)| {
            r.len() == want.ncols()
                && r.iter()
                    .enumerate()
                    .all(|(j, g)| close(*g, want[(i, j)], tol))
        });
    assert!(ok, "{line}\n  APL:  {got:?}\n  want: {want}");
}

#[test]
fn determinant_trace_norm_power() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = load(mode, "MATRIX");
        let mut rng = ChaCha8Rng::seed_from_u64(21);
        for _ in 0..40 {
            let n = rng.random_range(1..=5);
            let m = square(&mut rng, n);
            let t = literal(&m);
            check(&mut apl, &format!("DET {t}"), &[m.determinant()], REL);
            check(&mut apl, &format!("TRACE {t}"), &[m.trace()], REL);
            check(&mut apl, &format!("NORM {t}"), &[m.norm()], REL);
            let p = rng.random_range(0..=3);
            same_matrix(&mut apl, &format!("{t} MPOW {p}"), &m.pow(p), REL);
            let sym = &m + &m.transpose();
            check(&mut apl, &format!("ISSYM {}", literal(&sym)), &[1.0], REL);
            check(
                &mut apl,
                &format!("ISSYM {t}"),
                &[f64::from(u8::from(m == m.transpose()))],
                REL,
            );
        }
        check(&mut apl, "DET 2 2⍴1 2 2 4", &[0.0], REL);
        check(&mut apl, "NORM 3 4", &[5.0], REL);
    }
}

#[test]
fn inverse_and_solve() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = load(mode, "MATRIX");
        let mut rng = ChaCha8Rng::seed_from_u64(22);
        let mut done = 0;
        while done < 40 {
            let n = rng.random_range(2..=5);
            let m = square(&mut rng, n);
            if !well_conditioned(&m) {
                continue;
            }
            done += 1;
            let t = literal(&m);
            let inv = m.clone().try_inverse().expect("inverse");
            same_matrix(&mut apl, &format!("INV {t}"), &inv, INV);
            let b: Vec<f64> = (0..n)
                .map(|_| f64::from(rng.random_range(-9..=9)))
                .collect();
            let x = &inv * DMatrix::from_column_slice(n, 1, &b);
            let want: Vec<f64> = x.iter().copied().collect();
            check(&mut apl, &format!("{t} SOLVE {}", lit(&b)), &want, INV);
            same_matrix(&mut apl, &format!("ID {n}"), &DMatrix::identity(n, n), REL);
        }
    }
}

#[test]
fn gram_schmidt() {
    for mode in [Mode::A, Mode::B] {
        let mut apl = load(mode, "MATRIX");
        let mut rng = ChaCha8Rng::seed_from_u64(23);
        let mut done = 0;
        while done < 30 {
            let (r, c) = (rng.random_range(2..=5), rng.random_range(1..=3));
            let m = DMatrix::from_fn(r, c.min(r), |_, _| f64::from(rng.random_range(-5..=5)));
            if m.rank(1e-9) < m.ncols() {
                continue;
            }
            done += 1;
            let line = format!("GRAM {}", literal(&m));
            let q = rows(
                &apl.eval(&line)
                    .unwrap_or_else(|e| panic!("APL error:\n{e}")),
            );
            let q = DMatrix::from_fn(m.nrows(), m.ncols(), |i, j| q[i][j]);
            let qtq = q.transpose() * &q;
            assert!(
                (qtq - DMatrix::identity(m.ncols(), m.ncols())).norm() < 1e-6,
                "{line}: columns not orthonormal"
            );
            let projected = &q * (q.transpose() * &m);
            assert!(
                (projected - &m).norm() < 1e-6,
                "{line}: columns do not span the originals"
            );
        }
    }
}
