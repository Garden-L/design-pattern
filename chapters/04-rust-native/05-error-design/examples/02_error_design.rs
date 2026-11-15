use error_design::{parse_count, AppError};

pub fn run(input: &str) -> Result<u32, AppError> {
    parse_count(input)
}
#[allow(dead_code)]
pub fn main() {}
