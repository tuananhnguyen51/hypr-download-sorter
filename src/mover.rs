use std::fs;

use camino::{Utf8Path, Utf8PathBuf};

use crate::{error::Result, models::ManagedFile};

#[derive(Debug, Default)]
pub struct FileMover;

impl FileMover {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub fn move_file(&self, file: &ManagedFile, destination: &Utf8Path) -> Result<Utf8PathBuf> {
        fs::create_dir_all(destination)?;

        let target = unique_path(destination, file.path.file_name().unwrap_or("unknown"));

        fs::rename(&file.path, &target)?;

        Ok(target)
    }
}

fn unique_path(directory: &Utf8Path, filename: &str) -> Utf8PathBuf {
    let first = directory.join(filename);

    if !first.exists() {
        return first;
    }

    let stem = std::path::Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");

    let ext = std::path::Path::new(filename)
        .extension()
        .and_then(|s| s.to_str());

    for index in 1.. {
        let name = match ext {
            Some(ext) => format!("{stem} ({index}).{ext}"),

            None => format!("{stem} ({index})"),
        };

        let candidate = directory.join(name);

        if !candidate.exists() {
            return candidate;
        }
    }

    unreachable!()
}
