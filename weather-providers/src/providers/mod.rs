mod grpc_mock;
mod mock;
mod open_weather;
mod weather_api;

pub use self::{
    grpc_mock::GrpcMockProvider, mock::MockProvider, open_weather::OpenWeatherProvider,
    weather_api::WeatherApiProvider,
};
