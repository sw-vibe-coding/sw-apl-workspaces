//! One sw-apl session with library 2, and a line at a time.

use std::path::Path;

use apl_session::{Added, Files, Host, Mode, QUOTA, Session, Source};

use crate::console::Fed;
use crate::parse::numbers;

/// A session in a mode, with `ws/` under the repository root as
/// library 2, EXTENDED, exactly as `--lib 2=ws,EXTENDED` gives it, and
/// a scratch library 0 under `target/`.
pub struct Apl {
    session: Session,
    mode: Mode,
}

impl Apl {
    /// A fresh session in `mode` for the repository at `root`.
    #[must_use]
    pub fn new(mode: Mode, root: &Path) -> Apl {
        let ws = root.join("ws");
        let scratch = root.join("target").join("oracle");
        let _ = std::fs::create_dir_all(scratch.join("work"));
        let place = ws.display().to_string();
        let store =
            Added::new(Box::new(Files(scratch))).with(2, "EXTENDED", &place, Source::Dir(ws));
        let host = Host {
            quota: QUOTA,
            store: Box::new(store),
            mode,
        };
        let session = Session::attached(Box::new(Fed::default()), host);
        Apl { session, mode }
    }

    /// `)LOAD 2 NAME`, then the print width opened to its widest, so
    /// that a matrix comes back one row a line rather than wrapped at
    /// the workspace's 64; the width is a setting the load restores,
    /// which is why it is set after. `)WIDTH` in (A), `⎕PW` in (B).
    ///
    /// # Errors
    /// The load's reply, when it reported an error.
    pub fn load(&mut self, name: &str) -> Result<(), String> {
        self.eval(&format!(")LOAD 2 {name}"))?;
        let widen = match self.mode {
            Mode::A => ")WIDTH 254",
            Mode::B => "⎕PW←254",
        };
        self.eval(widen).map(|_| ())
    }

    /// One input line: the transcript lines it printed.
    ///
    /// # Errors
    /// The line and what it printed, when it reported an error.
    pub fn eval(&mut self, line: &str) -> Result<Vec<String>, String> {
        let reply = self.session.respond(line);
        if reply.error {
            return Err(format!("{line}\n{}", reply.lines.join("\n")));
        }
        Ok(reply.lines)
    }

    /// One input line: every number it printed, in order.
    ///
    /// # Errors
    /// As `eval`.
    pub fn numbers(&mut self, line: &str) -> Result<Vec<f64>, String> {
        Ok(numbers(&self.eval(line)?))
    }
}
