use crate::{WeatherProvider, common::*, models::WeatherInfo, utils::date::*};
use ::async_trait::async_trait;

pub struct MockProvider;

#[async_trait]
impl WeatherProvider for MockProvider {
    async fn get_weather(
        &self,
        _provider_key: Option<&str>,
        _address: &str,
        date: Option<&str>,
    ) -> Result<WeatherInfo> {
        let date = normalize_date(date);

        Ok(WeatherInfo {
            country: "Mock Country".to_string(),
            city: "Mock City".to_string(),
            date,
            temperature: 20.0,
            humidity: 50,
            description: Some("Sunny (Mock)".to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_provider_returns_data() {
        let provider = MockProvider;
        let result = provider.get_weather(None, "Nowhere", None).await;

        assert!(result.is_ok());
        let info = result.unwrap();

        assert_eq!(info.city, "Mock City");
        assert_eq!(info.country, "Mock Country");
        assert_eq!(info.temperature, 20.0);
        assert_eq!(info.humidity, 50);
        assert_eq!(info.description, Some("Sunny (Mock)".to_string()));
    }

    #[tokio::test]
    async fn test_mock_provider_handles_date() {
        let provider = MockProvider;
        let specific_date = "10/5/2023";

        let result = provider
            .get_weather(None, "Nowhere", Some(specific_date))
            .await;

        assert!(result.is_ok());
        let info = result.unwrap();

        assert_eq!(info.date, "2023-10-05");
    }

    #[tokio::test]
    async fn test_mock_provider_defaults_to_today() {
        use ::chrono::Utc;

        let provider = MockProvider;
        let result = provider.get_weather(None, "Nowhere", None).await;

        assert!(result.is_ok());
        let info = result.unwrap();

        // Check that the date is today's date
        let today = Utc::now().date_naive().format("%Y-%m-%d").to_string();
        assert_eq!(info.date, today);
    }
}
