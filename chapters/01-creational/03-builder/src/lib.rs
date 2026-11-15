#[derive(Debug, PartialEq, Eq)]
pub enum BuildError {
    MissingTitle,
    MissingBody,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Report {
    title: String,
    body: String,
}

impl Report {
    pub fn render(&self) -> String {
        format!("{}\n{}", self.title, self.body)
    }
}

#[derive(Default)]
pub struct ReportBuilder {
    title: Option<String>,
    body: Option<String>,
}

impl ReportBuilder {
    pub fn new() -> ReportBuilder {
        Self::default()
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
    pub fn build(self) -> Result<Report, BuildError> {
        let title = self
            .title
            .filter(|s| !s.trim().is_empty())
            .ok_or(BuildError::MissingTitle)?;
        let body = self
            .body
            .filter(|s| !s.trim().is_empty())
            .ok_or(BuildError::MissingBody)?;
        Ok(Report { title, body })
    }
}
