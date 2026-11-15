use template_method::{Empty, Lower, Pipeline, Upper};
pub fn run(input: &str, upper: bool) -> Result<String, Empty> {
    if upper {
        Upper.run(input)
    } else {
        Lower.run(input)
    }
}
#[allow(dead_code)]
pub fn main() {}
