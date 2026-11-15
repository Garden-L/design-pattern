use async_pipeline::AsyncPipelineError;
use concurrency_core::{Job, JobResult};
use std::{
    future::{poll_fn, Future},
    pin::Pin,
    task::Poll,
};
use tokio::sync::{mpsc, oneshot};

// Handwritten Future polling: the caller must preserve both wakeup paths,
// cancellation priority, state across polls, and the EOF rule.
pub async fn run(
    mut input: mpsc::Receiver<Job>,
    mut cancel: oneshot::Receiver<()>,
) -> Result<Vec<JobResult>, AsyncPipelineError> {
    let mut results = Vec::new();
    poll_fn(move |cx| loop {
        if Pin::new(&mut cancel).poll(cx).is_ready() {
            return Poll::Ready(Err(AsyncPipelineError::Cancelled));
        }
        match input.poll_recv(cx) {
            Poll::Ready(Some(job)) => results.push(JobResult {
                id: job.id,
                value: u64::from(job.value) * u64::from(job.value),
            }),
            Poll::Ready(None) => return Poll::Ready(Ok(std::mem::take(&mut results))),
            Poll::Pending => return Poll::Pending,
        }
    })
    .await
}
#[allow(dead_code)]
pub fn main() {}
