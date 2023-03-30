use reqwest::Response;
use serde::Deserialize;

use crate::{command::WeatherCommand, report::Report, types::Provider};
use anyhow::{Context, Result};

/// AccuWeather REST API adapter
pub struct AccuWeatherProvider {
    base_url: String,
    api_key: String,
    client: reqwest::Client,
}

impl Default for AccuWeatherProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AccuWeatherProvider {
    /// Create empty provider
    pub fn new() -> Self {
        Self {
            base_url: "http://dataservice.accuweather.com/locations/v1/cities/search".to_owned(),
            api_key: "".to_owned(),
            client: reqwest::Client::new(),
        }
    }
    /// Attach non-default API URI adapter, used for mocks
    pub fn with_base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.base_url = base_url.into();
        self
    }
    /// Attach AccuWeather API key to the adapter
    pub fn with_api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = api_key.into();
        self
    }

    /// Build a Location API URL given the location name
    fn build_location_search_request<S: AsRef<str>>(&self, location: S) -> String {
        format!("{}/{}", self.base_url, "locations/v1/search")
    }
}

impl AccuWeatherProvider {
    /// Unmarshal Location API response to get a location id
    pub async fn parse_location_search(
        &self,
        response: Response,
    ) -> Result<LocationSearchResponse> {
        response
            .json::<LocationSearchResponse>()
            .await
            .context("Couldn't parse the Location API Response")
    }

    /// Make a Location API request
    pub async fn request_location_search(&self) -> Result<Response, reqwest::Error> {
        self.client
            .get(self.build_location_search_request(""))
            .send()
            .await
    }
}

impl Provider for AccuWeatherProvider {
    fn run(&self, command: WeatherCommand) -> Result<Report> {
        unimplemented!("Accuweather control flow not added yet");
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationSearchItem {
    key: String,
}

pub type LocationSearchResponse = Vec<LocationSearchItem>;

#[cfg(test)]
mod tests {

    use super::*;
    fn accuweather_fixture<S: Into<String>>(base_url: S, api_key: S) -> AccuWeatherProvider {
        AccuWeatherProvider::new()
            .with_base_url(base_url)
            .with_api_key(api_key)
    }

    #[test]
    fn test_build_location_search_request() {
        let t = accuweather_fixture("http://localhost", "22222");
        assert_eq!(
            "http://localhost/locations/v1/search",
            t.build_location_search_request("Zaporizhzhia")
        );
    }
}
