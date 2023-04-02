use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::Input;
use weather::configuration::{open_or_default, Configuration};
use weather::providers::accuweather::AccuWeatherProvider;
use weather::providers::weatherapi::WeatherAPIProvider;
use weather::types::{AvailableProviders, Provider};
use weather::{args, util};

/// synchronous API key prompt
fn get_api_key() -> Result<String> {
    let api_key = Input::new()
        .with_prompt("Please enter your API key")
        .interact_text()
        .context("API key has not been entered");
    api_key
}

/// Handles provider configuration
fn handle_condigure(configuration: &mut Configuration, provider: AvailableProviders) -> Result<()> {
    let api_key = get_api_key()?;
    match provider {
        AvailableProviders::AccuWeather => configuration.set_accuweather_api_key(Some(api_key)),
        AvailableProviders::WeatherAPI => configuration.set_weatherapi_api_key(Some(api_key)),
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
        AvailableProviders::AccuWeather => {
            let api_key = configuration
                .accuweather_api_key
                .clone()
                .context("You haven't set AccuWeather API key")?;
            AccuWeatherProvider::new()
                .with_api_key(api_key)
                .run(weather_command)
                .await
        }
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
