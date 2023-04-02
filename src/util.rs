use crate::{
    args,
    command::{DateOffsetRepresentation, WeatherCommand},
};
use anyhow::Result;

/// Parses a custom argument format
// TODO: way too nested for anyone's taste
// TODO: lowercase too pls
pub fn parse_date_arg(date_str: &str) -> Result<DateOffsetRepresentation> {
    let lower_date_str = date_str.to_ascii_lowercase();
    // then we have a sign, possible a number and a suffix
    if lower_date_str.len() > 2 {
        let amount_res = lower_date_str[1..lower_date_str.len() - 1].parse::<isize>();
        let amount_res = match lower_date_str.chars().next() {
            Some('h') => amount_res.map(|a| -a),
            // Some('f') => amount_res.map(|a| a),
            _ => return Ok(DateOffsetRepresentation::Now),
        };
        if let Ok(amount) = amount_res {
            if amount == 0 {
                return Ok(DateOffsetRepresentation::Now);
            }
            let last_char = lower_date_str.chars().last();
            return match last_char {
                Some('h') => Ok(DateOffsetRepresentation::HourOffset(amount)),
                Some('d') => Ok(DateOffsetRepresentation::DayOffset(amount)),
                _ => Ok(DateOffsetRepresentation::Now),
            };
        }
    }
    Ok(DateOffsetRepresentation::Now)
}

/// Transforming of cmdline arguments to a relevant query helper structure
pub fn parse_get_action(ga: args::GetArgs) -> WeatherCommand {
    let date = if let Some(date) = ga.date {
        parse_date_arg(&date).unwrap_or_default()
    } else {
        DateOffsetRepresentation::Now
    };
    WeatherCommand::new(ga.location, date)
}

#[cfg(test)]
mod tests {
    use super::parse_date_arg;
    use crate::command::DateOffsetRepresentation;

    #[test]
    fn test_parse_date_arg() {
        assert_eq!(
            parse_date_arg("h8H").expect("Bad luck"),
            DateOffsetRepresentation::HourOffset(-8)
        );
        assert_eq!(
            parse_date_arg("h7D").expect("Bad luck"),
            DateOffsetRepresentation::DayOffset(-7)
        );
        assert_eq!(
            parse_date_arg("hNon8D").expect("Bad luck"),
            DateOffsetRepresentation::Now
        );
        assert_eq!(
            parse_date_arg("h0d").expect("Bad luck"),
            DateOffsetRepresentation::Now
        );
    }
}
