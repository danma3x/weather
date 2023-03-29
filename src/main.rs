use std::thread::sleep_ms;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::{Input, Select};
use weather::configuration::{open_or_default, AvailableProviders, Configuration};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    Get { location: String },
    Configure { provider: AvailableProviders },
}

fn select_provider(forced: bool) -> Result<AvailableProviders> {
    let providers = &["AccuWeather"];
    if forced {
        println!("You don't have a default provider set");
    }
    let selection = Select::new()
        .with_prompt("Please select a default provider")
        .items(providers)
        .interact_opt()
        .context("Dialoguer error")?
        .context("You haven't selected a provider.")?;
    let selected_provider = providers
        .get(selection)
        .expect("How did you manage to select outside of the list?")
        .clone();
    Ok(AvailableProviders::from(selected_provider))
}

fn get_api_key() -> Result<String> {
    let api_key = Input::new()
        .with_prompt("Please enter your API key")
        .interact_text()
        .context("API key not entered");
    api_key
}

#[async_std::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut configuration = match open_or_default() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e.to_string());
            Configuration::default()
        }
    };
    match args.action {
        Action::Configure { provider } => {
            let api_key = get_api_key()?;
            match provider {
                AvailableProviders::AccuWeather => {
                    configuration.accuweather_api_key = Some(api_key)
                }
                _ => (),
            }
        }
        Action::Get { location } => {
            if let AvailableProviders::Nothing = configuration.default_provider {
                configuration.default_provider = select_provider(true)?;
            } else {
            }
        }
    }
    configuration.save()?;
    Ok(())
}
