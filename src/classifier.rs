use camino::Utf8PathBuf;

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
        let mime = infer::get_from_path(&path)
            .ok()
            .flatten()
            .map(|kind| kind.mime_type().to_string());

        let category = mime
            .as_deref()
            .map(classify_mime)
            .unwrap_or_else(|| classify_extension(&path));

        Ok(ManagedFile {
            path,
            category,
            mime,
        })
    }
}

fn classify_mime(mime: &str) -> FileCategory {
    if mime.starts_with("image/") {
        return FileCategory::Image;
    }

    if mime.starts_with("video/") {
        return FileCategory::Video;
    }

    if mime.starts_with("audio/") {
        return FileCategory::Audio;
    }

    match mime {
        "application/pdf"
        | "text/plain"
        | "application/msword"
        | "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
            FileCategory::Document
        }

        "application/zip" | "application/x-rar" | "application/x-7z-compressed" => {
            FileCategory::Archive
        }

        "application/x-executable" => FileCategory::Executable,

        _ => FileCategory::Unknown,
    }
}

fn classify_extension(path: &camino::Utf8Path) -> FileCategory {
    let ext = path.extension().unwrap_or_default().to_ascii_lowercase();

    match ext.as_str() {
        "png" | "jpg" | "jpeg" | "webp" | "gif" | "svg" => FileCategory::Image,

        "mp4" | "mkv" | "webm" | "avi" => FileCategory::Video,

        "mp3" | "flac" | "wav" => FileCategory::Audio,

        "pdf" | "txt" | "doc" | "docx" | "odt" => FileCategory::Document,

        "zip" | "rar" | "7z" | "tar" => FileCategory::Archive,

        "appimage" | "bin" => FileCategory::Executable,

        _ => FileCategory::Unknown,
    }
}
