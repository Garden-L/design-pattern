use template_method::Empty;
pub fn run(input: &str, upper: bool) -> Result<String, Empty> {
    if upper {
        let input = input.trim();
        if input.is_empty() {
            return Err(Empty);
        }
        Ok(input.to_uppercase())
    } else {
        let input = input.trim();
        if input.is_empty() {
            return Err(Empty);
        }
        Ok(input.to_lowercase())
    }
}
#[allow(dead_code)]
pub fn main() {}
