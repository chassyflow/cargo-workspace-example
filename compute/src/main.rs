use std::{
    io::{read_to_string, Read},
    path::{Path, PathBuf},
};

use anyhow::{ensure, Context};
use serde::Deserialize;
use tracing::info;
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
    let path = args
        .get(1)
        .map(PathBuf::from)
        .context("Failed to parse path")?;
    ensure!(path.exists(), "File doesn't exist");
    let file = std::fs::File::open(path).context("Failed to open file")?;
    let content = read_to_string(file).context("Failed to read content")?;
    toml::from_str(&content).context("Failed to parse config")
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let cfg = get_config(args);
    if let Err(e) = &cfg {
        eprintln!("Failed to parse config via file: {:?}", e)
    }
    let cfg = cfg
        .or(envy::from_env::<Configuration>().context("Failed to parse env"))
        .context("Failed to determine configuration")?;
    dbg!(&cfg);
    // set up tracing for logging with defaults
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env() // env: RUST_LOG
                .unwrap_or_else(|_| "compute=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
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
