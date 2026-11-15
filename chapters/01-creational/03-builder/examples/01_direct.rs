#[derive(Debug, PartialEq, Eq)]
pub enum BuildError {
    MissingTitle,
    MissingBody,
}

pub fn report(title: &str, body: &str) -> Result<String, BuildError> {
    if title.trim().is_empty() {
        return Err(BuildError::MissingTitle);
    }
    if body.trim().is_empty() {
        return Err(BuildError::MissingBody);
    }
    Ok(format!("{title}\n{body}"))
}
#[allow(dead_code)]
pub fn main() {}
