//! A console that answers a read from a queue and shows nothing
//! itself: what a statement prints comes back in the `Reply`.

use std::collections::VecDeque;

use apl_session::{Console, Shown};

/// Lines waiting to be read by quad or quote-quad. Empty, a read is
/// an INTERRUPT, which is what an oracle wants: no case here should
/// ask for input.
#[derive(Debug, Default)]
pub struct Fed(pub VecDeque<String>);

impl Console for Fed {
    fn read(&mut self, _shown: &Shown, _prompt: &str) -> Option<String> {
        self.0.pop_front()
    }
}
