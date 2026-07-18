use std::fs;

use camino::Utf8PathBuf;
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
    /// Directory to watch.
    pub watch_dir: String,

    /// Enable desktop notifications.
    pub notifications: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            watch_dir: "~/Downloads".to_string(),
            notifications: true,
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

        let user_config = config_dir.join("config.toml");

        if user_config.exists() {
            return Self::load_file(user_config);
        }

        let default_config = Utf8PathBuf::from("config/default.toml");

        if default_config.exists() {
            return Self::load_file(default_config);
        }

        Err(AppError::config("No configuration file could be found."))
    }

    fn load_file(path: Utf8PathBuf) -> Result<Self> {
        let contents = fs::read_to_string(path)?;

        let config: Config = toml::from_str(&contents)?;

        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<()> {
        if self.watch_dir.trim().is_empty() {
            return Err(AppError::config("watch_dir cannot be empty."));
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
