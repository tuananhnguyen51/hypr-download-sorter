use camino::{Utf8Path, Utf8PathBuf};
use directories::{BaseDirs, ProjectDirs};

use crate::{
    error::{AppError, Result},
    models::FileCategory,
};

const QUALIFIER: &str = "dev";
const ORGANIZATION: &str = "hypr";
const APPLICATION: &str = "hypr-download-sorter";

/// Centralized path manager.
///
/// Every path used by the application should come from this module.
/// Other modules should never read `$HOME` or XDG variables directly.
#[derive(Debug, Clone)]
pub struct Paths {
    base_dirs: BaseDirs,
    project_dirs: ProjectDirs,
}

impl Paths {
    /// Create a new path manager.
    pub fn new() -> Result<Self> {
        let base_dirs = BaseDirs::new()
            .ok_or_else(|| AppError::message("Failed to locate XDG base directories."))?;

        let project_dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
            .ok_or_else(|| AppError::message("Failed to locate project directories."))?;

        Ok(Self {
            base_dirs,
            project_dirs,
        })
    }

    /// User's home directory.
    #[must_use]
    pub fn home_dir(&self) -> Utf8PathBuf {
        utf8(self.base_dirs.home_dir())
    }

    /// ~/.config/hypr-download-sorter
    #[must_use]
    pub fn config_dir(&self) -> Utf8PathBuf {
        utf8(self.project_dirs.config_dir())
    }

    /// ~/.local/state/hypr-download-sorter
    #[must_use]
    pub fn state_dir(&self) -> Utf8PathBuf {
        self.project_dirs
            .state_dir()
            .map(utf8)
            .unwrap_or_else(|| self.home_dir().join(".local/state"))
    }

    /// ~/Downloads
    #[must_use]
    pub fn downloads_dir(&self) -> Utf8PathBuf {
        self.home_dir().join("Downloads")
    }

    /// Expand "~" inside configuration values.
    pub fn expand<S>(&self, path: S) -> Utf8PathBuf
    where
        S: AsRef<str>,
    {
        let path = path.as_ref();

        if path == "~" {
            return self.home_dir();
        }

        if let Some(rest) = path.strip_prefix("~/") {
            return self.home_dir().join(rest);
        }

        Utf8PathBuf::from(path)
    }

    /// Default destination directory for a category.
    #[must_use]
    pub fn default_destination(&self, category: FileCategory) -> Utf8PathBuf {
        let home = self.home_dir();

        match category {
            FileCategory::Image => home.join("Pictures/Downloads"),
            FileCategory::Video => home.join("Videos/Downloads"),
            FileCategory::Audio => home.join("Music/Downloads"),
            FileCategory::Document => home.join("Documents/Downloads"),
            FileCategory::Archive => home.join("Archives"),
            FileCategory::Executable => home.join("Applications"),
            FileCategory::Unknown => self.downloads_dir(),
        }
    }
}

/// Ensure a directory exists.
pub fn ensure_directory(path: &Utf8Path) -> Result<()> {
    std::fs::create_dir_all(path)?;
    Ok(())
}

/// Convert a std::path::Path into Utf8PathBuf.
///
/// Panics are intentionally avoided.
/// If the OS returns a non-UTF8 path, we fall back to a lossy conversion.
fn utf8(path: &std::path::Path) -> Utf8PathBuf {
    match Utf8PathBuf::from_path_buf(path.to_path_buf()) {
        Ok(path) => path,
        Err(path) => Utf8PathBuf::from(path.to_string_lossy().into_owned()),
    }
}
