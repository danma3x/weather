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
    pub weather_text: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub epoch_time: chrono::DateTime<Utc>,
}

pub type WeatherConditionsResponse = Vec<WeatherConditions>;

pub type LocationSearchResponse = Vec<LocationSearchItem>;

/// Current and historical weather conditions API item
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ForecastHourly {
    pub icon_phrase: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub epoch_date_time: chrono::DateTime<Utc>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Temperature {}
