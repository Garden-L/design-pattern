use concurrency_async::{async_pipeline::AsyncPipelineError, Job, JobResult};
use tokio::sync::{mpsc, oneshot};
pub async fn run(
    input: mpsc::Receiver<Job>,
    cancel: oneshot::Receiver<()>,
) -> Result<Vec<JobResult>, AsyncPipelineError> {
    concurrency_async::async_pipeline::run_async_pipeline(input, cancel).await
}
