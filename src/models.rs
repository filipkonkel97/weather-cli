use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    pub weather: Vec<Weather>,
    pub main: Main,
    pub wind: Wind,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub main: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub humidity: u8,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    pub speed: f64,
}

#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    pub list: Vec<ForecastItem>,
    pub city: ForecastCity,
}

#[derive(Debug, Deserialize)]
pub struct ForecastItem {
    pub dt_txt: String,
    pub main: Main,
    pub weather: Vec<Weather>,
}

#[derive(Debug, Deserialize)]
pub struct ForecastCity {
    pub name: String,
}
