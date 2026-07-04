mod api;
mod cli;
mod commands;
mod config;
mod error;
mod models;

use crate::models::CurrentWeather;
use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Login => {
            commands::login::run()?;
        }
        Commands::Current { city } => {
            commands::current::run(city).await?;
        }
    }

    Ok(())
}
