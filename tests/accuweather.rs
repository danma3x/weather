use weather::providers::accuweather::AccuWeatherProvider;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[async_std::test]
async fn test_location_search_request() {
    let mock_server = MockServer::start().await;
    let search_mock = Mock::given(method("GET"))
        .and(path("/locations/v1/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(LocationSearchMockData));
    mock_server.register(search_mock).await;
    let accuweather_inst = AccuWeatherProvider::new()
        .with_api_key("22222")
        .with_base_url(mock_server.address().to_string());
    accuweather_inst.request_location_search();
}

pub const LocationSearchMockData: &'static str = include_str!("./fixtures/accuweather_search.json");
