use ::serde::Deserialize;

#[derive(Deserialize)]
pub struct WeatherApiResponse {
    pub location: WeatherApiLocation,
    pub current: WeatherApiCurrent,
}

#[derive(Deserialize)]
pub struct WeatherApiLocation {
    pub name: String,
    pub country: String,
}

#[derive(Deserialize)]
pub struct WeatherApiCurrent {
    pub temp_f: f32,
    pub humidity: u8,
    pub condition: WeatherApiCondition,
}

#[derive(Deserialize)]
pub struct WeatherApiCondition {
    pub text: String,
}
