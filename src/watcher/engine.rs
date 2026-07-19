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
            // Nhận batch event từ notify
            let events = self.watcher.recv()?;

            // Đưa toàn bộ event vào debounce
            self.debounce.push(events);

            // Lấy các file đã hết thời gian debounce
            let ready = self.debounce.ready();

            for path in ready {
                if !self.stability.wait_until_stable(&path).await? {
                    continue;
                }

                self.pipeline.process(path).await?;
            }
        }
    }
}
