use std::time::Duration;

use camino::{Utf8Path, Utf8PathBuf};

use crate::{Result, filter, pipeline::Pipeline};

use super::{debounce::Debouncer, service::WatchService, stability::StabilityChecker};

const POLL_INTERVAL: Duration = Duration::from_millis(50);
const DEBOUNCE_DELAY: Duration = Duration::from_millis(500);
const STABILITY_DELAY: Duration = Duration::from_millis(300);
const STABILITY_RETRIES: u8 = 3;

#[derive(Debug)]
pub struct WatchEngine {
    watcher: WatchService,
    debounce: Debouncer,
    stability: StabilityChecker,
    pipeline: Pipeline,
}

impl WatchEngine {
    fn poll_events(&mut self) -> Result<()> {
        if let Some(events) = self.watcher.try_recv()? {
            tracing::debug!("received {} events", events.len());
            self.debounce.push(events);
        }

        Ok(())
    }

    fn ready_paths(&mut self) -> Vec<Utf8PathBuf> {
        let ready = self.debounce.ready();

        tracing::debug!("ready {} files", ready.len());

        ready
    }

    async fn process_path(
        &self,
        path: Utf8PathBuf,
    ) -> Result<()> {
        if !filter::should_process(&path) {
            tracing::debug!("ignored {}", path);
            return Ok(());
        }

        tracing::debug!("processing {}", path);

        if !self.stability.wait_until_stable(&path).await? {
            tracing::debug!("not stable {}", path);
            return Ok(());
        }

        tracing::debug!("stable {}", path);

        self.pipeline.process(path).await
    }

    pub fn new(pipeline: Pipeline) -> Result<Self> {
        Ok(Self {
            watcher: WatchService::new()?,
            debounce: Debouncer::new(DEBOUNCE_DELAY),
            stability: StabilityChecker::new(STABILITY_DELAY,STABILITY_RETRIES),
            pipeline,
        })
    }

    pub fn watch(&mut self, path: &Utf8Path) -> Result<()> {
        self.watcher.watch(path)
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            self.poll_events()?;

            for path in self.ready_paths() {
                if let Err(err) = self.process_path(path).await {
                    tracing::error!("{:#?}", err);
                }
            }

            tokio::time::sleep(POLL_INTERVAL).await;
        }
    }
}
