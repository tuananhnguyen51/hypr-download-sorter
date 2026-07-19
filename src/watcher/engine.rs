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
                tracing::debug!("received {} events", events.len());
                self.debounce.push(events);
            }

            let ready = self.debounce.ready();

            tracing::debug!("ready {} files", ready.len());

            for path in ready {
                tracing::debug!("processing = {}", path);

                if !self.stability.wait_until_stable(&path).await? {
                    tracing::debug!("not stable yet: {}", path);
                    continue;
                }

                tracing::debug!("stable: {}", path);

                match self.pipeline.process(path).await {
                    Ok(_) => tracing::info!("pipeline ok"),
                    Err(err) => {
                        tracing::error!("pipeline failed: {:#?}", err);
                    }
                }
            }

            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
}
