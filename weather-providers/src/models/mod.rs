pub mod open_weather;
pub mod weather_api;

use ::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherInfo {
    pub country: String,
    pub city: String,
    pub date: String,
    pub temperature: f32,
    pub humidity: u8,
    pub description: Option<String>,
}

impl std::fmt::Display for WeatherInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = self
            .description
            .as_ref()
            .map(|desc| format!(", {desc}"))
            .unwrap_or_default();

        write!(
            f,
            "Weather in '{}, {}': {:.1}°F{}, Humidity: {}%",
            self.country, self.city, self.temperature, description, self.humidity
        )
    }
}
