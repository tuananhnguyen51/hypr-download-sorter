use std::time::Instant;

use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    models::ManagedFile,
    Result,
    classifier::Classifier,
    mover::Mover,
    notifier::Notifier,
    rules::RuleEngine,
};

#[derive(Debug)]
pub struct Pipeline {
    classifier: Classifier,
    rules: RuleEngine,
    mover: Mover,
    notifier: Notifier,
}

impl Pipeline {
    pub fn new(
        classifier: Classifier,
        rules: RuleEngine,
        mover: Mover,
        notifier: Notifier,
    ) -> Self {
        Self {
            classifier,
            rules,
            mover,
            notifier,
        }
    }

    fn move_file(
        &self,
        file: &ManagedFile,
        destination: &Utf8Path,
    ) -> Result<()> {
        self.mover
            .move_file(file.path.as_path(), destination)
    }

    async fn notify(
        &self,
        file: &ManagedFile,
        destination: &Utf8Path,
    ) -> Result<()> {
        self.notifier
            .notify_success(file, destination)
            .await
    }

    async fn move_and_notify(
        &self,
        file: &ManagedFile,
        destination: &Utf8Path,
    ) -> Result<()> {
        self.move_file(file, destination)?;
        self.notify(file, destination).await
    }

    pub async fn process(&self, path: Utf8PathBuf) -> Result<()> {
        tracing::debug!("Pipeline: {}", path);

        let start = Instant::now();

        if !path.exists() {
            return Ok(());
        }

        let file = self.classifier.classify(path)?;
        let destination = self.rules.resolve_destination(&file)?;

        if let Err(err) = self
            .move_and_notify(&file, destination.as_path())
            .await
        {
            self.notifier
                .notify_error(&err.to_string())
                .await?;

            return Err(err);
        }

        tracing::info!("Moved {} -> {}", file.path, destination);
        tracing::debug!("Finished in {:?}", start.elapsed());

        Ok(())
    }


}
