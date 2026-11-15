use decorator::{Logger, Service, ServiceError, Uppercase};
use std::{cell::RefCell, rc::Rc};

pub fn execute(input: &str) -> (Result<String, ServiceError>, Vec<String>) {
    let events = Rc::new(RefCell::new(Vec::new()));
    let result = Logger::new(Uppercase, events.clone()).execute(input);
    let entries = events.borrow().clone();
    (result, entries)
}
#[allow(dead_code)]
pub fn main() {}
