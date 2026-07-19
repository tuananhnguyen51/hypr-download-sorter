use hypr_download_sorter::{
    Result, classifier::Classifier, config::Config, mover::Mover, notifier::Notifier,
    pipeline::Pipeline, rules::RuleEngine, startup::StartupScanner, watcher::WatchEngine,
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

    let classifier = Classifier::new();
    let rules = RuleEngine::new(config.clone());
    let mover = Mover::new();
    let notifier = Notifier::new().await?;

    let pipeline = Pipeline::new(classifier, rules, mover, notifier);

    let scanner = StartupScanner::new();

    scanner.scan(watch_dir.as_path(), &pipeline).await?;

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
