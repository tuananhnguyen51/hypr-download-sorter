use hypr_download_sorter::{Result, config::Config, watcher::WatchService};

use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt};

fn main() {
    if let Err(err) = run() {
        error!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    init_logging();

    info!("Starting hypr-download-sorter");

    let config = Config::load()?;

    let watch_dir = config.watch_directory()?;

    info!("Watching {}", watch_dir);

    let mut watcher = WatchService::new()?;

    watcher.watch(watch_dir.as_path())?;

    info!("Watcher initialized");

    loop {
        let events = watcher.recv()?;

        for event in events {
            info!("{}: {}", event.kind.as_str(), event.path);
        }
    }
}

fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}
