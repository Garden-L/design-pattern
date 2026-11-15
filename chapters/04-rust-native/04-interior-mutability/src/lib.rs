//! Local mutation behind a shared reference.
use std::cell::RefCell;

#[derive(Default)]
pub struct AuditLog {
    entries: RefCell<Vec<String>>,
}
impl AuditLog {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record(&self, entry: &str) {
        self.entries.borrow_mut().push(entry.into());
    }
    pub fn entries(&self) -> Vec<String> {
        self.entries.borrow().clone()
    }
}
