use std::{io::read_to_string, path::PathBuf};

use anyhow::{ensure, Context};
use serde::Deserialize;
use tracing::{debug, error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod algorithms;

#[derive(Debug, Deserialize)]
enum Computation {
    Pi,
    Fibonacci,
}

#[derive(Debug, Deserialize)]
struct Configuration {
    pub computation: Computation,
    pub max_n: usize,
}

fn get_config(args: Vec<String>) -> anyhow::Result<Configuration> {
    info!("Attempting to parse path from provided argument");
    let path = args
        .get(1)
        .map(PathBuf::from)
        .context("Failed to parse path")?;
    info!("Checking that file exists: {}", &path.to_str().unwrap());
    ensure!(path.exists(), "File doesn't exist");
    info!("Attempting to open file");
    let file = std::fs::File::open(path).context("Failed to open file")?;
    info!("Attempting to read content");
    let content = read_to_string(file).context("Failed to read content")?;
    info!("Attempting to parse content as TOML");
    toml::from_str(&content).context("Failed to parse config")
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env() // env: RUST_LOG
                .unwrap_or_else(|_| "compute=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("Determining what configuration should be used");
    let args: Vec<String> = std::env::args().collect();
    let cfg = get_config(args);
    if let Err(e) = &cfg {
        error!("Failed to parse config via file: {:?}", e)
    }
    let cfg = cfg
        .or(envy::from_env::<Configuration>().context("failed to parse from env"))
        .context("Failed to determine config")?;
    debug!("config: {:?}", &cfg);
    // set up tracing for logging with defaults
    match cfg {
        Configuration {
            max_n,
            computation: Computation::Pi,
        } => {
            info!("Performing Pi computations");
            algorithms::pi::compute(max_n);
        }
        Configuration {
            max_n,
            computation: Computation::Fibonacci,
        } => {
            info!("Performing Fibonacci computations");
            algorithms::fib::compute(max_n as u128);
        }
    };
    Ok(())
}
