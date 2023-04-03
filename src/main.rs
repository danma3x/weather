use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::Password;
use weather::configuration::{open_or_default, Configuration};
use weather::providers::aerisweather::AerisWeatherProvider;
use weather::providers::weatherapi::WeatherAPIProvider;
use weather::types::{AvailableProviders, Provider};
use weather::{args, util};

/// API key prompt
fn get_api_key<S: Into<String>>(prompt: S) -> Result<String> {
    let api_key = Password::new()
        .with_prompt(prompt)
        .allow_empty_password(false)
        .interact()
        .context("API key has not been entered");
    api_key
}

/// Handles provider configuration
fn handle_condigure(configuration: &mut Configuration, provider: AvailableProviders) -> Result<()> {
    match provider {
        // AvailableProviders::AccuWeather => {
        // let api_key = get_api_key("Please enter an API key for AccuWeather")?;
        // configuration.set_accuweather_api_key(Some(api_key));
        // }
        AvailableProviders::WeatherAPI => {
            let api_key = get_api_key("Please enter an API key for WeatherAPI")?;
            configuration.set_weatherapi_api_key(Some(api_key));
        }
        AvailableProviders::AerisWeather => {
            let client_id = get_api_key("Please enter a client id for AerisWeather")?;
            let client_secret = get_api_key("Please enter a client secret for AerisWeather")?;
            configuration.set_aerisweather_client_secret(Some(client_id), Some(client_secret));
        }
    }
    Ok(())
}

/// Handles the configuration of a default provider selection
fn handle_change_default_provider(configuration: &mut Configuration, provider: AvailableProviders) {
    println!("Have set the new default provider {:?}", provider);
    configuration.set_default_provider(Some(provider));
}

/// Handles the weather provider interaction and report generation
async fn handle_get(configuration: &Configuration, get_action: args::GetArgs) -> Result<()> {
    let weather_command = util::parse_get_action(get_action);
    log::debug!("Weather command: {:?}", weather_command);
    let provider_tag = configuration.default_provider.clone().context(
            "You haven't selected a default provider yet, please run >weather configure <provider> first")?;
    let report = match provider_tag {
        // AvailableProviders::AccuWeather => {
        // let api_key = configuration
        // .accuweather_api_key
        // .clone()
        // .context("You haven't set AccuWeather API key")?;
        // AccuWeatherProvider::new()
        // .with_api_key(api_key)
        // .run(weather_command)
        // .await
        // }
        AvailableProviders::WeatherAPI => {
            let api_key = configuration
                .weatherapi_api_key
                .clone()
                .context("You haven't set WeatherAPI API key")?;
            WeatherAPIProvider::default()
                .with_api_key(api_key)
                .run(weather_command)
                .await
        }
        AvailableProviders::AerisWeather => {
            let client_id = configuration
                .aerisweather_client_id
                .clone()
                .context("You haven't set AerisWeather client id")?;
            let client_secret = configuration
                .aerisweather_client_secret
                .clone()
                .context("You haven't set AerisWeather client secret")?;
            AerisWeatherProvider::default()
                .with_credentials(client_id, client_secret)
                .run(weather_command)
                .await
        }
    }
    .context("Failed to build a report")?;
    println!("{}", report);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Don't forget the RUST_LOG next time please
    env_logger::init();

    let args = args::Args::parse();
    let mut configuration = match open_or_default(args.config_path) {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            Configuration::default()
        }
    };
    match args.action {
        args::Action::Configure { provider } => handle_condigure(&mut configuration, provider)?,
        args::Action::Default { provider } => {
            handle_change_default_provider(&mut configuration, provider)
        }
        args::Action::Get(get_action) => handle_get(&configuration, get_action).await?,
    }
    configuration.save()?;
    Ok(())
}
