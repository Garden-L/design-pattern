use typestate::{BuildError, Request, RequestBuilder};

// Dynamic input still needs boundary validation; after that, build is infallible.
pub fn run(method: Option<&str>, url: Option<&str>) -> Result<Request, BuildError> {
    let method = method.ok_or(BuildError::MissingMethod)?;
    let url = url.ok_or(BuildError::MissingUrl)?;
    Ok(RequestBuilder::new().method(method).url(url).build())
}
#[allow(dead_code)]
pub fn main() {}
