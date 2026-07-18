use std::time::Duration;

use camino::Utf8Path;
use tokio::time::sleep;

use crate::Result;

#[derive(Debug, Clone)]
pub struct StabilityChecker {
    interval: Duration,
    checks: u8,
}

impl StabilityChecker {
    #[must_use]
    pub fn new(interval: Duration, checks: u8) -> Self {
        Self { interval, checks }
    }

    pub async fn wait_until_stable(&self, path: &Utf8Path) -> Result<bool> {
        let mut previous_size = None;

        for _ in 0..self.checks {
            let metadata = tokio::fs::metadata(path).await?;

            let size = metadata.len();

            if let Some(old_size) = previous_size
                && old_size == size
            {
                return Ok(true);
            }

            previous_size = Some(size);

            sleep(self.interval).await;
        }

        Ok(false)
    }
}

impl Default for StabilityChecker {
    fn default() -> Self {
        Self {
            interval: Duration::from_millis(500),
            checks: 2,
        }
    }
}
