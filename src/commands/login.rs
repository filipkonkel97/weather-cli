use crate::config::Config;
use anyhow::Result;
use std::io;

pub fn run() -> Result<()> {
    println!("Please enter your OpenWeather API key:");

    let mut api_key = String::new();

    io::stdin().read_line(&mut api_key)?;

    let cfg = Config {
        api_key: api_key.trim().to_string(),
    };

    cfg.save()?;

    println!("API key saved!");

    Ok(())
}
