#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalError {
    Syntax,
    Overflow,
}
#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
}
impl Expr {
    pub fn evaluate(&self) -> Result<i64, EvalError> {
        match self {
            Self::Number(n) => Ok(*n),
            Self::Add(a, b) => a
                .evaluate()?
                .checked_add(b.evaluate()?)
                .ok_or(EvalError::Overflow),
        }
    }
}
/// A small postfix language: signed integer literals and binary addition.
pub fn parse(input: &str) -> Result<Expr, EvalError> {
    let mut stack = Vec::new();
    for token in input.split_whitespace() {
        if token == "+" {
            let b = stack.pop().ok_or(EvalError::Syntax)?;
            let a = stack.pop().ok_or(EvalError::Syntax)?;
            stack.push(Expr::Add(Box::new(a), Box::new(b)));
        } else {
            stack.push(Expr::Number(token.parse().map_err(|_| EvalError::Syntax)?));
        }
    }
    if stack.len() != 1 {
        return Err(EvalError::Syntax);
    }
    stack.pop().ok_or(EvalError::Syntax)
}
