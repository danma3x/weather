use self::api::{LocationSearchResponse, WeatherConditionsResponse};
use crate::{command::WeatherCommand, report::Report, types::Provider};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Response;

mod api;

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
            base_url: "http://dataservice.accuweather.com/".to_owned(),
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
    fn url_location_api(&self) -> String {
        format!("{}/{}", self.base_url, "locations/v1/search")
    }

    #[allow(dead_code)]
    fn url_current_weather<S: AsRef<str>>(&self, location: S) -> String {
        format!(
            "{}/{}/{}",
            self.base_url,
            "currentconditions/v1",
            location.as_ref()
        )
    }
}

impl AccuWeatherProvider {
    /// Unmarshal Location API response to get a location id

    /// Make a Location API request
    async fn request_location_search(
        &self,
        command: &WeatherCommand,
    ) -> Result<Response, reqwest::Error> {
        self.client
            .get(self.url_location_api())
            .query(&[("apikey", &self.api_key), ("q", &command.location)])
            .send()
            .await
    }

    async fn _request_current_weather_conditions<S: AsRef<str>>(
        &self,
        location: S,
    ) -> Result<Response, reqwest::Error> {
        self.client
            .get(self.url_current_weather(location))
            .query(&[("apikey", &self.api_key)])
            .send()
            .await
    }

    async fn _request_forecast_weather_hourly<S: AsRef<str>>(
        &self,
        _location: S,
    ) -> Result<Response, reqwest::Error> {
        unimplemented!()
    }

    async fn _request_forecast_weather_daily<S: AsRef<str>>(
        &self,
        _location: S,
    ) -> Result<Response, reqwest::Error> {
        unimplemented!()
    }

    async fn _request_historical_weather_daily<S: AsRef<str>>(
        &self,
        _location: S,
    ) -> Result<Response, reqwest::Error> {
        unimplemented!()
    }
}

async fn parse_location_search(response: Response) -> Result<LocationSearchResponse> {
    response
        .json::<api::LocationSearchResponse>()
        .await
        .context("Couldn't parse the Location API Response")
}

#[allow(dead_code)]
async fn parse_weather_conditions(response: Response) -> Result<WeatherConditionsResponse> {
    response
        .json::<api::WeatherConditionsResponse>()
        .await
        .context("Couldn't parse the Weather condition in response")
}

#[async_trait]
impl Provider for AccuWeatherProvider {
    async fn run(&self, command: WeatherCommand) -> Result<Report> {
        let location_resp = self
            .request_location_search(&command)
            .await
            .context("Unable to make a location query")?;
        let _location = parse_location_search(location_resp);

        unimplemented!("");
    }
}

#[cfg(test)]
mod tests;
