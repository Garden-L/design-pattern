use interpreter::EvalError;
pub fn evaluate(input: &str) -> Result<i64, EvalError> {
    let mut stack: Vec<i64> = Vec::new();
    for token in input.split_whitespace() {
        if token == "+" {
            let b = stack.pop().ok_or(EvalError::Syntax)?;
            let a = stack.pop().ok_or(EvalError::Syntax)?;
            stack.push(a.checked_add(b).ok_or(EvalError::Overflow)?);
        } else {
            stack.push(token.parse().map_err(|_| EvalError::Syntax)?);
        }
    }
    if stack.len() != 1 {
        return Err(EvalError::Syntax);
    }
    stack.pop().ok_or(EvalError::Syntax)
}
#[allow(dead_code)]
pub fn main() {}
