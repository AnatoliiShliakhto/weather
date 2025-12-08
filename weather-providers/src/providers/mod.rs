mod mock;
mod grpc_mock;
mod weather_api;
mod open_weather;

pub use self::{
    mock::MockProvider,
    grpc_mock::GrpcMockProvider,
    weather_api::WeatherApiProvider,
    open_weather::OpenWeatherProvider,
};