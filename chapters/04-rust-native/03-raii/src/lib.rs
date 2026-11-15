//! Scope-bound cleanup using `Drop`.
use std::cell::Cell;

#[derive(Default)]
pub struct Tracker {
    active: Cell<usize>,
}
impl Tracker {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn active(&self) -> usize {
        self.active.get()
    }
    pub fn enter(&self) -> Guard<'_> {
        self.active.set(self.active.get() + 1);
        Guard { tracker: self }
    }
}
pub struct Guard<'a> {
    tracker: &'a Tracker,
}
impl Drop for Guard<'_> {
    fn drop(&mut self) {
        self.tracker.active.set(self.tracker.active.get() - 1);
    }
}
