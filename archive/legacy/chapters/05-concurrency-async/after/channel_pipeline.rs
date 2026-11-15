use concurrency_async::{channel_pipeline::PipelineError, Job, JobResult};
pub fn run(input: Vec<Job>, workers: usize) -> Result<Vec<JobResult>, PipelineError> {
    concurrency_async::channel_pipeline::run_pipeline(input, workers)
}
