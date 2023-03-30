use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use crate::types::AvailableProviders;

/// Application configuration
#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub default_provider: Option<AvailableProviders>,
    accuweather_api_key: Option<String>,
    path_override: Option<PathBuf>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            default_provider: Default::default(),
            accuweather_api_key: None,
            path_override: None,
        }
    }
}

/// Try to obtain the config path
fn config_path() -> Result<PathBuf> {
    let mut config_path = dirs::config_dir().context("Couldn't find the config path")?;
    config_path.push("weather");
    if config_path.exists() {
        fs::create_dir_all(&config_path)?;
    }
    config_path.push("config.json");
    Ok(config_path)
}

/// Attempt to open a config file at a default location
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
    pub fn set_accuweather_api_key(&mut self, api_key_opt: Option<String>) {
        self.accuweather_api_key = api_key_opt;
    }

    /// Attempt to save a config file to a default location
    pub fn save(&self) -> Result<()> {
        let path = config_path()?;
        let file = File::create(path).context("Config file was not found")?;
        let wrtr = BufWriter::new(file);
        serde_json::to_writer_pretty(wrtr, &self).context("Couldn't save the config file")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_config_path() {}
}
