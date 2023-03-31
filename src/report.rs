/// Assembled weather report data
pub struct Report {
    _title: String,
}

impl Report {
    /// Initialize a report, add fields to it via the builder pattern in your favorite weather provider adapter
    pub fn new(_title: String) -> Self {
        Self { _title }
    }
    // pub fn add_weather_condition(self) -> Self {
    //     self
    // }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nothing to report here yet")
    }
}
