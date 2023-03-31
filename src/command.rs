/// A simple abstraction for types of dates a user might want to enter
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DateRepresentation {
    Now,
    HourOffset(isize),
    Date(chrono::NaiveDate),
}

impl Default for DateRepresentation {
    fn default() -> Self {
        Self::Now
    }
}

/// Bundle of data that is to be used
pub struct WeatherCommand {
    pub location: String,
    pub time_offset: DateRepresentation,
}

impl WeatherCommand {
    pub fn new<S: Into<String>>(location: S) -> Self {
        Self {
            location: location.into(),
            time_offset: DateRepresentation::default(),
        }
    }
}
