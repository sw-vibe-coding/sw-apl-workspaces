//! The shebang cases under `oracle/cases/`: each an executable `.apl`
//! that reaches a workspace through the library directory, as the
//! CLI and the demo do, which the in-process tests do not exercise.
//! Run from the repository root, each must exit 0 and report no
//! error.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The repository root, three above the crate; this file uses nothing
/// else of the common module, so it does not pull it in.
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

#[test]
fn every_case_runs_clean() {
    let dir = root().join("oracle").join("cases");
    let mut names: Vec<_> = fs::read_dir(&dir)
        .expect("cases")
        .flatten()
        .map(|e| e.path())
        .collect();
    names.sort();
    assert!(!names.is_empty(), "no cases under {}", dir.display());
    for case in names {
        let output = Command::new(&case)
            .current_dir(root())
            .output()
            .unwrap_or_else(|e| panic!("{}: {e}", case.display()));
        let text = String::from_utf8_lossy(&output.stdout);
        let errored = text.lines().any(|l| {
            l.ends_with(" ERROR") || l == "INCORRECT COMMAND" || l.starts_with("WS NOT FOUND")
        });
        assert!(
            output.status.success() && !errored,
            "{}\n{text}",
            case.display()
        );
    }
}
