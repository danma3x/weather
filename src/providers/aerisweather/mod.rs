use crate::{
    command::{DateOffsetRepresentation, WeatherCommand},
    report::Report,
    types::Provider,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::Utc;
use reqwest::Response;

mod api;
#[cfg(test)]
mod tests;

mod report;

pub struct AerisWeatherProvider {
    base_url: String,
    client: reqwest::Client,
    client_id: String,
    client_secret: String,
}

impl Default for AerisWeatherProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AerisWeatherProvider {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.aerisapi.com/".to_owned(),
            client_id: "".to_owned(),
            client_secret: "".to_owned(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn with_credentials<S: Into<String>>(mut self, client_id: S, client_secret: S) -> Self {
        self.client_id = client_id.into();
        self.client_secret = client_secret.into();
        self
    }
}

impl AerisWeatherProvider {
    fn url_daily(&self, location: &str) -> String {
        format!("{}/{}/{}", self.base_url, "conditions/summary", location)
    }

    fn url_hourly(&self, location: &str) -> String {
        format!("{}/{}/{}", self.base_url, "conditions", location)
    }
}

async fn parse_hourly(response: Response) -> Result<api::hourly::Json> {
    response
        .json::<api::hourly::Json>()
        .await
        .context("Couldn't parse the hourly status")
}

async fn parse_daily(response: Response) -> Result<api::daily::Json> {
    response
        .json::<api::daily::Json>()
        .await
        .context("Couldn't parse the hourly status")
}

impl AerisWeatherProvider {
    async fn branch_hourly(&self, location: &str, hours: isize) -> Result<Report> {
        let hour_offset_to_str = || {
            if hours > 0 {
                format!("+{}hours", hours)
            } else {
                format!("{}hours", hours)
            }
        };
        match hours {
            hours if hours > 0 => {
                let mut r = Report::new("AerisWeather - forecast(hourly)");
                let res = self
                    .request_hourly(location, &hour_offset_to_str())
                    .await
                    .context("Failed to make hourly API request")?;
                let parsed = parse_hourly(res).await?;
                report::report_hourly(&mut r, parsed);
                Ok(r)
            }
            hours if hours < 0 => {
                let mut r = Report::new("AerisWeather - history(hourly)");
                let res = self
                    .request_hourly(location, &hour_offset_to_str())
                    .await
                    .context("Failed to make hourly API request")?;
                let parsed = parse_hourly(res).await?;
                report::report_hourly(&mut r, parsed);
                Ok(r)
            }
            _ => {
                let mut r = Report::new("AerisWeather - current");
                let res = self
                    .request_hourly(location, "now")
                    .await
                    .context("Failed to make hourly API request")?;
                let parsed = parse_hourly(res).await?;
                report::report_hourly(&mut r, parsed);
                Ok(r)
            }
        }
    }

    async fn branch_daily(&self, location: &str, days: isize) -> Result<Report> {
        let day_offset_to_str = || {
            DateOffsetRepresentation::DayOffset(days)
                .to_chrono(Utc::now().into())
                .format("%Y/%m/%d")
                .to_string()
        };
        let precalculated_offset = day_offset_to_str();
        if days > 0 {
            let mut r = Report::new("AerisWeather - forecast(daily)");
            let res = self
                .request_daily(location, &precalculated_offset)
                .await
                .context("Failed to make daily API request")?;
            let parsed = parse_daily(res).await?;
            report::report_daily(&mut r, parsed);
            Ok(r)
        } else {
            let mut r = Report::new("AerisWeather - history(daily)");
            let res = self
                .request_daily(location, &precalculated_offset)
                .await
                .context("Failed to make daily API request")?;
            let parsed = parse_daily(res).await?;
            report::report_daily(&mut r, parsed);
            Ok(r)
        }
    }

    async fn branch_current(&self, location: &str) -> Result<Report> {
        let mut r = Report::new("AerisWeather - current");
        let res = self
            .request_hourly(location, "now")
            .await
            .context("Failed to make hourly API request")?;
        let parsed = parse_hourly(res).await?;
        report::report_hourly(&mut r, parsed);
        Ok(r)
    }
}

impl AerisWeatherProvider {
    async fn request_hourly(
        &self,
        location: &str,
        for_param: &str,
    ) -> Result<Response, reqwest::Error> {
        self.client
            .get(self.url_hourly(location))
            .query(&[
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
            ])
            .query(&[("plimit", "1"), ("format", "json"), ("filter", "1min")])
            .query(&[("for", for_param)])
            .send()
            .await
    }

    async fn request_daily(
        &self,
        location: &str,
        for_param: &str,
    ) -> Result<Response, reqwest::Error> {
        self.client
            .get(self.url_daily(location))
            .query(&[
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
            ])
            .query(&[("format", "json")])
            .query(&[("for", for_param)])
            .send()
            .await
    }
}

#[async_trait]
impl Provider for AerisWeatherProvider {
    async fn run(&self, wc: WeatherCommand) -> Result<Report> {
        match wc.date {
            DateOffsetRepresentation::DayOffset(days) => {
                if days == 0 {
                    self.branch_current(&wc.location).await
                } else {
                    self.branch_daily(&wc.location, days).await
                }
            }
            DateOffsetRepresentation::HourOffset(hours) => {
                if hours == 0 {
                    self.branch_current(&wc.location).await
                } else {
                    self.branch_hourly(&wc.location, hours).await
                }
            }
            DateOffsetRepresentation::Now => self.branch_hourly(&wc.location, 0).await,
        }
    }
}
