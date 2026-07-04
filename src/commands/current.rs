use crate::config::Config;
use anyhow::Result;

pub async fn run(city: String) -> Result<()> {
    let cfg = Config::load()?;

    println!("Weather for {}", city);

    Ok(())
}
