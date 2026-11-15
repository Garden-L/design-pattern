use async_pipeline::AsyncPipelineError;
use concurrency_core::{Job, JobResult};
use tokio::sync::{mpsc, oneshot};
pub async fn run(
    input: mpsc::Receiver<Job>,
    cancel: oneshot::Receiver<()>,
) -> Result<Vec<JobResult>, AsyncPipelineError> {
    async_pipeline::run_async_pipeline(input, cancel).await
}
#[allow(dead_code)]
pub fn main() {}
