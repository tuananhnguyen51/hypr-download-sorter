use camino::Utf8PathBuf;

/// Internal filesystem event.
///
/// This type is independent of the underlying watcher implementation
/// (`notify` on Linux). The rest of the application should only work
/// with `FileEvent`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEvent {
    /// Absolute path to the affected file.
    pub path: Utf8PathBuf,

    /// Type of filesystem event.
    pub kind: FileEventKind,
}

impl FileEvent {
    /// Create a new filesystem event.
    #[must_use]
    pub fn new(path: Utf8PathBuf, kind: FileEventKind) -> Self {
        Self { path, kind }
    }
}

/// High-level filesystem event kinds.
///
/// These are intentionally simpler than `notify::EventKind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileEventKind {
    /// A file has been created.
    Created,

    /// A file has been modified.
    Modified,

    /// A file has been removed.
    Removed,

    /// A file has been renamed or moved.
    Renamed,

    /// Any event we do not explicitly recognize.
    Other,
}

impl FileEventKind {
    /// Human-readable event name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Modified => "modified",
            Self::Removed => "removed",
            Self::Renamed => "renamed",
            Self::Other => "other",
        }
    }
}
