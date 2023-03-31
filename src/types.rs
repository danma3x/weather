use crate::{command::WeatherCommand, report::Report};
use anyhow::Result;
use async_trait::async_trait;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// Any given provider should implement this in addition to his private API handling to produce a report
#[async_trait]
pub trait Provider {
    async fn run(&self, command: WeatherCommand) -> Result<Report>;
}

/// Enumeration of supported weather providers
#[derive(Serialize, Deserialize, Clone, ValueEnum, Debug, PartialEq)]
pub enum AvailableProviders {
    AccuWeather,
}

impl AvailableProviders {
    /// Get a nullable provider by providing a string, used in the interactive part of the app
    pub fn from_string<S: AsRef<str>>(provider_str: S) -> Option<AvailableProviders> {
        match provider_str.as_ref() {
            "AccuWeather" => Some(AvailableProviders::AccuWeather),
            _ => None,
        }
    }
}
