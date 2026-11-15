use behavioral::interpreter::{parse, EvalError};
pub fn evaluate(input: &str) -> Result<i64, EvalError> {
    parse(input)?.evaluate()
}
