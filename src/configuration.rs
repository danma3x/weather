use crate::types::AvailableProviders;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

/// Application configuration
#[derive(Serialize, Deserialize, Default)]
pub struct Configuration {
    #[serde(skip)]
    config_path: Option<PathBuf>,
    pub default_provider: Option<AvailableProviders>,
    accuweather_api_key: Option<String>,
}

/// Try to obtain the config path
fn obtain_default_os_config_path() -> Result<PathBuf> {
    let mut config_path = dirs::config_dir().context("Couldn't find the config path")?;
    config_path.push("weather");
    if config_path.exists() {
        fs::create_dir_all(&config_path)?;
    }
    config_path.push("config.json");
    Ok(config_path)
}

/// Attempt to open a config file at a specified location or revert to using an XDG-compliant configuration directory
pub fn open_or_default(path_opt: Option<PathBuf>) -> Result<Configuration> {
    let path = match path_opt {
        Some(p) => p,
        None => obtain_default_os_config_path()?,
    };
    if path.exists() {
        let cfg_file = File::open(path).context("Couldn't open the configuration file")?;
        let rdr = BufReader::new(cfg_file);
        let read_cfg =
            serde_json::from_reader(rdr).context("Couldn't parse the configuration file")?;
        Ok(read_cfg)
    } else {
        Ok(Configuration {
            config_path: Some(path),
            ..Default::default()
        })
    }
}

impl Configuration {
    pub fn set_accuweather_api_key(&mut self, api_key_opt: Option<String>) {
        self.accuweather_api_key = api_key_opt;
    }

    pub fn set_default_provider(&mut self, provider_opt: Option<AvailableProviders>) {
        self.default_provider = provider_opt;
    }

    /// Attempt to save a config file to a default location
    pub fn save(&self) -> Result<()> {
        let path = match &self.config_path {
            Some(p) => p.clone(),
            None => obtain_default_os_config_path()
                .context("Was unable to obtain OS-dependent config path")?,
        };
        let file = File::create(path).context("Config file was not found")?;
        let wrtr = BufWriter::new(file);
        serde_json::to_writer_pretty(wrtr, &self).context("Couldn't save the config file")?;
        Ok(())
    }

    pub fn with_config_path(mut self, path_opt: Option<PathBuf>) -> Self {
        self.config_path = path_opt;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn obtain_username() -> String {
        std::env::var("USER").expect("Was not able to obtain a username")
    }

    #[test]

    fn test_obtain_default_os_config_path() {
        let user = obtain_username();
        #[cfg(target_os = "linux")]
        let expected_path_end = format!("{}/.config/weather/config.json", user);
        #[cfg(target_os = "windows")]
        let expected_path_end = format!(r"Users\{}\AppData\Roaming\weather\config.json", user);
        #[cfg(target_os = "macos")]
        let expected_path_end = format!(
            r"/Users/{}/Library/Application Support/weather/config.json",
            user
        );

        // Lin: Some(/home/alice/.config)
        // Win: Some(C:\Users\Alice\AppData\Roaming)
        // Mac: Some(/Users/Alice/Library/Application Support)

        assert_eq!(
            obtain_default_os_config_path()
                .expect("Unable to obtain the path")
                .as_path()
                .ends_with(expected_path_end),
            true
        );
    }

    #[test]
    fn test_save_load_cycle() {
        let mut config = Configuration::default().with_config_path(Some(".tmp/test.json".into()));
        config.set_accuweather_api_key(Some("11111".to_owned()));
        config.set_default_provider(Some(AvailableProviders::AccuWeather));
        fs::create_dir_all(".tmp").expect("Wasn't able to create a temporary test directory");
        config.save().expect("Was unable to save a config");
        let config =
            open_or_default(Some(".tmp/test.json".into())).expect("Couldn't open a config file");
        assert_eq!(config.accuweather_api_key, Some("11111".into()));
        assert_eq!(
            config.default_provider,
            Some(AvailableProviders::AccuWeather)
        );
    }
}
