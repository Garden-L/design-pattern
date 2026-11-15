use concurrency_async::{work_queue::QueueError, Job, JobResult};
// A finite batch works; it has no reusable submit/close lifecycle.
pub fn run(input: Vec<Job>, workers: usize) -> Result<Vec<JobResult>, QueueError> {
    if workers == 0 {
        return Err(QueueError::ZeroWorkers);
    }
    let mut results = Vec::new();
    for job in input {
        results.push(JobResult {
            id: job.id,
            value: u64::from(job.value) * u64::from(job.value),
        });
    }
    Ok(results)
}
