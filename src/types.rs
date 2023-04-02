use crate::{command::WeatherCommand, report::Report};
use anyhow::Result;
use async_trait::async_trait;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// Any given provider should implement this in addition to his private API handling to produce a report
/// At the moment is functionally useless as we're not relying on a trait to avoid repeating code
//TODO: Maybe either get rid of AvailableProviders enum and use this trait to register providers at runtime
#[async_trait]
pub trait Provider {
    async fn run(&self, command: WeatherCommand) -> Result<Report>;
}

/// Enumeration of supported weather providers
/// Used by configuration struct and argument parser at runtime
#[derive(Serialize, Deserialize, Clone, ValueEnum, Debug, PartialEq)]
pub enum AvailableProviders {
    AccuWeather,
    WeatherAPI,
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
