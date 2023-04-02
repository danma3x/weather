use chrono::{DateTime, Duration, FixedOffset};

/// A simple abstraction for types of dates a user might want to enter
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum DateOffsetRepresentation {
    Now,
    HourOffset(isize),
    DayOffset(isize),
    Date(chrono::NaiveDate),
}

impl Default for DateOffsetRepresentation {
    fn default() -> Self {
        Self::Now
    }
}

impl DateOffsetRepresentation {
    pub fn to_chrono(&self, origin: DateTime<FixedOffset>) -> DateTime<FixedOffset> {
        match self {
            DateOffsetRepresentation::DayOffset(days) => origin + Duration::days(*days as i64),
            DateOffsetRepresentation::HourOffset(hours) => origin + Duration::hours(*hours as i64),
            _ => origin,
        }
    }
}

/// Bundle of data that is to be used
#[derive(Debug)]
pub struct WeatherCommand {
    pub location: String,
    pub date: DateOffsetRepresentation,
}

impl WeatherCommand {
    pub fn new<S: Into<String>>(location: S, date: DateOffsetRepresentation) -> Self {
        Self {
            location: location.into(),
            date,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_chrono() {
        let d = DateTime::<FixedOffset>::parse_from_rfc3339("2019-10-12T07:20:50.52Z")
            .expect("Couldn't parse the datetime");
        let min_5d = DateOffsetRepresentation::DayOffset(-5).to_chrono(d);
        assert_eq!(min_5d.to_rfc3339(), "2019-10-07T07:20:50.520+00:00");

        let plus_8h = DateOffsetRepresentation::HourOffset(8).to_chrono(d);
        assert_eq!(plus_8h.to_rfc3339(), "2019-10-12T15:20:50.520+00:00")
    }
}
