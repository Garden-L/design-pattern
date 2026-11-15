use rust_native::function_composition::{compose, normalize, positive, InputError};
pub fn parse(input: &str) -> Result<u64, InputError> {
    compose(normalize, positive)(input)
}
