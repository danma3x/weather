use chrono::Utc;
use serde::Deserialize;

/// Current and historical weather conditions API item
#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourlyData {
    success: bool,
    response: HourlyResponse,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourlyPeriod {
    #[serde(with = "chrono::serde::ts_seconds")]
    timestamp: chrono::DateTime<Utc>,
    temp_c: f64,
    temp_f: f64,
    wind_dir: String,
    weather: String,
    humidity: usize,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyPeriod {
    #[serde(with = "chrono::serde::ts_seconds")]
    timestamp: chrono::DateTime<Utc>,
    temp_c: f64,
    temp_f: f64,
    wind_dir: String,
    weather: String,
    humidity: usize,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourlyResponse {
    periods: Vec<HourlyPeriod>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyResponse {
    periods: Vec<HourlyPeriod>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyData {
    success: bool,
    response: HourlyResponse,
}
