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
    pub const fn new(interval: Duration, checks: u8) -> Self {
        Self { interval, checks }
    }

    async fn file_size(path: &Utf8Path) -> Result<u64> {
        Ok(tokio::fs::metadata(path).await?.len())
    }

    fn is_stable(previous: Option<u64>, current: u64) -> bool {
        matches!(previous, Some(size) if size == current)
    }

    pub async fn wait_until_stable(
        &self,
        path: &Utf8Path,
    ) -> Result<bool> {
        let mut previous = None;

        for attempt in 1..=self.checks {
            let current = Self::file_size(path).await?;

            tracing::debug!(
                "stability {}/{}: {} ({} bytes)",
                attempt,
                self.checks,
                path,
                current
            );

            if Self::is_stable(previous, current) {
                tracing::debug!("file stable");
                return Ok(true);
            }

            previous = Some(current);

            sleep(self.interval).await;
        }

        tracing::debug!("file still changing");

        Ok(false)
    }
}

impl Default for StabilityChecker {
    fn default() -> Self {
        Self::new(Duration::from_millis(500), 2)
    }
}