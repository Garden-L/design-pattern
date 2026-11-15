use channel_pipeline::PipelineError;
use concurrency_core::{Job, JobResult};
pub fn run(input: Vec<Job>, workers: usize) -> Result<Vec<JobResult>, PipelineError> {
    channel_pipeline::run_pipeline(input, workers)
}
#[allow(dead_code)]
pub fn main() {}
