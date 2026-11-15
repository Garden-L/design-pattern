// TODO: add a second decorator and verify wrapping order.
pub trait Service { fn call(&self) -> String; }
