use crate::api::WeatherClient;
use crate::config::Config;
use anyhow::Result;

pub async fn run(city: String) -> Result<()> {
    let config = Config::load()?;

    let client = WeatherClient::new(config.api_key);

    let forecast = client.weather_forecast(&city).await?;

    println!("Forecast for {}\n", forecast.city.name);

    for item in forecast.list.iter().take(5) {
        let weather = &item.weather[0];

        println!("--------------------------------");
        println!("{}", item.dt_txt);
        println!("Temperature: {}°C", item.main.temp);
        println!("Feels like: {}°C", item.main.feels_like);
        println!("Humidity: {}%", item.main.humidity);
        println!("Pressure: {} hPa", item.main.pressure);
        println!("Wind speed: {} m/s", item.wind.speed);
        println!("Description: {}", weather.description);
        println!("Pop: {:.0}%", item.pop * 100.0);
    }

    Ok(())
}
