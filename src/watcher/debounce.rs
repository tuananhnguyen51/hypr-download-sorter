use std::collections::HashSet;

use camino::Utf8PathBuf;

use super::event::FileEvent;

/// Simple event debouncer.
///
/// Current implementation only removes duplicated paths.
/// A time-based debounce will be added in a later commit.
#[derive(Debug, Default)]
pub struct Debouncer {
    pending: HashSet<Utf8PathBuf>,
}

impl Debouncer {
    /// Create a new debouncer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            pending: HashSet::new(),
        }
    }

    /// Push events into the debouncer.
    ///
    /// Duplicate file paths are ignored.
    pub fn push(&mut self, events: Vec<FileEvent>) {
        for event in events {
            self.pending.insert(event.path);
        }
    }

    /// Take all pending paths.
    pub fn flush(&mut self) -> Vec<Utf8PathBuf> {
        self.pending.drain().collect()
    }

    /// Check if there are pending events.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    /// Clear all pending events.
    pub fn clear(&mut self) {
        self.pending.clear();
    }
}
