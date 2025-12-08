pub mod weather_proto {
    tonic::include_proto!("weather");
}

use crate::{WeatherProvider, common::*, models::WeatherInfo, utils::date::*};
use ::async_trait::async_trait;
use weather_proto::{WeatherRequest, weather_service_client::WeatherServiceClient};

/// Mock provider address for weather data using gRPC
const MOCK_SERVER: &str = "http://[::1]:54583";

pub struct GrpcMockProvider;

#[async_trait]
impl WeatherProvider for GrpcMockProvider {
    async fn get_weather(
        &self,
        _provider_key: Option<&str>,
        address: &str,
        date: Option<&str>,
    ) -> Result<WeatherInfo> {
        let date_normalized = normalize_date(date);

        let client_result = WeatherServiceClient::connect(MOCK_SERVER).await;

        match client_result {
            Ok(mut client) => {
                let request = tonic::Request::new(WeatherRequest {
                    location: address.to_string(),
                    date: date_normalized.clone(),
                });

                let response = client
                    .get_weather(request)
                    .await
                    .map_err(|e| format!("gRPC error: {e}"))?
                    .into_inner();

                Ok(WeatherInfo {
                    country: response.country,
                    city: response.city,
                    date: response.date,
                    temperature: response.temperature,
                    humidity: response.humidity as u8,
                    description: Some(response.description),
                })
            }
            Err(_) => {
                eprintln!(
                    "(gRPC Mock: Server not found at '{MOCK_SERVER}', returning static data)"
                );

                Ok(WeatherInfo {
                    country: "gRPC Mock Country".to_string(),
                    city: "gRPC Mock City".to_string(),
                    date: date_normalized,
                    temperature: 42.0,
                    humidity: 88,
                    description: Some("Rain (Mock)".to_string()),
                })
            }
        }
    }
}
