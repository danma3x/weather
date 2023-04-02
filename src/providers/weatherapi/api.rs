use chrono::Utc;
use serde::Deserialize;

pub mod current {
    use super::*;
    #[derive(Deserialize, Debug)]
    pub struct Json {
        pub location: Location,
        pub current: Current,
    }

    #[derive(Deserialize, Debug)]
    pub struct Current {
        #[serde(with = "chrono::serde::ts_seconds")]
        pub last_updated_epoch: chrono::DateTime<Utc>,
        pub temp_c: f64,
        pub temp_f: f64,
        pub wind_kph: f64,
        pub wind_mph: f64,
        pub wind_dir: String,
        pub humidity: usize,
        pub condition: Condition,
    }

    #[derive(Deserialize, Debug)]
    pub struct Condition {
        pub text: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct Location {
        pub name: String,
    }
}

pub mod forecast {
    use super::*;
    #[derive(Deserialize, Debug)]
    pub struct Json {
        pub location: Location,
        pub current: Current,
        pub forecast: Forecast,
    }
    #[derive(Deserialize, Debug)]
    pub struct Current {
        #[serde(with = "chrono::serde::ts_seconds")]
        pub last_updated_epoch: chrono::DateTime<Utc>,
        pub temp_c: f64,
        pub temp_f: f64,
        pub wind_kph: f64,
        pub wind_mph: f64,
        pub wind_dir: String,
        pub humidity: usize,
        pub condition: Condition,
    }
    #[derive(Deserialize, Debug)]
    pub struct Condition {
        pub text: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct Forecast {
        pub forecastday: Vec<ForecastDay>,
    }

    #[derive(Deserialize, Debug)]
    pub struct ForecastDay {
        #[serde(with = "chrono::serde::ts_seconds")]
        pub date_epoch: chrono::DateTime<Utc>,
        pub day: Day,
    }
    #[derive(Deserialize, Debug)]
    pub struct Location {
        pub name: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Day {
        pub maxtemp_f: f64,
        pub mintemp_f: f64,
        pub maxtemp_c: f64,
        pub mintemp_c: f64,
        pub maxwind_mph: f64,
        pub maxwind_kph: f64,
        pub avghumidity: f64,
        pub condition: Condition,
    }
}

pub mod history {

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Forecast {
        pub forecastday: Vec<ForecastDay>,
    }

    #[derive(Deserialize, Debug)]
    pub struct ForecastDay {
        #[serde(with = "chrono::serde::ts_seconds")]
        pub date_epoch: chrono::DateTime<Utc>,
        pub day: Day,
    }
    #[derive(Deserialize, Debug)]
    pub struct Day {
        pub mintemp_c: f64,
        pub maxtemp_c: f64,
        pub mintemp_f: f64,
        pub maxtemp_f: f64,
        pub avghumidity: f64,
        pub condition: Condition,
    }
    #[derive(Deserialize, Debug)]
    pub struct Condition {
        pub text: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Json {
        pub location: Location,
        pub forecast: Forecast,
    }
    #[derive(Deserialize, Debug)]
    pub struct Location {
        pub name: String,
    }
}
