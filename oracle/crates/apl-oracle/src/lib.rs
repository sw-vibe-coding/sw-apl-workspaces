//! The oracle's way into sw-apl: a session run in process, with this
//! repository's `ws/` as library 2, a console that feeds lines from a
//! queue, and a reader of the numbers APL prints. The checks
//! themselves are the integration tests in `tests/`, one file per
//! workspace, each against a reference that shares nothing with the
//! APL: `statrs`, `nalgebra`, or a closed form.

mod console;
mod parse;
mod session;

pub use apl_session::Mode;
pub use console::Fed;
pub use parse::{apl, number, numbers, rows};
pub use session::Apl;
