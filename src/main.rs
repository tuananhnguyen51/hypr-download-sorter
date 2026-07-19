use camino::Utf8PathBuf;

use hypr_download_sorter::{
    AppError, Result, classifier::Classifier, config::Config, mover::Mover, notifier::Notifier,
    pipeline::Pipeline, rules::RuleEngine, watcher::WatchEngine,
};

use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        println!("{:#?}", err);
        error!("{err}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    init_logging();

    info!("Starting hypr-download-sorter");

    let config = Config::load()?;

    let watch_dir = config.watch_directory()?;

    info!("Watching {}", watch_dir);

    // Home directory
    let home = std::env::var("HOME")
        .map(Utf8PathBuf::from)
        .map_err(|_| AppError::message("HOME environment variable not set"))?;

    // Components
    let classifier = Classifier::new();
    let rules = RuleEngine::new(home);
    let mover = Mover::new();
    let notifier = Notifier::new().await?;

    let pipeline = Pipeline::new(classifier, rules, mover, notifier);

    let mut engine = WatchEngine::new(pipeline)?;

    engine.watch(watch_dir.as_path())?;
    engine.run().await?;

    Ok(())
}

fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}
