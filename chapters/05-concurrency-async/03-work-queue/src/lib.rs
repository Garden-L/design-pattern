//! A reusable bounded worker pool whose owner closes and joins its workers.
pub use concurrency_core::{Job, JobResult};
use std::{
    error::Error,
    fmt,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc, Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueError {
    ZeroWorkers,
    Closed,
}

impl fmt::Display for QueueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ZeroWorkers => "at least one worker is required",
            Self::Closed => "work queue is closed",
        })
    }
}
impl Error for QueueError {}

type TaggedResult = (usize, JobResult);

pub struct WorkQueue {
    sender: Option<mpsc::SyncSender<(usize, Job)>>,
    sequence: AtomicUsize,
    workers: Vec<JoinHandle<Vec<TaggedResult>>>,
}

impl WorkQueue {
    /// The waiting input buffer has capacity `workers`; no runtime is needed.
    pub fn new(workers: usize) -> Result<Self, QueueError> {
        if workers == 0 {
            return Err(QueueError::ZeroWorkers);
        }
        let (sender, receiver) = mpsc::sync_channel::<(usize, Job)>(workers);
        let receiver = Arc::new(Mutex::new(receiver));
        let mut handles = Vec::new();
        for _ in 0..workers {
            let receiver = Arc::clone(&receiver);
            handles.push(thread::spawn(move || {
                let mut completed = Vec::new();
                loop {
                    let item = receiver.lock().expect("receiver lock poisoned").recv();
                    let Ok((sequence, job)) = item else { break };
                    completed.push((sequence, job.process()));
                }
                completed
            }));
        }
        Ok(Self {
            sender: Some(sender),
            sequence: AtomicUsize::new(0),
            workers: handles,
        })
    }
    /// Blocks for buffer capacity. Successful jobs are drained on shutdown.
    /// Sequential calls preserve order. Overlapping calls use ticket order,
    /// assigned here before sending, which need not be completion/ID order.
    pub fn submit(&self, job: Job) -> Result<(), QueueError> {
        let sender = self.sender.as_ref().ok_or(QueueError::Closed)?;
        let sequence = self.sequence.fetch_add(1, Ordering::Relaxed);
        sender.send((sequence, job)).map_err(|_| QueueError::Closed)
    }
    /// Explicit EOF signal: buffered jobs drain, then blocked receivers wake.
    /// Idempotent; subsequent submissions return `QueueError::Closed`.
    pub fn close(&mut self) {
        self.sender.take();
    }
    /// Close, join every worker, and return all accepted jobs in ticket order.
    /// Panics if an internal worker panics. Jobs contain no user callbacks;
    /// the specified Vec return type cannot report a worker failure.
    pub fn shutdown(mut self) -> Vec<JobResult> {
        let (mut results, panicked) = self.finish();
        assert!(!panicked, "work queue worker panicked");
        results.sort_by_key(|(sequence, _)| *sequence);
        results.into_iter().map(|(_, result)| result).collect()
    }

    fn finish(&mut self) -> (Vec<TaggedResult>, bool) {
        self.close();
        let mut results = Vec::new();
        let mut panicked = false;
        // No result channel can block a worker while the owner is joining it.
        for worker in self.workers.drain(..) {
            match worker.join() {
                Ok(completed) => results.extend(completed),
                Err(_) => panicked = true,
            }
        }
        (results, panicked)
    }
}

impl Drop for WorkQueue {
    fn drop(&mut self) {
        // Also join during unwinding, without adding a second panic.
        let _ = self.finish();
    }
}
