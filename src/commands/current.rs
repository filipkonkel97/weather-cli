use crate::api::WeatherClient;
use crate::config::Config;
use anyhow::Result;

pub async fn run(city: String) -> Result<()> {
    let config = Config::load()?;

    let client = WeatherClient::new(config.api_key);

    let weather = client.current_weather(&city).await?;

    println!("City: {}", weather.name);
    println!("Temp: {}°C", weather.main.temp);
    println!("Feels like: {}°C", weather.main.feels_like);
    println!("Humidity: {}%", weather.main.humidity);

    if let Some(w) = weather.weather.first() {
        println!("Condition: {} ({})", w.main, w.description);
    }

    println!("Wind: {} m/s", weather.wind.speed);

    Ok(())
}
