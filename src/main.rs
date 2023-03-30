use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{Input, Select};
use weather::configuration::{open_or_default, Configuration};
use weather::types::AvailableProviders;

mod args {

    use clap::{FromArgMatches, Parser, Subcommand};
    use weather::{command::DateRepresentation, types::AvailableProviders};
    #[derive(Parser, Debug)]
    pub struct GetAction {
        location: String,
        #[clap()]
        date: DateRepresentation,
    }

    #[derive(Subcommand, Debug)]
    pub enum Action {
        Get(GetAction),
        Configure { provider: AvailableProviders },
    }

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    pub struct Args {
        #[command(subcommand)]
        pub action: Action,
    }
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
    let provider_opt = AvailableProviders::from_string(selected_provider);
    Ok(provider_opt.context("Provider has not been selected")?)
}

/// synchronous API key prompt
fn get_api_key() -> Result<String> {
    let api_key = Input::new()
        .with_prompt("Please enter your API key")
        .interact_text()
        .context("API key not entered");
    api_key
}

#[async_std::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();
    let mut configuration = match open_or_default() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e.to_string());
            Configuration::default()
        }
    };
    match args.action {
        args::Action::Configure { provider } => {
            let api_key = get_api_key()?;
            match provider {
                AvailableProviders::AccuWeather => {
                    configuration.set_accuweather_api_key(Some(api_key))
                }
            }
        }
        args::Action::Get(_get_action) => {
            if let None = configuration.default_provider {
                let provider = select_provider(true)?;
                configuration.default_provider = Some(provider);
            } else {
            }
        }
    }
    configuration.save()?;
    Ok(())
}
