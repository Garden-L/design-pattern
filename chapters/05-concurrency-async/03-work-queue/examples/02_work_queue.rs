use concurrency_core::{Job, JobResult};
use work_queue::QueueError;
pub fn run(input: Vec<Job>, workers: usize) -> Result<Vec<JobResult>, QueueError> {
    let queue = work_queue::WorkQueue::new(workers)?;
    for job in input {
        queue.submit(job)?;
    }
    Ok(queue.shutdown())
}
#[allow(dead_code)]
pub fn main() {}
