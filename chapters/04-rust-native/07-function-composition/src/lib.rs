#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputError {
    Invalid,
    NonPositive,
}
pub fn normalize(input: &str) -> Result<u64, InputError> {
    input.trim().parse().map_err(|_| InputError::Invalid)
}
pub fn positive(number: u64) -> Result<u64, InputError> {
    if number == 0 {
        Err(InputError::NonPositive)
    } else {
        Ok(number)
    }
}
pub fn compose<A, B, C, E>(
    first: impl Fn(A) -> Result<B, E>,
    second: impl Fn(B) -> Result<C, E>,
) -> impl Fn(A) -> Result<C, E> {
    move |input| first(input).and_then(&second)
}
