use camino::Utf8Path;

use crate::{Result, pipeline::Pipeline};

#[derive(Debug, Default)]
pub struct StartupScanner;

impl StartupScanner {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub async fn scan(&self, directory: &Utf8Path, pipeline: &Pipeline) -> Result<()> {
        tracing::info!("Running startup scan...");

        for entry in std::fs::read_dir(directory)? {
            let entry = entry?;

            let path = match camino::Utf8PathBuf::from_path_buf(entry.path()) {
                Ok(path) => path,
                Err(_) => continue,
            };

            if !path.is_file() {
                continue;
            }

            tracing::debug!("startup: {}", path);

            if let Err(err) = pipeline.process(path).await {
                tracing::warn!("{err}");
            }
        }

        tracing::info!("Startup scan complete");

        Ok(())
    }
}
