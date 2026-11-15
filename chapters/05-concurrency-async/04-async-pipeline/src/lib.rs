//! One async receive/transform/collect stage, with no spawned tasks.
pub use concurrency_core::{Job, JobResult};
use std::{error::Error, fmt};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncPipelineError {
    Cancelled,
}

impl fmt::Display for AsyncPipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("async pipeline cancelled")
    }
}
impl Error for AsyncPipelineError {}

/// Receive until the last input sender drops, preserving receive order.
/// Sending cancellation OR dropping its sender returns Cancelled and discards
/// partial results. Ready cancellation wins over ready input/EOF. Dropping this
/// future drops its receiver; no detached tasks or external side effects exist.
pub async fn run_async_pipeline(
    mut input: mpsc::Receiver<Job>,
    mut cancel: oneshot::Receiver<()>,
) -> Result<Vec<JobResult>, AsyncPipelineError> {
    let mut results = Vec::new();
    loop {
        tokio::select! {
            biased;
            _ = &mut cancel => return Err(AsyncPipelineError::Cancelled),
            job = input.recv() => match job {
                Some(job) => results.push(job.process()),
                None => return Ok(results),
            }
        }
    }
}
