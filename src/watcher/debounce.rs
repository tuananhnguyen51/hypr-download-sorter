use std::collections::HashMap;
use std::time::{Duration, Instant};

use camino::Utf8PathBuf;

use super::event::FileEvent;

#[derive(Debug)]
pub struct Debouncer {
    pending: HashMap<Utf8PathBuf, Instant>,
    delay: Duration,
}

impl Debouncer {
    #[must_use]
    pub fn new(delay: Duration) -> Self {
        Self {
            pending: HashMap::new(),
            delay,
        }
    }

    /// Add incoming filesystem events.
    ///
    /// Every new event resets the timer.
    pub fn push(&mut self, events: Vec<FileEvent>) {
        let now = Instant::now();

        for event in events {
            self.pending.insert(event.path, now);
        }
    }

    /// Return files that have not changed
    /// during the debounce interval.
    pub fn ready(&mut self) -> Vec<Utf8PathBuf> {
        let now = Instant::now();

        let ready = self
            .pending
            .iter()
            .filter_map(|(path, instant)| {
                if now.duration_since(*instant) >= self.delay {
                    Some(path.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for path in &ready {
            self.pending.remove(path);
        }

        ready
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    pub fn clear(&mut self) {
        self.pending.clear();
    }
}

impl Default for Debouncer {
    fn default() -> Self {
        Self::new(Duration::from_secs(1))
    }
}
