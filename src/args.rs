use crate::types::AvailableProviders;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
#[derive(Parser, Debug)]
pub struct GetArgs {
    /// A location to look up the weather conditions for
    pub location: String,
    /// An optional time offset in the form of
    pub date: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Get weather status for a given location
    Get(GetArgs),
    /// Interactive Configuration of a weather provider
    Configure { provider: AvailableProviders },
    /// Set the default provider to be used later
    Default { provider: AvailableProviders },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
    /// Specify a path for a configuration file
    #[arg(short, long)]
    pub config_path: Option<PathBuf>,
}
