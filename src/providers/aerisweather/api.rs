use chrono::Utc;
use serde::Deserialize;

pub mod hourly {
    use super::*;

    /// Current and historical weather conditions API item
    /// Root of Json response
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Json {
        pub success: bool,
        pub response: Vec<Response>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Periods {
        #[serde(with = "chrono::serde::ts_seconds")]
        pub timestamp: chrono::DateTime<Utc>,
        pub temp_c: f64,
        pub temp_f: f64,
        pub wind_dir: String,
        pub weather: String,
        pub humidity: usize,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Response {
        pub periods: Vec<Periods>,
    }
}

pub mod daily {
    use super::*;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Periods {
        #[serde(with = "chrono::serde::ts_seconds")]
        pub timestamp: chrono::DateTime<Utc>,
        pub wind_speed: WindSpeed,
        pub humidity: Humidity,
        pub pressure: Pressure,
        pub temp: Temp,
        pub weather: Weather,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Weather {
        pub phrase: String,
        pub primary: String,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Humidity {
        pub max: f64,
        pub min: f64,
        pub avg: f64,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WindSpeed {
        pub max_dir: String,
        pub min_dir: String,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Pressure {
        pub max_m_b: f64,
        pub min_m_b: f64,
        pub avg_m_b: f64,
        pub max_i_n: f64,
        pub min_i_n: f64,
        pub avg_i_n: f64,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Temp {
        pub avg_c: f64,
        pub avg_f: f64,
        pub min_c: f64,
        pub min_f: f64,
        pub max_c: f64,
        pub max_f: f64,
    }

    /// Root of Json response
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Json {
        pub success: bool,
        pub response: Vec<Response>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Response {
        pub periods: Vec<Periods>,
    }
}
