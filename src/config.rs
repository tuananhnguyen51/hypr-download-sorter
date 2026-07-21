use std::fs;

use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;

use crate::{
    error::{AppError, Result},
    paths::{Paths, ensure_directory},
};

/// Application configuration.
///
/// This structure is deserialized from `config.toml`.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub watch_dir: Utf8PathBuf,

    pub documents: Utf8PathBuf,
    pub images: Utf8PathBuf,
    pub videos: Utf8PathBuf,
    pub music: Utf8PathBuf,
    pub archives: Utf8PathBuf,
    pub executables: Utf8PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            watch_dir: "~/Downloads".into(),

            documents: "~/Documents/Downloads".into(),
            images: "~/Pictures/Downloads".into(),
            videos: "~/Videos/Downloads".into(),
            music: "~/Music/Downloads".into(),
            archives: "~/Archives/Downloads".into(),
            executables: "~/Applications/Downloads".into(),
        }
    }
}

impl Config {
    /// Load configuration.
    ///
    /// Search order:
    ///
    /// 1. ~/.config/hypr-download-sorter/config.toml
    /// 2. config/default.toml (development)
    pub fn load() -> Result<Self> {
        let paths = Paths::new()?;

        let config_dir = paths.config_dir();
        ensure_directory(config_dir.as_path())?;

        let user_config = config_dir.join("default.toml");

        if user_config.exists() {
            return Self::load_file(user_config);
        }

        for path in [
            config_dir.join("default.toml"),
            Utf8PathBuf::from("config/default.toml"),
        ] {
            if path.exists() {
                return Self::load_file(path);
            }
        }


        Err(AppError::config("No configuration file could be found."))
    }

    fn validate(&self) -> Result<()> {
        let watch = self.watch_directory()?;
        Self::validate_directory(&watch)?;

        Ok(())
    }

    fn load_file(path: Utf8PathBuf) -> Result<Self> {
        let contents = fs::read_to_string(path)?;

        let config: Config = toml::from_str(&contents)?;

        config.validate()?;

        Ok(config)
    }

    fn validate_directory(path: &Utf8Path) -> Result<()> {
        if !path.exists() {
            return Err(AppError::config(format!("{path} does not exist")));
        }

        if !path.is_dir() {
            return Err(AppError::config(format!("{path} is not directory")));
        }

        Ok(())
    }

    /// Expanded watch directory.
    ///
    /// "~" is automatically expanded.
    pub fn watch_directory(&self) -> Result<Utf8PathBuf> {
        let paths = Paths::new()?;

        Ok(paths.expand(&self.watch_dir))
    }
}
