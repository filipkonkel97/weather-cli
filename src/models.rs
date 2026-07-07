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
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: u32,
    pub humidity: u8,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: u16,
}

#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    pub list: Vec<ForecastItem>,
    pub city: ForecastCity,
}

#[derive(Debug, Deserialize)]
pub struct ForecastItem {
    pub dt: i64,
    pub dt_txt: String,
    pub main: Main,
    pub weather: Vec<Weather>,
    pub wind: Wind,
    pub visibility: u32,
    pub pop: f64,
}

#[derive(Debug, Deserialize)]
pub struct ForecastCity {
    pub name: String,
}
