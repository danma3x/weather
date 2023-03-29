use anyhow::{Context, Result};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

impl From<&str> for AvailableProviders {
    fn from(value: &str) -> Self {
        match value {
            "AccuWeather" => AvailableProviders::AccuWeather,
            _ => AvailableProviders::Nothing,
        }
    }
}

/// Enumeration of weather provider adapters
#[derive(Serialize, Deserialize, Clone, ValueEnum, Debug)]
pub enum AvailableProviders {
    #[value(skip)]
    Nothing,
    AccuWeather,
}

impl Default for AvailableProviders {
    fn default() -> Self {
        AvailableProviders::Nothing
    }
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub default_provider: AvailableProviders,
    pub accuweather_api_key: Option<String>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            default_provider: Default::default(),
            accuweather_api_key: None,
        }
    }
}

fn config_path() -> Result<PathBuf> {
    let mut config_path = dirs::config_dir().context("Couldn't find the config path")?;
    config_path.push("weather");
    fs::create_dir_all(&config_path)?;
    config_path.push("config.json");
    Ok(config_path)
}

pub fn open_or_default() -> Result<Configuration> {
    let path = config_path()?;
    let config = match File::open(path) {
        Ok(f) => {
            let rdr = BufReader::new(f);
            serde_json::from_reader(rdr).context("Couldn't parse the configuration file")?
        }
        _ => Configuration::default(),
    };

    Ok(config)
}

impl Configuration {
    pub fn save(&self) -> Result<()> {
        let path = config_path()?;
        let file = File::create(path).context("Config file was not found")?;
        let wrtr = BufWriter::new(file);
        serde_json::to_writer_pretty(wrtr, &self).context("Couldn't save the config file")?;
        Ok(())
    }
}
