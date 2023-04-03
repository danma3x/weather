/// Assembled freeform weather report data that implements some rudimentary formatting
pub struct Report {
    title: String,
    sections: Vec<ReportSection>,
}

/// Basic type for representing report fields
pub type SectionRepr = Vec<(String, String)>;

/// Part of the full report with its own title
pub struct ReportSection {
    title: String,
    contents: SectionRepr,
}

impl ReportSection {
    pub fn new(title: String, contents: SectionRepr) -> Self {
        Self { title, contents }
    }
}

impl Report {
    /// Initialize a report, add fields to it via the builder pattern in your favorite weather provider adapter
    pub fn new<S: Into<String>>(title: S) -> Self {
        Self {
            title: title.into(),
            sections: Vec::new(),
        }
    }

    /// Adds a new section to a report
    pub fn add_section(&mut self, section: ReportSection) {
        self.sections.push(section);
    }
}

impl std::fmt::Display for Report {
    //// Formatting of a report as it is intended to be shown to a user
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "{}", self.title);
        self.sections.iter().fold(Ok(()), |result, section| {
            result.and_then(|_| writeln!(f, "{}", section))
        })
    }
}

impl std::fmt::Display for ReportSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "{}", &self.title);
        self.contents.iter().fold(Ok(()), |result, (k, v)| {
            result.and_then(|_| writeln!(f, "{:<35} | {:<20}", k, v))
        })
    }
}
