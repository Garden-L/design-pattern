use std::{cell::RefCell, rc::Rc};
use structural::proxy::{DocumentStore, MemoryStore, StoreError};

pub fn fetch_twice(key: &str) -> (Result<String, StoreError>, usize) {
    let calls = Rc::new(RefCell::new(0));
    let store = MemoryStore::new(calls.clone());
    let first = store.fetch(key);
    let _second = store.fetch(key);
    let count = *calls.borrow();
    (first, count)
}
