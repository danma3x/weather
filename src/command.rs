#[allow(dead_code)]
pub enum TimeOffset {
    Now,
    HourOffset(isize),
    DayOffset(isize),
}

impl Default for TimeOffset {
    fn default() -> Self {
        Self::Now
    }
}

#[allow(dead_code)]
pub struct WeatherCommand {
    location: String,
    time_offset: TimeOffset,
}

impl WeatherCommand {
    pub fn new<S: Into<String>>(location: S) -> Self {
        Self {
            location: location.into(),
            time_offset: TimeOffset::default(),
        }
    }
}
