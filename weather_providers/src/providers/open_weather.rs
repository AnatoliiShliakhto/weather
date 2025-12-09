use crate::{
    WeatherProvider,
    common::*,
    models::{WeatherInfo, open_weather::*},
    utils::date::*,
};
use ::reqwest::Url;
use ::tracing::instrument;

#[derive(Debug)]
pub struct OpenWeatherProvider;

#[async_trait::async_trait]
impl WeatherProvider for OpenWeatherProvider {
    #[instrument(fields(provider_key, address, date))]
    async fn get_weather(
        &self,
        provider_key: Option<&str>,
        address: &str,
        date: Option<&str>,
    ) -> Result<WeatherInfo> {
        let provider_key = provider_key.ok_or_else(|| {
            Error::from("'OpenWeather' API key not set. Please set it using: 'weather provider ow --key <API_KEY>'")
        })?;

        // --- Geocoding API ---
        let geo_url = Url::parse_with_params(
            "https://api.openweathermap.org/geo/1.0/direct",
            &[("appid", provider_key), ("q", address), ("limit", "1")],
        )
        .map_err(|e| format!("Failed to build URL: {e}"))?;

        let geo_response = reqwest::get(geo_url).await?.error_for_status()?;
        let geo_body = geo_response.json::<Vec<OpenWeatherGeoResponse>>().await?;

        let location = geo_body
            .first()
            .ok_or_else(|| format!("Location not found: '{address}'"))?;

        // --- Weather API ---
        let date = normalize_date(date);

        let url = Url::parse_with_params(
            "https://api.openweathermap.org/data/3.0/onecall/day_summary",
            &[
                ("appid", provider_key),
                ("lat", &location.lat.to_string()),
                ("lon", &location.lon.to_string()),
                ("date", &date),
                ("units", "imperial"),
            ],
        )
        .map_err(|e| format!("Failed to build URL: {e}"))?;

        let response = reqwest::get(url).await?.error_for_status()?;
        let body = response.json::<OpenWeatherResponse>().await?;

        Ok(WeatherInfo {
            country: location.country.clone(),
            city: location.name.clone(),
            date,
            temperature: body.temperature.afternoon,
            humidity: body.humidity.afternoon,
            description: None,
        })
    }
}
