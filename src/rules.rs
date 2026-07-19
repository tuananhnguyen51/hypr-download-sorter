use camino::Utf8PathBuf;

use crate::{
    Result,
    config::Config,
    error::AppError,
    models::{FileCategory, ManagedFile},
};

#[derive(Debug, Clone)]
pub struct RuleEngine {
    config: Config,
}

impl RuleEngine {
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Resolve destination directory.
    pub fn destination(&self, category: FileCategory) -> Result<Utf8PathBuf> {
        self.config.destination(category)
    }

    /// Resolve full destination path.
    pub fn resolve(&self, file: &ManagedFile) -> Result<Utf8PathBuf> {
        let directory = self.destination(file.category)?;

        let filename = file
            .path
            .file_name()
            .ok_or_else(|| AppError::message("missing filename"))?;

        Ok(directory.join(filename))
    }
}
