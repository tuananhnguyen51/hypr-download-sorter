use camino::Utf8PathBuf;

/// High-level category of a file.
///
/// The classifier is responsible for mapping MIME types
/// and extensions into one of these categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileCategory {
    Image,
    Video,
    Audio,
    Document,
    Archive,
    Executable,
    Unknown,
}

impl FileCategory {
    /// Human-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Image => "Image",
            Self::Video => "Video",
            Self::Audio => "Audio",
            Self::Document => "Document",
            Self::Archive => "Archive",
            Self::Executable => "Executable",
            Self::Unknown => "Unknown",
        }
    }

    /// Default destination folder name.
    ///
    /// This is only a fallback.
    /// The final destination is determined by the rule engine.
    #[must_use]
    pub const fn default_directory(self) -> &'static str {
        match self {
            Self::Image => "Pictures",
            Self::Video => "Videos",
            Self::Audio => "Music",
            Self::Document => "Documents",
            Self::Archive => "Archives",
            Self::Executable => "Applications",
            Self::Unknown => "Downloads",
        }
    }
}

/// A managed file inside the processing pipeline.
#[derive(Debug, Clone)]
pub struct ManagedFile {
    /// Absolute path to the file.
    pub path: Utf8PathBuf,

    /// Detected category.
    pub category: FileCategory,

    /// MIME type if known.
    ///
    /// Example:
    /// image/png
    /// application/pdf
    pub mime: Option<String>,
}

impl ManagedFile {
    #[must_use]
    pub fn new(path: Utf8PathBuf) -> Self {
        Self {
            path,
            category: FileCategory::Unknown,
            mime: None,
        }
    }
}
