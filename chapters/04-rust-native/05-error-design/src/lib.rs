//! Domain errors retain their underlying cause.
use std::{error::Error, fmt, io, num::ParseIntError};

#[derive(Debug)]
pub enum AppError {
    MissingCount,
    Parse(ParseIntError),
    Io(io::Error),
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCount => write!(f, "missing count"),
            Self::Parse(_) => write!(f, "invalid count"),
            Self::Io(_) => write!(f, "count I/O failed"),
        }
    }
}
impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parse(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::MissingCount => None,
        }
    }
}
impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        Self::Parse(error)
    }
}
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
pub fn parse_count(input: &str) -> Result<u32, AppError> {
    if input.is_empty() {
        return Err(AppError::MissingCount);
    }
    Ok(input.parse()?)
}
