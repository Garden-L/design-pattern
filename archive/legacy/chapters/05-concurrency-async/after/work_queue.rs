use concurrency_async::{work_queue::QueueError, Job, JobResult};
pub fn run(input: Vec<Job>, workers: usize) -> Result<Vec<JobResult>, QueueError> {
    let queue = concurrency_async::work_queue::WorkQueue::new(workers)?;
    for job in input {
        queue.submit(job)?;
    }
    Ok(queue.shutdown())
}
