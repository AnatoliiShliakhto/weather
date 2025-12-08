use ::serde::Deserialize;

// #[derive(Deserialize)]
// pub struct OpenWeatherResponse {
//     pub name: String,
//     pub weather: Vec<OpenWeatherWeather>,
//     pub main: OpenWeatherMain,
//     pub sys: OpenWeatherSys,
// }
//
// #[derive(Deserialize)]
// pub struct OpenWeatherWeather {
//     pub main: String,
//     pub description: String,
// }
//
// #[derive(Deserialize)]
// pub struct OpenWeatherMain {
//     pub temp: f32,
//     pub humidity: u8,
// }
//
// #[derive(Deserialize)]
// pub struct OpenWeatherSys {
//     pub country: String,
// }

#[derive(Debug, Deserialize)]
pub struct OpenWeatherGeoResponse {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
}

#[derive(Deserialize)]
pub struct OpenWeatherResponse {
    pub temperature: OpenWeatherTemperature,
    pub humidity: OpenWeatherHumidity,
}

#[derive(Deserialize)]
pub struct OpenWeatherHumidity {
    pub afternoon: u8,
}

#[derive(Deserialize)]
pub struct OpenWeatherTemperature {
    pub afternoon: f32,
}
