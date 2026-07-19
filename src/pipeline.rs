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
        let file = self.classifier.classify(path)?;

        let destination = self.rules.resolve(&file)?;

        self.mover.move_file(&file.path, &destination)?;

        self.notifier
            .notify(
                "File sorted",
                &format!("Moved to {}", destination),
            )
            .await?;

        Ok(())
    }
}
