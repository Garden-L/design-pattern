//! Shared job types used by the concurrency and async pattern crates.

/// IDs are labels, not ordering keys. Duplicate IDs are allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Job {
    pub id: u64,
    pub value: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JobResult {
    pub id: u64,
    pub value: u64,
}

impl Job {
    /// Widen before multiplying: every u32 square fits in u64.
    pub fn process(self) -> JobResult {
        JobResult {
            id: self.id,
            value: u64::from(self.value) * u64::from(self.value),
        }
    }
}
