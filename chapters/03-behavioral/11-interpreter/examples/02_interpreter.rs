use interpreter::{parse, EvalError};
pub fn evaluate(input: &str) -> Result<i64, EvalError> {
    parse(input)?.evaluate()
}
#[allow(dead_code)]
pub fn main() {}
