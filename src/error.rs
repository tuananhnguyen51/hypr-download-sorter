use thiserror::Error;

/// Convenient result type used throughout the project.
pub type Result<T> = std::result::Result<T, AppError>;

/// Top-level application error.
///
/// All modules should convert their internal errors into this type.
/// This keeps error handling consistent across the daemon.
#[derive(Debug, Error)]
pub enum AppError {
    /// Standard I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Filesystem watcher error.
    #[error("Watcher error: {0}")]
    Notify(#[from] notify::Error),

    /// TOML parsing error.
    #[error("Failed to parse configuration: {0}")]
    Toml(#[from] toml::de::Error),

    /// Configuration is invalid.
    #[error("Configuration error: {0}")]
    Config(String),

    /// UTF-8 path conversion failed.
    #[error("Invalid UTF-8 path: {0}")]
    InvalidPath(String),

    /// Generic application error.
    #[error("{0}")]
    Message(String),

    #[error("D-Bus error: {0}")]
    Zbus(#[from] zbus::Error),
}

impl AppError {
    /// Create a configuration error.
    #[must_use]
    pub fn config<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self::Config(message.into())
    }

    /// Create a generic error.
    #[must_use]
    pub fn message<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self::Message(message.into())
    }
}
