//! A bounded input queue, parallel transforms, and an ordered collector.
pub use concurrency_core::{Job, JobResult};
use std::{
    error::Error,
    fmt,
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineError {
    ZeroWorkers,
    ChannelClosed,
    WorkerPanicked,
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ZeroWorkers => "at least one worker is required",
            Self::ChannelClosed => "pipeline input channel closed unexpectedly",
            Self::WorkerPanicked => "a pipeline thread panicked",
        })
    }
}
impl Error for PipelineError {}

/// Return results in input order, regardless of IDs or completion order.
/// Zero workers is always an error, including with empty input.
pub fn run_pipeline(input: Vec<Job>, workers: usize) -> Result<Vec<JobResult>, PipelineError> {
    if workers == 0 {
        return Err(PipelineError::ZeroWorkers);
    }
    let (jobs, receiver) = mpsc::sync_channel::<(usize, Job)>(workers);
    let receiver = Arc::new(Mutex::new(receiver));
    // Finite input bounds this unbounded output; collecting runs concurrently.
    let (results, output) = mpsc::channel();
    thread::scope(|scope| {
        let producer = scope.spawn(move || {
            for item in input.into_iter().enumerate() {
                jobs.send(item).map_err(|_| PipelineError::ChannelClosed)?;
            }
            // Dropping the only jobs sender signals EOF, including empty input.
            Ok(())
        });
        let mut handles = Vec::new();
        for _ in 0..workers {
            let receiver = Arc::clone(&receiver);
            let results = results.clone();
            handles.push(scope.spawn(move || loop {
                // recv needs exclusive access; computation must not hold the lock.
                let item = receiver.lock().expect("receiver lock poisoned").recv();
                let Ok((sequence, job)) = item else { break };
                if results.send((sequence, job.process())).is_err() {
                    break;
                }
            }));
        }
        drop(receiver);
        drop(results); // Collector sees EOF after all workers release their clones.
        let mut collected: Vec<_> = output.into_iter().collect();
        let produced = producer.join();
        let mut panicked = produced.is_err();
        for handle in handles {
            panicked |= handle.join().is_err();
        }
        if panicked {
            return Err(PipelineError::WorkerPanicked);
        }
        produced.expect("producer joined")?;
        collected.sort_by_key(|(sequence, _)| *sequence);
        Ok(collected.into_iter().map(|(_, result)| result).collect())
    })
}
