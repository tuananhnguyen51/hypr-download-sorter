use camino::{Utf8Path, Utf8PathBuf};

use crate::Result;

use super::{Debouncer, StabilityChecker, WatchService};

#[derive(Debug)]
pub struct WatcherEngine {
    service: WatchService,
    debouncer: Debouncer,
    stability: StabilityChecker,
}

impl WatcherEngine {
    pub fn new() -> Result<Self> {
        Ok(Self {
            service: WatchService::new()?,
            debouncer: Debouncer::default(),
            stability: StabilityChecker::default(),
        })
    }

    pub fn watch(&mut self, path: &Utf8Path) -> Result<()> {
        self.service.watch(path)
    }

    pub async fn poll(&mut self) -> Result<Vec<Utf8PathBuf>> {
        let events = self.service.try_recv()?;

        if !events.is_empty() {
            self.debouncer.push(events);
        }

        let mut ready = Vec::new();

        for path in self.debouncer.ready() {
            if self.stability.wait_until_stable(&path).await? {
                ready.push(path);
            }
        }

        Ok(ready)
    }
}
