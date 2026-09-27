//! Between APL's printed numbers and `f64`, both ways.

/// One printed number: a high minus is a sign, in the mantissa or
/// the exponent.
#[must_use]
pub fn number(token: &str) -> Option<f64> {
    token.replace('¯', "-").parse().ok()
}

/// Every number in `lines`, in reading order; words that are not
/// numbers are skipped.
#[must_use]
pub fn numbers(lines: &[String]) -> Vec<f64> {
    lines
        .iter()
        .flat_map(|l| l.split_whitespace())
        .filter_map(number)
        .collect()
}

/// The numbers of each non-empty line, a row each: a matrix as APL
/// printed it.
#[must_use]
pub fn rows(lines: &[String]) -> Vec<Vec<f64>> {
    lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.split_whitespace().filter_map(number).collect())
        .collect()
}

/// A vector as APL reads it: blanks between, a high minus for a
/// negative, and no exponent, since Rust's shortest form has none.
#[must_use]
pub fn apl(values: &[f64]) -> String {
    let words: Vec<String> = values
        .iter()
        .map(|v| format!("{v}").replace('-', "¯"))
        .collect();
    words.join(" ")
}
