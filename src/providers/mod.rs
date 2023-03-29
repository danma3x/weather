use self::accuweather::AccuWeatherProvider;
use crate::command::WeatherCommand;
use anyhow::Result;

pub mod accuweather;

/// Part of the control flow after provider to be used is determined
pub trait Provider {
    fn run(&self, command: WeatherCommand) -> Result<()>;
}
