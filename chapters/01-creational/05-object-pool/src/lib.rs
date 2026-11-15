use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};
/// A single-threaded fixed-size pool. Empty buffers retain capacity when returned.
pub struct BufferPool {
    free: RefCell<Vec<Vec<u8>>>,
}
impl BufferPool {
    pub fn new(capacity: usize) -> Self {
        Self {
            free: RefCell::new(vec![Vec::new(); capacity]),
        }
    }
    pub fn acquire(&self) -> Option<Lease<'_>> {
        self.free
            .borrow_mut()
            .pop()
            .map(|buffer| Lease { pool: self, buffer })
    }
}
/// The borrow prevents the pool from being dropped before outstanding leases.
pub struct Lease<'a> {
    pool: &'a BufferPool,
    buffer: Vec<u8>,
}
impl Deref for Lease<'_> {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
impl DerefMut for Lease<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}
impl Drop for Lease<'_> {
    fn drop(&mut self) {
        self.buffer.clear();
        self.pool
            .free
            .borrow_mut()
            .push(std::mem::take(&mut self.buffer));
    }
}
