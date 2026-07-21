use std::{fs, io};

use camino::{Utf8Path, Utf8PathBuf};

use crate::{AppError, Result};

#[derive(Debug, Clone)]
pub struct Mover;

impl Default for Mover {
    fn default() -> Self {
        Self::new()
    }
}

impl Mover {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub fn move_file(&self, source: &Utf8Path, destination: &Utf8Path) -> Result<()> {
        let destination = Self::unique_destination(destination)?;
        Self::ensure_parent(&destination)?;

        match fs::rename(source, &destination) {
            Ok(_) => Ok(()),

            Err(err) if Self::is_cross_device(&err) => Self::copy_and_remove(source, &destination),

            Err(err) => Err(err.into()),
        }
    }

    fn copy_and_remove(source: &Utf8Path, destination: &Utf8Path) -> Result<()> {
        fs::copy(source, destination)?;

        if let Err(err) = fs::remove_file(source) {
            let _ = fs::remove_file(destination);
            return Err(err.into());
        }

        Ok(())
    }

    fn ensure_parent(destination: &Utf8Path) -> Result<()> {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        Ok(())
    }

    fn unique_destination(path: &Utf8Path) -> Result<Utf8PathBuf> {
        if !path.exists() {
            return Ok(path.to_path_buf());
        }

        let stem = path.file_stem().unwrap_or("file");
        let ext = path.extension();

        let Some(parent) = path.parent() else {
            return Ok(path.to_path_buf());
        };

        for i in 1..=100_000 {
            let filename = match ext {
                Some(ext) => format!("{stem} ({i}).{ext}"),
                None => format!("{stem} ({i})"),
            };

            let candidate = parent.join(filename);

            if !candidate.exists() {
                return Ok(candidate);
            }
        }

        Err(AppError::message(
            "could not determine unique destination filename",
        ))
    }

    fn is_cross_device(err: &io::Error) -> bool {
        matches!(err.raw_os_error(), Some(libc::EXDEV))
    }
}
