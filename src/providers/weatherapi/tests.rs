use super::{api::*, *};
use crate::command::WeatherCommand;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

const CURRENT_MOCK: &'static str = include_str!("./fixtures/current.json");
const FORECAST_MOCK: &'static str = include_str!("./fixtures/forecast.json");
const HISTORY_MOCK: &'static str = include_str!("./fixtures/history.json");

fn make_weatherapi<S: Into<String>>(base_url: S, api_key: S) -> WeatherAPIProvider {
    WeatherAPIProvider::default()
        .with_base_url(base_url)
        .with_api_key(api_key)
}

#[tokio::test]
async fn test_current_request() {
    let mock_server = MockServer::start().await;

    let search_mock = Mock::given(method("GET"))
        .and(path("/v1/current.json"))
        .and(query_param("q", "Zaporizhzhia"))
        .and(query_param("key", "22222"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_string(CURRENT_MOCK),
        );

    mock_server.register(search_mock).await;
    let weatherapi = make_weatherapi(
        format!("http://{}", mock_server.address().to_string()),
        "22222".into(),
    );

    let wc = WeatherCommand::new("Zaporizhzhia", DateOffsetRepresentation::Now);

    let req_loc_search_res = weatherapi.request_current(&wc.location).await;
    let req_loc_search = req_loc_search_res.expect("Location search request completely failed");

    let _demarshaled_response = parse_current(req_loc_search)
        .await
        .expect("Couldn't parse the response at all");
}

#[test]
fn test_parse_current() {
    let current =
        serde_json::from_str::<current::Json>(CURRENT_MOCK.into()).expect("Couldn't parse");
    assert_eq!(current.location.name, "Zaporizhzhya");
    assert_eq!(current.current.wind_kph, 23.4);
    assert_eq!(current.current.temp_c, 7.5);
    assert_eq!(current.current.condition.text, "Overcast".to_owned());
}

#[test]
fn test_parse_forecast() {
    let forecast =
        serde_json::from_str::<forecast::Json>(FORECAST_MOCK.into()).expect("Couldn't parse");
    assert_eq!(forecast.location.name, "Zaporizhzhya");
    assert_eq!(forecast.forecast.forecastday[0].day.maxwind_kph, 27.4);
    assert_eq!(forecast.forecast.forecastday[0].day.maxwind_mph, 17.0);
    assert_eq!(forecast.forecast.forecastday[0].day.avghumidity, 50.0);
}

#[test]
fn test_parse_history() {
    let history =
        serde_json::from_str::<history::Json>(HISTORY_MOCK.into()).expect("Couldn't parse");
    assert_eq!(history.location.name, "Zaporizhzhya");
}

// not really needed, but I had to check
#[test]
fn test_reqwest_querystring() {
    let cl = reqwest::Client::new();
    let req = cl
        .get("http://testme.t")
        .query(&[("test", "val")])
        .query(&[("test2", "val2")])
        .build();
    assert_eq!(
        req.expect("Couldn't build a request").url().as_str(),
        "http://testme.t/?test=val&test2=val2"
    )
}
