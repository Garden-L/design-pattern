use foundations::design::AmountError;
pub fn render(input: &str, hex: bool) -> Result<String, AmountError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(AmountError::Empty);
    }
    let amount: u64 = input.parse().map_err(|_| AmountError::InvalidNumber)?;
    Ok(if hex {
        format!("{amount:x}")
    } else {
        amount.to_string()
    })
}
