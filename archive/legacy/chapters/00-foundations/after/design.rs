use foundations::design::{self, AmountError, Decimal, Hex};
pub fn render(input: &str, hex: bool) -> Result<String, AmountError> {
    let formatter: &dyn design::FormatAmount = if hex { &Hex } else { &Decimal };
    design::render_dyn(input, formatter)
}
