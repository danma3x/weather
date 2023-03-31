#[derive(Deserialize)]
pub struct Location {
    name: String,
}

#[derive(Deserialize)]
pub struct Status {
    #[serde(with = "chrono::serde::ts_seconds")]
    last_updated_epoch: chrono::DateTime<Utc>,
    temp_c: f64,
    temp_f: f64,
    wind_kph: f64,
    wind_mph: f64,
    wind_dir: Stringf,
    humidity: usize,
    condition: Condition,
}

#[derive(Deserialize)]
pub struct Current {
    location: Location,
    current: Status,
}

#[derive(Deserialize)]
pub struct Condition {
    text: String,
}
