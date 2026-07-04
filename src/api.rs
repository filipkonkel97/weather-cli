use anyhow::Result;
use reqwest::Client;

use crate::models::CurrentWeather;

pub struct WeatherClient {
    client: reqwest::Client,
    api_key: String,
}

impl WeatherClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn current_weather(&self, city: &str) -> Result<CurrentWeather> {
        let url = "https://api.openweathermap.org/data/2.5/weather";

        let response = self
            .client
            .get(url)
            .query(&[("q", city), ("appid", &self.api_key), ("units", "metric")])
            .send()
            .await?
            .error_for_status()?
            .json::<CurrentWeather>()
            .await?;

        Ok(response)
    }
}
