use camino::{Utf8Path, Utf8PathBuf};

use crate::Result;

#[derive(Debug, Clone)]
pub struct Mover;

impl Mover {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub fn move_file(&self, source: &Utf8Path, destination: &Utf8PathBuf) -> Result<()> {
        std::fs::rename(source, destination)?;

        Ok(())
    }
}

impl Default for Mover {
    fn default() -> Self {
        Self::new()
    }
}
