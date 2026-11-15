use rust_native::error_design::{parse_count, AppError};

pub fn run(input: &str) -> Result<u32, AppError> {
    parse_count(input)
}
