//! An in-process async request server backed by the chapter's blocking work queue.
use concurrency_async::{
    async_pipeline::AsyncPipelineError,
    work_queue::{QueueError, WorkQueue},
};
pub use concurrency_async::{Job, JobResult};
use rust_native::enum_state_machine::{ConnectionError, ConnectionState};
use std::{error::Error, fmt};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub enum ServerError {
    InvalidCapacity,
    Queue(QueueError),
    State(ConnectionError),
    Cancelled(AsyncPipelineError),
    Worker(tokio::task::JoinError),
}
impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCapacity => f.write_str("bridge capacity must be positive"),
            Self::Queue(error) => write!(f, "server queue failed: {error}"),
            Self::State(error) => write!(f, "invalid server state: {error:?}"),
            Self::Cancelled(error) => write!(f, "server stopped: {error}"),
            Self::Worker(error) => write!(f, "server worker failed: {error}"),
        }
    }
}
impl Error for ServerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Queue(error) => Some(error),
            Self::Cancelled(error) => Some(error),
            Self::Worker(error) => Some(error),
            _ => None,
        }
    }
}
pub struct ServerBuilder {
    workers: usize,
    capacity: usize,
}
impl Default for ServerBuilder {
    fn default() -> Self {
        Self {
            workers: 1,
            capacity: 1,
        }
    }
}
impl ServerBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn workers(mut self, workers: usize) -> Self {
        self.workers = workers;
        self
    }
    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }
    /// Validate configuration without starting threads or requiring a runtime.
    pub fn build(self) -> Result<Server, ServerError> {
        if self.workers == 0 {
            return Err(ServerError::Queue(QueueError::ZeroWorkers));
        }
        if self.capacity == 0 {
            return Err(ServerError::InvalidCapacity);
        }
        Ok(Server {
            workers: self.workers,
            capacity: self.capacity,
            state: ConnectionState::Disconnected,
        })
    }
}
pub struct Server {
    workers: usize,
    capacity: usize,
    state: ConnectionState,
}

// Reset even if the service future is dropped while waiting. Drop never joins
// threads on the executor: only a completed serve guarantees joined workers.
struct ResetState<'a>(&'a mut ConnectionState);
impl Drop for ResetState<'_> {
    fn drop(&mut self) {
        *self.0 = ConnectionState::Disconnected;
    }
}

impl Server {
    pub fn state(&self) -> ConnectionState {
        self.state
    }
    pub fn start(&mut self) -> Result<(), ServerError> {
        self.state = self.state.connect().map_err(ServerError::State)?;
        Ok(())
    }
    /// Requires start and a Tokio runtime. EOF returns ordered responses.
    /// Sending or dropping cancel closes input, drains/joins accepted queue jobs,
    /// and returns Cancelled without partial responses. Ready cancellation wins.
    /// Keep the cancellation sender alive during normal completion.
    pub async fn serve(
        &mut self,
        mut input: mpsc::Receiver<Job>,
        mut cancel: oneshot::Receiver<()>,
    ) -> Result<Vec<JobResult>, ServerError> {
        self.state.send().map_err(ServerError::State)?;
        let _state = ResetState(&mut self.state);
        let workers = self.workers;
        let (forward, mut requests) = mpsc::channel(self.capacity);
        let worker = tokio::task::spawn_blocking(move || -> Result<Vec<JobResult>, ServerError> {
            let queue = WorkQueue::new(workers).map_err(ServerError::Queue)?;
            while let Some(job) = requests.blocking_recv() {
                queue.submit(job).map_err(ServerError::Queue)?;
            }
            Ok(queue.shutdown())
        });
        let outcome = loop {
            let job = tokio::select! {
                biased;
                _ = &mut cancel => break Err(ServerError::Cancelled(AsyncPipelineError::Cancelled)),
                job = input.recv() => match job {
                    Some(job) => job,
                    None => break Ok(()),
                }
            };
            // Cancellation must also interrupt waiting for bridge capacity.
            tokio::select! {
                biased;
                _ = &mut cancel => break Err(ServerError::Cancelled(AsyncPipelineError::Cancelled)),
                sent = forward.send(job) => if sent.is_err() {
                    break Err(ServerError::Queue(QueueError::Closed));
                }
            }
        };
        drop(input); // Release external producers before waiting on cleanup.
        drop(forward); // EOF drains the blocking bridge and its queue.
        let results = worker.await.map_err(ServerError::Worker)??;
        outcome?;
        Ok(results)
    }
}
