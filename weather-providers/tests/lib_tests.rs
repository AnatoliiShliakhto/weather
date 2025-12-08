use ::weather_providers::{Provider, create_provider};

#[tokio::test]
async fn test_mock_provider_via_trait() {
    // This integration test ensures that the public API of the library behaves correctly.
    // It creates a Mock provider via the factory function and calls it.

    let provider = create_provider(Provider::Mock);

    let response = provider
        .get_weather(None, "New York", Some("2024-01-01"))
        .await;

    assert!(response.is_ok());

    let weather = response.unwrap();
    assert_eq!(weather.city, "Mock City");
    assert_eq!(weather.country, "Mock Country");
    assert!(weather.temperature > 0.0);
    // Ensure the date passed down matches the normalized result (assuming ISO format)
    assert_eq!(weather.date, "2024-01-01");
}

#[test]
fn test_provider_enum_parsing() {
    // Test that string conversion to enum works as expected for the public API.
    assert_eq!(Provider::try_from("ow").unwrap(), Provider::OpenWeather);
    assert_eq!(Provider::try_from("mock").unwrap(), Provider::Mock);
    assert!(Provider::try_from("invalid").is_err());
}
