use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    sync::Arc,
};

use clap::Parser;
use log_viewer::{
    app::{AppState, app_logic},
    log_record::LogRecord,
};
use tracing_subscriber::{Registry, layer::SubscriberExt};
use winit::error::EventLoopError;
use xilem::{EventLoop, Xilem};

const DEFAULT_LOG_FILE: &str = "xilem.log";

#[derive(Parser)]
struct Args {
    /// Name of the log file to open
    #[clap(short, long)]
    logfile: Option<PathBuf>,
}

fn main() -> Result<(), EventLoopError> {
    let args = Args::parse();
    let logfile = args
        .logfile
        .unwrap_or_else(|| PathBuf::from(DEFAULT_LOG_FILE));
    let logs = read_log_file(&logfile).unwrap_or(vec![]);

    // logging
    let file = File::create(DEFAULT_LOG_FILE).unwrap();
    let layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(Arc::new(file));
    let subscriber = Registry::default().with(layer);
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set global subscriber");

    tracing::error!("error");
    tracing::warn!("warn");
    tracing::info!("info");
    tracing::debug!("debug");
    tracing::trace!("trace");

    let app = Xilem::new(AppState::new(logs), app_logic);
    app.run_windowed(EventLoop::with_user_event(), "Log Viewer".into())?;
    Ok(())
}

fn read_log_file(logfile: &Path) -> io::Result<Vec<LogRecord>> {
    let log_file = BufReader::new(File::open(logfile)?);
    Ok(log_file
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            serde_json::from_str::<LogRecord>(&line).ok()
        })
        .collect())
}
