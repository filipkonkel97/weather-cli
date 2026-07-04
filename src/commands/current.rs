use crate::config::Config;
use crate::models::CurrentWeather;
use anyhow::Result;

pub async fn run(city: String) -> Result<()> {
    let cfg = Config::load()?;

    println!("Weather for {}", city);

    Ok(())
}
