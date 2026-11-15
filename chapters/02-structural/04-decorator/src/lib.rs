use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceError {
    EmptyInput,
}

pub trait Service {
    fn execute(&self, input: &str) -> Result<String, ServiceError>;
}

pub struct Uppercase;
impl Service for Uppercase {
    fn execute(&self, input: &str) -> Result<String, ServiceError> {
        if input.is_empty() {
            Err(ServiceError::EmptyInput)
        } else {
            Ok(input.to_uppercase())
        }
    }
}

/// A decorator that records entry and outcome around the wrapped service.
pub struct Logger<T: Service> {
    inner: T,
    events: Rc<RefCell<Vec<String>>>,
}
impl<T: Service> Logger<T> {
    pub fn new(inner: T, events: Rc<RefCell<Vec<String>>>) -> Self {
        Self { inner, events }
    }
}
impl<T: Service> Service for Logger<T> {
    fn execute(&self, input: &str) -> Result<String, ServiceError> {
        self.events.borrow_mut().push(format!("before:{input}"));
        let result = self.inner.execute(input);
        self.events.borrow_mut().push(
            if result.is_ok() {
                "after:ok"
            } else {
                "after:error"
            }
            .into(),
        );
        result
    }
}
