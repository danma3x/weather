use super::*;
use crate::command::WeatherCommand;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

const LOCATION_KEY: &'static str = "326514";

/// Mock data
pub const LOCATION_SEARCH_MOCK: &'static str = include_str!("./fixtures/location.json");
pub const LOCATION_WEATHER_CONDITIONS_MOCK: &'static str =
    include_str!("./fixtures/current_weather_conditions.json");

fn make_accuweather<S: Into<String>>(base_url: S, api_key: S) -> AccuWeatherProvider {
    AccuWeatherProvider::new()
        .with_base_url(base_url)
        .with_api_key(api_key)
}

#[tokio::test]
async fn test_location_request() {
    let mock_server = MockServer::start().await;

    let search_mock = Mock::given(method("GET"))
        .and(path("/locations/v1/search"))
        .and(query_param("q", "Zaporizhzhia"))
        .and(query_param("apikey", "22222"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_string(LOCATION_SEARCH_MOCK),
        );

    mock_server.register(search_mock).await;
    // let api_key = real_credentials();
    // let accuweather_inst = AccuWeatherProvider::new().with_api_key(api_key);
    let accuweather_inst = make_accuweather(
        format!("http://{}", mock_server.address().to_string()),
        "22222".into(),
    );

    let wc = WeatherCommand::new(
        "Zaporizhzhia",
        crate::command::DateOffsetRepresentation::Now,
    );

    let req_loc_search_res = accuweather_inst.request_location_search(&wc).await;
    let req_loc_search = req_loc_search_res.expect("Location search request completely failed");

    let demarshaled_response = parse_location_search(req_loc_search)
        .await
        .expect("Couldn't parse the response at all");
    let location_item = demarshaled_response.first().expect("Parse result is empty");
    assert_eq!(LOCATION_KEY, location_item.key);
}

#[tokio::test]
async fn test_current_weather_conditions_request() {
    let mock_server = MockServer::start().await;

    let search_mock = Mock::given(method("GET"))
        .and(path("/currentconditions/v1/326514"))
        .and(query_param("apikey", "22222"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_string(LOCATION_WEATHER_CONDITIONS_MOCK),
        );

    mock_server.register(search_mock).await;
    // let api_key = real_credentials();
    // let accuweather_inst = AccuWeatherProvider::new().with_api_key(api_key);
    let accuweather_inst = make_accuweather(
        format!("http://{}", mock_server.address().to_string()),
        "22222".into(),
    );

    // let wc = WeatherCommand::new("Zaporizhzhia", crate::command::DateRepresentation::Now);

    let req_loc_search_res = accuweather_inst
        ._request_current_weather_conditions("326514")
        .await;
    let req_loc_search = req_loc_search_res.expect("Location search request completely failed");

    let demarshaled_response = parse_weather_conditions(req_loc_search)
        .await
        .expect("Couldn't parse the response at all");
    let _location_item = demarshaled_response.first().expect("Parse result is empty");
}

#[test]
fn test_url_location_api() {
    let t = make_accuweather("http://localhost", "22222");
    assert_eq!("http://localhost/locations/v1/search", t.url_location_api());
}

#[test]

fn test_url_current_weather() {
    let t = make_accuweather("http://localhost", "22222");
    assert_eq!(
        "http://localhost/currentconditions/v1/326514",
        t.url_current_weather(LOCATION_KEY)
    );
}
