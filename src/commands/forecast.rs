use crate::api::WeatherClient;
use crate::config::Config;
use anyhow::Result;

pub async fn run(city: String) -> Result<()> {
    let config = Config::load()?;

    let client = WeatherClient::new(config.api_key);

    let forecast = client.weather_forecast(&city).await?;

    println!("Forecast for {}\n", forecast.city.name);

    for item in forecast.list.iter().take(5) {
        println!("{} -> {}°C", item.dt_txt, item.main.temp);
    }

    Ok(())
}
