use rust_native::function_composition::InputError;
pub fn parse(input: &str) -> Result<u64, InputError> {
    let value = input.trim().parse().map_err(|_| InputError::Invalid)?;
    if value == 0 {
        Err(InputError::NonPositive)
    } else {
        Ok(value)
    }
}
