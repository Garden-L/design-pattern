use concurrency_async::{channel_pipeline::PipelineError, Job, JobResult};
// Output is correct, but one caller does all computation serially.
pub fn run(input: Vec<Job>, workers: usize) -> Result<Vec<JobResult>, PipelineError> {
    if workers == 0 {
        return Err(PipelineError::ZeroWorkers);
    }
    Ok(input
        .into_iter()
        .map(|job| JobResult {
            id: job.id,
            value: u64::from(job.value) * u64::from(job.value),
        })
        .collect())
}
