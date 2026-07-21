use camino::Utf8Path;
use notify_rust::Notification;

use crate::{models::ManagedFile, Result};

#[derive(Debug, Clone, Default)]
pub struct Notifier;

impl Notifier {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub async fn notify_success(
        &self,
        file: &ManagedFile,
        destination: &Utf8Path,
    ) -> Result<()> {
        self.notify(
            "Download sorted",
            &format!(
                "{} → {}",
                Self::filename(file),
                destination,
            ),
        )
        .await
    }

    pub async fn notify_error(
        &self,
        message: &str,
    ) -> Result<()> {
        self.notify("Sorting failed", message).await
    }

    pub async fn notify_skip(
        &self,
        file: &ManagedFile,
    ) -> Result<()> {
        self.notify("File skipped", Self::filename(file))
            .await
    }

    fn filename(file: &ManagedFile) -> &str {
        file.path.file_name().unwrap_or("unknown")
    }

    async fn notify(
        &self,
        title: &str,
        body: &str,
    ) -> Result<()> {
        tracing::debug!("notification: {} - {}", title, body);

        Notification::new()
            .summary(title)
            .body(body)
            .show()?;

        Ok(())
    }
}