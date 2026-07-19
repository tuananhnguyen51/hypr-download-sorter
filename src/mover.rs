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
        let ext = path.extension();

        let Some(parent) = path.parent() else {
            return Ok(path.to_path_buf());
        };

        for i in 1..=100_000 {
            let name = match ext {
                Some(ext) => format!("{stem} ({i}).{ext}"),
                None => format!("{stem} ({i})"),
            };

            let candidate = parent.join(name);

            if !candidate.exists() {
                return Ok(candidate);
            }
        }

        Err(AppError::message("could not find a unique filename"))
    }

    pub fn move_file(&self, source: &Utf8Path, destination: &Utf8PathBuf) -> Result<()> {
        if source == destination {
            return Ok(());
        }

        Self::ensure_parent(destination)?;

        let destination = Self::unique_destination(destination)?;

        std::fs::rename(source, &destination)?;

        Ok(())
    }
}
