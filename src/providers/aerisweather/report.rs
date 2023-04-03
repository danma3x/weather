use super::api;
use crate::report::{Report, ReportSection, SectionRepr};

pub fn report_hourly(report: &mut Report, hourly: api::hourly::Json) {
    hourly.response.into_iter().for_each(|r| {
        r.periods.into_iter().for_each(|p| {
            let mut details = SectionRepr::new();
            details.push(("Condition".to_owned(), p.weather));
            details.push(("Wind direction".to_owned(), p.wind_dir));
            details.push(("Temperature, C".to_owned(), format!("{}°", p.temp_c)));
            details.push(("Temperature, F".to_owned(), format!("{}°", p.temp_f)));
            details.push(("Humidity".to_owned(), format!("{}%", p.humidity)));
            let rs = ReportSection::new(
                p.timestamp.format("%d/%m/%Y %I:%M %p (UTC)").to_string(),
                details,
            );
            report.add_section(rs);
        })
    })
}

pub fn report_daily(report: &mut Report, daily: api::daily::Json) {
    daily.response.into_iter().for_each(|r| {
        r.periods.into_iter().for_each(|p| {
            let mut details = SectionRepr::new();
            details.push(("Condition".to_owned(), p.weather.phrase));
            details.push(("Average temp., C".to_owned(), format!("{}°", p.temp.avg_c)));
            details.push(("Average temp., F".to_owned(), format!("{}°", p.temp.avg_f)));
            details.push(("Min. temp., C".to_owned(), format!("{}°", p.temp.min_c)));
            details.push(("Max. temp., C".to_owned(), format!("{}°", p.temp.max_c)));
            details.push(("Min. temp., F".to_owned(), format!("{}°", p.temp.min_f)));
            details.push(("Max. temp., F".to_owned(), format!("{}°", p.temp.max_f)));
            details.push((
                "Average humidity".to_owned(),
                format!("{}%", p.humidity.avg),
            ));
            details.push(("Wind direction".to_owned(), p.wind_speed.max_dir));
            let rs = ReportSection::new(
                p.timestamp.format("%d/%m/%Y %I:%M %p (UTC)").to_string(),
                details,
            );
            report.add_section(rs);
        })
    })
}
