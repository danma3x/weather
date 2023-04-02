use crate::{
    command::{DateOffsetRepresentation, WeatherCommand},
    report::Report,
    types::Provider,
};
use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use reqwest::Response;
mod api;
mod report;

#[cfg(test)]
mod tests;

/// WeatherAPI REST API adapter
pub struct WeatherAPIProvider {
    base_url: String,
    api_key: String,
    client: reqwest::Client,
}

impl Default for WeatherAPIProvider {
    /// Create empty provider
    fn default() -> Self {
        Self {
            base_url: "https://api.weatherapi.com".to_owned(),
            api_key: "".to_owned(),
            client: reqwest::Client::new(),
        }
    }
}

impl WeatherAPIProvider {
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

    fn url_current(&self) -> String {
        format!("{}/{}", self.base_url, "v1/current.json")
    }

    fn _url_history(&self) -> String {
        format!("{}/{}", self.base_url, "v1/history.json")
    }

    fn _url_forecast(&self) -> String {
        format!("{}/{}", self.base_url, "v1/forecast.json")
    }
}

impl WeatherAPIProvider {
    async fn request_current(&self, location: &String) -> Result<Response, reqwest::Error> {
        self.client
            .get(self.url_current())
            .query(&[("key", &self.api_key), ("q", location)])
            .query(&[("aqi", "no")])
            .send()
            .await
    }

    async fn request_forecast(
        &self,
        location: &String,
        days: isize,
    ) -> Result<Response, reqwest::Error> {
        self.client
            .get(self._url_forecast())
            .query(&[("key", &self.api_key), ("q", location)])
            .query(&[("aqi", "no"), ("alerts", "no")])
            .query(&[("days", days)])
            .send()
            .await
    }

    //TODO: handle dt parameter
    async fn request_history(
        &self,
        location: &String,
        dt: &String,
    ) -> Result<Response, reqwest::Error> {
        self.client
            .get(self._url_history())
            .query(&[("key", &self.api_key), ("q", location)])
            .query(&[("aqi", "no"), ("alerts", "no")])
            .query(&[("dt", dt)])
            .send()
            .await
    }
}

impl WeatherAPIProvider {
    async fn branch_current(&self, location: String) -> Result<Report> {
        log::debug!("branch_current");
        let res = self
            .request_current(&location)
            .await
            .context("Failed the current weather request")?;
        let parsed = parse_current(res)
            .await
            .context("Failed to parse the current weather response")?;
        let report = report::report_current(parsed);
        Ok(report)
    }

    async fn branch_forecast(&self, location: &String, days: isize) -> Result<Report> {
        log::debug!("branch_forecast");
        let res = self
            .request_forecast(location, days)
            .await
            .context("Failed the forecast weather request")?;
        let parsed = parse_forecast(res)
            .await
            .context("Failed to parse the current weather response")?;
        let report = report::report_forecast(parsed);
        Ok(report)
    }

    async fn branch_history(&self, location: &String, dt: &String) -> Result<Report> {
        log::debug!("branch_history");
        log::debug!("date: {}", dt);
        let res = self
            .request_history(location, dt)
            .await
            .context("Failed the history weather request")?;
        let parsed = parse_history(res)
            .await
            .context("Failed to parse the current weather response")?;
        let report = report::report_history(parsed);
        Ok(report)
    }
}

async fn parse_current(response: Response) -> Result<api::current::Json> {
    response
        .json::<api::current::Json>()
        .await
        .context("Couldn't parse the current weather status")
}
async fn parse_forecast(response: Response) -> Result<api::forecast::Json> {
    response
        .json::<api::forecast::Json>()
        .await
        .context("Couldn't parse the current weather status")
}
async fn parse_history(response: Response) -> Result<api::history::Json> {
    response
        .json::<api::history::Json>()
        .await
        .context("Couldn't parse the current weather status")
}

#[async_trait]
impl Provider for WeatherAPIProvider {
    async fn run(&self, wc: WeatherCommand) -> Result<Report> {
        log::debug!("{:?}", self.api_key);
        log::debug!("{:?}", wc.location);
        match wc.date {
            DateOffsetRepresentation::Now => self.branch_current(wc.location).await,
            DateOffsetRepresentation::DayOffset(days) => {
                if days > 0 {
                    self.branch_forecast(&wc.location, days).await
                } else {
                    let dt = wc
                        .date
                        .to_chrono(chrono::Utc::now().into())
                        .format("%Y-%m-%d")
                        .to_string();
                    self.branch_history(&wc.location, &dt).await
                }
            }
            DateOffsetRepresentation::HourOffset(_h) => {
                bail!("Hourly offsets are not supported yet")
            }
            _ => bail!("Other date offsets are not implemented yet"),
        }
    }
}
