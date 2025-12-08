use crate::{
    WeatherProvider,
    common::*,
    models::{WeatherInfo, weather_api::*},
    utils::date::*,
};
use ::reqwest::Url;

pub struct WeatherApiProvider;

#[async_trait::async_trait]
impl WeatherProvider for WeatherApiProvider {
    async fn get_weather(
        &self,
        provider_key: Option<&str>,
        address: &str,
        date: Option<&str>,
    ) -> Result<WeatherInfo> {
        let provider_key = provider_key.ok_or_else(|| {
            Error::from("'WeatherApi' API key not set. Please set it using: 'weather provider wa --key <API_KEY>'")
        })?;

        let date = normalize_date(date);

        let url = Url::parse_with_params(
            "https://api.weatherapi.com/v1/current.json",
            &[
                ("key", provider_key),
                ("q", address),
                ("dt", &date),
                ("aqi", "no"),
                ("days", "1"),
            ],
        )
        .map_err(|e| format!("Failed to build URL: {e}"))?;

        let response = reqwest::get(url).await?.error_for_status()?;
        let body = response.json::<WeatherApiResponse>().await?;

        Ok(WeatherInfo {
            country: body.location.country,
            city: body.location.name,
            date,
            temperature: body.current.temp_f,
            humidity: body.current.humidity,
            description: Some(body.current.condition.text),
        })
    }
}
