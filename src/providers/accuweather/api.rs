use chrono::Utc;
use serde::Deserialize;

///Location API parse primitive
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationSearchItem {
    pub key: String,
}

/// Current and historical weather conditions API item
#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WeatherConditions {
    weather_text: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    epoch_time: chrono::DateTime<Utc>,
}

#[allow(dead_code)]
pub type WeatherConditionsResponse = Vec<WeatherConditions>;

pub type LocationSearchResponse = Vec<LocationSearchItem>;

/// Current and historical weather conditions API item
#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ForecastHourly {
    icon_phrase: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    epoch_date_time: chrono::DateTime<Utc>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Temperature {}
