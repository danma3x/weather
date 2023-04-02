use super::api;
use crate::report::{Report, ReportSection, SectionRepr};

pub fn report_current(current: api::current::Json) -> Report {
    let mut r = Report::new("WeatherAPI - current");
    let mut details = SectionRepr::new();
    details.push(("Condition".to_owned(), current.current.condition.text));
    details.push((
        "Temperature".to_owned(),
        format!("{}°", current.current.temp_c),
    ));

    details.push((
        "Humidity".to_owned(),
        format!("{}%", current.current.humidity),
    ));
    details.push(("Wind direction".to_owned(), current.current.wind_dir));
    details.push((
        "Wind speed km/h".to_owned(),
        format!("{}", current.current.wind_kph),
    ));
    details.push((
        "Wind speed ml/h".to_owned(),
        format!("{}", current.current.wind_mph),
    ));
    let rs = ReportSection::new("Status".to_owned(), details);
    r.add_section(rs);
    r
}

pub fn report_forecast(forecast: api::forecast::Json) -> Report {
    let mut r = Report::new("WeatherAPI - forecast");
    forecast.forecast.forecastday.into_iter().for_each(|day| {
        let mut day_report = SectionRepr::new();
        day_report.push(("Condition".to_owned(), day.day.condition.text));
        day_report.push((
            "Minimum temp., C".to_owned(),
            format!("{}°", day.day.mintemp_c),
        ));
        day_report.push((
            "Maximum temp., C".to_owned(),
            format!("{}°", day.day.maxtemp_c),
        ));
        day_report.push((
            "Minimum temp., F".to_owned(),
            format!("{}°", day.day.mintemp_f),
        ));
        day_report.push((
            "Maximum temp., F".to_owned(),
            format!("{}°", day.day.maxtemp_f),
        ));
        let rs = ReportSection::new(day.date_epoch.format("%d/%m/%Y").to_string(), day_report);
        r.add_section(rs);
    });
    r
}

pub fn report_history(history: api::history::Json) -> Report {
    let mut r = Report::new("WeatherAPI - history");
    history.forecast.forecastday.into_iter().for_each(|day| {
        let mut day_report = SectionRepr::new();
        day_report.push(("Condition".to_owned(), day.day.condition.text));
        day_report.push((
            "Minimum temp., C".to_owned(),
            format!("{}°", day.day.mintemp_c),
        ));
        day_report.push((
            "Maximum temp., C".to_owned(),
            format!("{}°", day.day.maxtemp_c),
        ));
        day_report.push((
            "Minimum temp., F".to_owned(),
            format!("{}°", day.day.mintemp_f),
        ));
        day_report.push((
            "Maximum temp., F".to_owned(),
            format!("{}°", day.day.maxtemp_f),
        ));
        let rs = ReportSection::new(day.date_epoch.format("%d/%m/%Y").to_string(), day_report);
        r.add_section(rs);
    });
    r
}
