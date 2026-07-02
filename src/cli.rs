use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "weather-cli")]
#[command(version)]
#[command(about = "CLI for OpenWeather API")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Login,
    Current { city: String },
}
