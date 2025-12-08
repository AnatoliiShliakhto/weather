//! # Weather Providers Library
//!
//! `weather-providers` is an extensible library for fetching weather data from various
//! third-party services (providers). It provides a unified interface (`WeatherProvider` trait)
//! to decouple application logic from specific API implementations.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use weather_providers::{create_provider, Result, Provider};
//!
//! async fn weather() -> Result<()> {
//!     // Use the enum variant directly
//!     let weather = create_provider(Provider::Mock)
//!         .get_weather(Some("mock-api-key"), "London", None)
//!         .await?;
//!
//!     println!("{}", weather);
//!
//!     Ok(())
//! }
//! ```

mod common;
mod models;
mod providers;
mod utils;

use crate::providers::*;
use ::clap::ValueEnum;
use ::std::fmt::Display;
use async_trait::async_trait;

// Re-export commonly used types for easier access
pub use self::{
    common::{Error, Result},
    models::WeatherInfo,
};

/// Creates a new weather provider instance based on the given identifier.
///
/// This factory function takes a `Provider` enum variant and returns a boxed trait object
/// implementing `WeatherProvider`.
///
/// # Arguments
///
/// * `provider` - The enum variant identifying the provider (e.g., `Provider::OpenWeather`).
///
/// # Returns
///
/// Returns a `Box<dyn WeatherProvider>` that can be used polymorphically to fetch weather data.
///
/// # Examples
///
/// ```rust
/// use weather_providers::{create_provider, Result, Provider};
///
/// async fn weather() -> Result<()> {
///     let weather_info = create_provider(Provider::Mock)
///         .get_weather(Some("mock-key"), "UK, London", None)
///         .await?;
///
///     println!("{weather_info}");
///
///     Ok(())
/// }
/// ```
pub fn create_provider(provider: Provider) -> Box<dyn WeatherProvider> {
    match provider {
        Provider::Mock => Box::new(MockProvider),
        Provider::GrpcMock => Box::new(GrpcMockProvider),
        Provider::OpenWeather => Box::new(OpenWeatherProvider),
        Provider::WeatherApi => Box::new(WeatherApiProvider),
    }
}

#[async_trait]
pub trait WeatherProvider: Send + Sync {
    async fn get_weather(
        &self,
        provider_key: Option<&str>,
        address: &str,
        date: Option<&str>,
    ) -> Result<WeatherInfo>;
}

/// The type of weather provider.
///
/// Used to select a specific implementation at runtime.
#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum Provider {
    /// A mock provider for testing or offline use.
    Mock,
    /// A mock provider for testing or offline use using gRPC.
    GrpcMock,
    /// The OpenWeatherMap API provider.
    OpenWeather,
    /// A generic WeatherAPI provider (placeholder).
    WeatherApi,
}

impl Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Provider {
    pub fn is_mock(&self) -> bool {
        matches!(self, Provider::Mock)
    }

    pub fn id(&self) -> &'static str {
        match self {
            Provider::Mock => "mock",
            Provider::GrpcMock => "grpc",
            Provider::OpenWeather => "ow",
            Provider::WeatherApi => "wa",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Provider::Mock => "MockWeather",
            Provider::GrpcMock => "GrpcMockWeather",
            Provider::OpenWeather => "OpenWeather",
            Provider::WeatherApi => "WeatherApi",
        }
    }
}

impl TryFrom<&str> for Provider {
    type Error = Error;

    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "mockweather" | "mock" => Ok(Provider::Mock),
            "grpcmockweather" | "grpc" => Ok(Provider::GrpcMock),
            "openweather" | "ow" => Ok(Provider::OpenWeather),
            "weatherapi" | "wa" => Ok(Provider::WeatherApi),
            _ => Err(Error::from(format!(
                "Unknown provider: '{s}'.\nAvailable providers: {}",
                Provider::value_variants()
                    .iter()
                    .map(|p| format!("'{p}' ({id})", id = p.id()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_type_parsing() {
        assert_eq!(Provider::try_from("ow").ok(), Some(Provider::OpenWeather));
        assert_eq!(
            Provider::try_from("OpenWeather").ok(),
            Some(Provider::OpenWeather)
        );

        assert_eq!(Provider::try_from("wa").ok(), Some(Provider::WeatherApi));
        assert_eq!(
            Provider::try_from("weatherapi").ok(),
            Some(Provider::WeatherApi)
        );

        assert_eq!(Provider::try_from("mockweather").ok(), Some(Provider::Mock));
        assert_eq!(Provider::try_from("mock").ok(), Some(Provider::Mock));

        assert!(Provider::try_from("").is_err());
        assert!(Provider::try_from("unknown").is_err());
    }

    #[test]
    fn test_provider_type_display() {
        assert_eq!(Provider::WeatherApi.to_string(), "WeatherApi");
        assert_eq!(Provider::OpenWeather.to_string(), "OpenWeather");
        assert_eq!(Provider::Mock.to_string(), "MockWeather");
        assert_eq!(Provider::GrpcMock.to_string(), "GrpcMockWeather");
    }
}
