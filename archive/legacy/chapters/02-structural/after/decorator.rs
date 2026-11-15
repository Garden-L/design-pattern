use std::{cell::RefCell, rc::Rc};
use structural::decorator::{Logger, Service, ServiceError, Uppercase};

pub fn execute(input: &str) -> (Result<String, ServiceError>, Vec<String>) {
    let events = Rc::new(RefCell::new(Vec::new()));
    let result = Logger::new(Uppercase, events.clone()).execute(input);
    let entries = events.borrow().clone();
    (result, entries)
}
