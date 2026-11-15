use typestate::{BuildError, Request};

// An incomplete request exists at runtime, so validation is required.
pub fn run(method: Option<&str>, url: Option<&str>) -> Result<Request, BuildError> {
    let method = method.ok_or(BuildError::MissingMethod)?;
    let url = url.ok_or(BuildError::MissingUrl)?;
    Ok(Request {
        method: method.into(),
        url: url.into(),
    })
}
#[allow(dead_code)]
pub fn main() {}
