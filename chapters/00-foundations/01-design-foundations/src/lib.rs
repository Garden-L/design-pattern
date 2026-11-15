#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmountError {
    Empty,
    InvalidNumber,
}
pub trait FormatAmount {
    fn format(&self, amount: u64) -> String;
}
pub struct Decimal;
pub struct Hex;
impl FormatAmount for Decimal {
    fn format(&self, amount: u64) -> String {
        amount.to_string()
    }
}
impl FormatAmount for Hex {
    fn format(&self, amount: u64) -> String {
        format!("{amount:x}")
    }
}
pub fn parse(input: &str) -> Result<u64, AmountError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(AmountError::Empty);
    }
    input.parse().map_err(|_| AmountError::InvalidNumber)
}
pub fn render<F: FormatAmount>(input: &str, formatter: &F) -> Result<String, AmountError> {
    Ok(formatter.format(parse(input)?))
}
pub fn render_dyn(input: &str, formatter: &dyn FormatAmount) -> Result<String, AmountError> {
    Ok(formatter.format(parse(input)?))
}
