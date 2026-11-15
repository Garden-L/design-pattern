use rust_native::error_design::AppError;

// Each caller maps the parse error explicitly.
pub fn run(input: &str) -> Result<u32, AppError> {
    if input.is_empty() {
        return Err(AppError::MissingCount);
    }
    input.parse().map_err(AppError::Parse)
}
