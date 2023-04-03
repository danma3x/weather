use super::api;

const HOURLY_MOCK: &'static str = include_str!("./fixtures/hourly.json");
const DAILY_MOCK: &'static str = include_str!("./fixtures/daily.json");

#[test]
fn test_parse_hourly() {
    let _hourly = serde_json::from_str::<api::hourly::Json>(HOURLY_MOCK).expect("Failed to parse");
}

#[test]
fn test_parse_daily() {
    let _daily = serde_json::from_str::<api::daily::Json>(DAILY_MOCK).expect("Failed to parse");
}
