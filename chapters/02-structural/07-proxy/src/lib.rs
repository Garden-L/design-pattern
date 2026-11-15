use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreError {
    NotFound,
}
pub trait DocumentStore {
    fn fetch(&self, key: &str) -> Result<String, StoreError>;
}
pub struct MemoryStore {
    calls: Rc<RefCell<usize>>,
}
impl MemoryStore {
    pub fn new(calls: Rc<RefCell<usize>>) -> Self {
        Self { calls }
    }
}
impl DocumentStore for MemoryStore {
    fn fetch(&self, key: &str) -> Result<String, StoreError> {
        *self.calls.borrow_mut() += 1;
        match key {
            "guide" => Ok("Rust guide".into()),
            _ => Err(StoreError::NotFound),
        }
    }
}
pub struct CachedProxy<T: DocumentStore> {
    inner: T,
    cache: RefCell<HashMap<String, String>>,
}
impl<T: DocumentStore> CachedProxy<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            cache: RefCell::new(HashMap::new()),
        }
    }
}
impl<T: DocumentStore> DocumentStore for CachedProxy<T> {
    fn fetch(&self, key: &str) -> Result<String, StoreError> {
        if let Some(value) = self.cache.borrow().get(key) {
            return Ok(value.clone());
        }
        let value = self.inner.fetch(key)?;
        self.cache.borrow_mut().insert(key.into(), value.clone());
        Ok(value)
    }
}
