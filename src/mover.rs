use camino::{Utf8Path, Utf8PathBuf};

use crate::{AppError, Result};

#[derive(Debug, Clone)]
pub struct Mover;

const MAX_DUPLICATES: usize = 100_000;

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

    fn ensure_parent(destination: &Utf8Path) -> Result<()> {
        if let Some(parent) = destination.parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(())
    }

    fn unique_destination(path: &Utf8Path) -> Result<Utf8PathBuf> {
        if !path.exists() {
            return Ok(path.to_path_buf());
        }

        let stem = path.file_stem().unwrap_or("file");
        let extension = path.extension();

        let Some(parent) = path.parent() else {
            return Err(AppError::message("destination has no parent directory"));
        };

        for index in 1..=MAX_DUPLICATES {
            let filename = match extension {
                Some(ext) => format!("{stem} ({index}).{ext}"),
                None => format!("{stem} ({index})"),
            };

            let candidate = parent.join(filename);

            if !candidate.exists() {
                return Ok(candidate);
            }
        }

        Err(AppError::message(
            "could not find an available destination filename",
        ))
    }

    pub fn move_file(&self, source: &Utf8Path, destination: &Utf8Path) -> Result<()> {
        let destination = Self::unique_destination(destination)?;

        Self::ensure_parent(&destination)?;

        tracing::info!("Moving {} -> {}", source, destination);

        match std::fs::rename(source, &destination) {
            Ok(()) => Ok(()),

            Err(err) if err.raw_os_error() == Some(libc::EXDEV) => {
                tracing::debug!("Cross-device move detected, falling back to copy");

                std::fs::copy(source, &destination)?;
                std::fs::remove_file(source)?;

                Ok(())
            }

            Err(err) => Err(err.into()),
        }
    }
}
