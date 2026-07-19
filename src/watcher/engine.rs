use std::time::Duration;

use camino::Utf8Path;

use crate::{Result, pipeline::Pipeline};

use super::{debounce::Debouncer, service::WatchService, stability::StabilityChecker};

#[derive(Debug)]
pub struct WatchEngine {
    watcher: WatchService,
    debounce: Debouncer,
    stability: StabilityChecker,
    pipeline: Pipeline,
}

impl WatchEngine {
    pub fn new(pipeline: Pipeline) -> Result<Self> {
        Ok(Self {
            watcher: WatchService::new()?,
            debounce: Debouncer::new(Duration::from_millis(500)),
            stability: StabilityChecker::new(Duration::from_millis(300), 3),
            pipeline,
        })
    }

    pub fn watch(&mut self, path: &Utf8Path) -> Result<()> {
        self.watcher.watch(path)
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            if let Some(events) = self.watcher.try_recv()? {
                self.debounce.push(events);
            }

            for path in self.debounce.ready() {
                if !self.stability.wait_until_stable(&path).await? {
                    continue;
                }

                self.pipeline.process(path).await?;
            }

            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
}
