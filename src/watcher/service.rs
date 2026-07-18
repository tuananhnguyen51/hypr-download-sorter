use std::path::Path;
use std::sync::mpsc::{Receiver, channel};

use camino::Utf8Path;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

use crate::{AppError, Result};

use super::{convert::convert_event, event::FileEvent};

/// Filesystem watching service.
#[derive(Debug)]
pub struct WatchService {
    watcher: RecommendedWatcher,
    receiver: Receiver<notify::Result<Event>>,
}

impl WatchService {
    /// Create a new watcher service.
    pub fn new() -> Result<Self> {
        let (tx, rx) = channel();

        let watcher = RecommendedWatcher::new(
            move |event| {
                let _ = tx.send(event);
            },
            Config::default(),
        )?;

        Ok(Self {
            watcher,
            receiver: rx,
        })
    }

    /// Start watching a directory.
    pub fn watch(&mut self, path: &Utf8Path) -> Result<()> {
        self.watcher
            .watch(Path::new(path.as_str()), RecursiveMode::NonRecursive)?;

        Ok(())
    }

    /// Receive filesystem events.
    pub fn recv(&self) -> Result<Vec<FileEvent>> {
        match self.receiver.recv() {
            Ok(result) => {
                let event = result?;
                Ok(convert_event(event))
            }

            Err(err) => Err(AppError::message(err.to_string())),
        }
    }
}
