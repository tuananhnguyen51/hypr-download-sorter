use camino::{Utf8Path, Utf8PathBuf};

use crate::models::FileCategory;

/// Resolves destination paths for classified files.
#[derive(Debug, Clone)]
pub struct RuleEngine {
    pictures: Utf8PathBuf,
    videos: Utf8PathBuf,
    music: Utf8PathBuf,
    documents: Utf8PathBuf,
    archives: Utf8PathBuf,
    applications: Utf8PathBuf,
    unknown: Utf8PathBuf,
}

impl RuleEngine {
    #[must_use]
    pub fn new(home: Utf8PathBuf) -> Self {
        Self {
            pictures: home.join("Pictures/Downloads"),
            videos: home.join("Videos/Downloads"),
            music: home.join("Music/Downloads"),
            documents: home.join("Documents/Downloads"),
            archives: home.join("Archives/Downloads"),
            applications: home.join("Applications/Downloads"),
            unknown: home.join("Downloads"),
        }
    }

    /// Resolve destination directory.
    pub fn destination(&self, category: FileCategory) -> &Utf8Path {
        match category {
            FileCategory::Image => &self.pictures,

            FileCategory::Video => &self.videos,

            FileCategory::Audio => &self.music,

            FileCategory::Document => &self.documents,

            FileCategory::Archive => &self.archives,

            FileCategory::Executable => &self.applications,

            FileCategory::Unknown => &self.unknown,
        }
    }
}
