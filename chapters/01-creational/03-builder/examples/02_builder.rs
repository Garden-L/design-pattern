use builder::{BuildError, ReportBuilder};

pub fn report(title: &str, body: &str) -> Result<String, BuildError> {
    ReportBuilder::new()
        .title(title)
        .body(body)
        .build()
        .map(|report| report.render())
}
#[allow(dead_code)]
pub fn main() {}
