use camino::Utf8PathBuf;

use crate::{Result, classifier::Classifier, mover::Mover, notifier::Notifier, rules::RuleEngine};

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

    pub async fn process(&self, path: Utf8PathBuf) -> Result<()> {
        tracing::debug!("Pipeline: {}", path);

        tracing::debug!("ENTER PIPELINE");

        if !path.exists() {
            tracing::debug!("Skipping vanished file: {}", path);
            return Ok(());
        }

        let file = self.classifier.classify(path)?;
        tracing::debug!("mime={:?}, category={:?}", file.mime, file.category);

        let destination = self.rules.resolve(&file)?;
        tracing::debug!("Moving {} -> {}", file.path, destination);

        self.mover.move_file(&file.path, &destination)?;

        self.notifier
            .notify("File sorted", &format!("Moved to {}", destination))
            .await?;

        Ok(())
    }
}
