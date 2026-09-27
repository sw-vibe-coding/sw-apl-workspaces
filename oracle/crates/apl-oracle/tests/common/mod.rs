//! What every oracle test needs: a session with a workspace loaded,
//! a tolerance, and a check that names the APL line when it fails.

use std::path::{Path, PathBuf};

use apl_oracle::{Apl, Mode};

/// The repository root, three above the crate.
#[must_use]
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// A session in `mode` with `)LOAD 2 NAME` done.
#[must_use]
pub fn load(mode: Mode, name: &str) -> Apl {
    let mut apl = Apl::new(mode, &root());
    apl.load(name).unwrap_or_else(|e| panic!("{e}"));
    apl
}

/// Within `tol` relative, with a floor of `tol` absolute for values
/// near zero.
#[must_use]
pub fn close(got: f64, want: f64, tol: f64) -> bool {
    (got - want).abs() <= tol * got.abs().max(want.abs()).max(1.0)
}

/// `line` prints exactly the numbers `want`, each within `tol`; the
/// message names the line and both sides.
pub fn check(apl: &mut Apl, line: &str, want: &[f64], tol: f64) {
    let got = apl
        .numbers(line)
        .unwrap_or_else(|e| panic!("APL error:\n{e}"));
    let same = got.len() == want.len() && got.iter().zip(want).all(|(g, w)| close(*g, *w, tol));
    assert!(same, "{line}\n  APL:  {got:?}\n  want: {want:?}");
}
