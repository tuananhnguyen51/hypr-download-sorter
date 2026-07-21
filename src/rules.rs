use camino::Utf8PathBuf;

use crate::{
    Result,
    config::Config,
    error::AppError,
    models::{FileCategory, ManagedFile},
    paths::Paths,
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
        let paths = Paths::new()?;

        Ok(match category {
            FileCategory::Image => paths.expand(&self.config.images),
            FileCategory::Video => paths.expand(&self.config.videos),
            FileCategory::Audio => paths.expand(&self.config.music),
            FileCategory::Document => paths.expand(&self.config.documents),
            FileCategory::Archive => paths.expand(&self.config.archives),
            FileCategory::Executable => paths.expand(&self.config.executables),
            FileCategory::Unknown => paths.expand(&self.config.documents),
        })
    }

    /// Resolve full destination path.
    pub fn resolve_destination(&self, file: &ManagedFile) -> Result<Utf8PathBuf> {
        let directory = self.destination(file.category)?;

        let filename = file
            .path
            .file_name()
            .ok_or_else(|| AppError::message("missing filename"))?;

        Ok(directory.join(filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, models::FileCategory};

    #[test]
    fn image_destination() -> Result<()> {
        let config = Config::default();
        let rules = RuleEngine::new(config);
        let path = rules.destination(FileCategory::Image)?;

        assert!(path.ends_with("Pictures/Downloads"));

        Ok(())
    }
    #[test]
    fn video_destination() -> Result<()> {
        let config = Config::default();
        let rules = RuleEngine::new(config);

        let path = rules.destination(FileCategory::Video)?;

        assert!(path.ends_with("Videos/Downloads"));

        Ok(())
    }

    #[test]
    fn document_destination() -> Result<()> {
        let config = Config::default();
        let rules = RuleEngine::new(config);

        let path = rules.destination(FileCategory::Document)?;

        assert!(path.ends_with("Documents/Downloads"));

        Ok(())
    }
}
