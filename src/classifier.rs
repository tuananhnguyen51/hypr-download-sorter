use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    Result,
    models::{FileCategory, ManagedFile},
};

#[derive(Debug, Default)]
pub struct Classifier;

impl Classifier {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub fn classify(&self, path: Utf8PathBuf) -> Result<ManagedFile> {
        if path.is_dir() {
            return Ok(ManagedFile {
                path,
                category: FileCategory::Unknown,
                mime: None,
            });
        }

        let mime = infer::get_from_path(&path)?.map(|kind| kind.mime_type().to_owned());

        let extension_category = classify_extension(&path);

        let category = if extension_category != FileCategory::Unknown {
            extension_category
        } else {
            mime.as_deref()
                .map(classify_mime)
                .unwrap_or(FileCategory::Unknown)
        };

        let file = ManagedFile {
            path,
            category,
            mime,
        };

        tracing::debug!("mime={:?}, category={:?}", file.mime, file.category);

        Ok(file)
    }
}

fn classify_extension(path: &Utf8Path) -> FileCategory {
    let Some(extension) = path.extension() else {
        return FileCategory::Unknown;
    };

    match extension.to_ascii_lowercase().as_str() {
        // Images
        "jpg" | "jpeg" | "png" | "gif"
        | "bmp" | "webp" | "svg"
        | "tif" | "tiff" | "ico"
            => FileCategory::Image,

        // Videos
        "mp4" | "mkv" | "avi" | "mov"
        | "webm" | "flv" | "wmv"
        | "m4v" | "mpeg" | "mpg"
            => FileCategory::Video,

        // Audio
        "mp3" | "wav" | "flac" | "aac"
        | "ogg" | "opus" | "m4a"
        | "wma"
            => FileCategory::Audio,

        // Documents
        "pdf" | "txt" | "md"
        | "doc" | "docx"
        | "xls" | "xlsx"
        | "ppt" | "pptx"
        | "odt" | "ods" | "odp"
            => FileCategory::Document,

        // Archives
        "zip" | "rar" | "7z"
        | "tar" | "gz"
        | "xz" | "bz2"
            => FileCategory::Archive,

        // Executables
        "appimage" | "deb"
        | "rpm" | "apk"
        | "run" | "bin"
        | "exe" | "msi"
            => FileCategory::Executable,

        _ => FileCategory::Unknown,
    }
}

fn classify_mime(mime: &str) -> FileCategory {
    match mime {
        m if m.starts_with("image/") => FileCategory::Image,

        m if m.starts_with("video/") => FileCategory::Video,

        m if m.starts_with("audio/") => FileCategory::Audio,

        "application/pdf" => FileCategory::Document,

        m if m.starts_with("text/") => FileCategory::Document,

        "application/zip"
        | "application/x-7z-compressed"
        | "application/x-rar-compressed"
        | "application/gzip"
        | "application/x-tar" => FileCategory::Archive,

        "application/x-executable" | "application/x-sharedlib" | "application/x-pie-executable" => {
            FileCategory::Executable
        }

        _ => FileCategory::Unknown,
    }
}
