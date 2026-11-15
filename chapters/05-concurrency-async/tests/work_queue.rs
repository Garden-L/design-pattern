// Coverage boundary: no worker-panic injection or forced completion reordering.
// Normal shutdown joins are tested; exceptional joins remain unverified.
mod support;
use concurrency_async::{
    work_queue::{QueueError, WorkQueue},
    Job, JobResult,
};
use std::sync::Arc;

#[test]
fn zero_workers_is_rejected() {
    assert!(matches!(WorkQueue::new(0), Err(QueueError::ZeroWorkers)));
}

#[test]
fn close_rejects_new_jobs_and_shutdown_drains_in_submission_order() {
    support::bounded(|| {
        let mut queue = WorkQueue::new(3).unwrap();
        for job in [
            Job { id: 5, value: 4 },
            Job { id: 1, value: 3 },
            Job { id: 5, value: 0 },
        ] {
            queue.submit(job).unwrap();
        }
        queue.close();
        queue.close();
        assert_eq!(
            queue.submit(Job { id: 99, value: 2 }),
            Err(QueueError::Closed)
        );
        assert_eq!(
            queue.shutdown(),
            vec![
                JobResult { id: 5, value: 16 },
                JobResult { id: 1, value: 9 },
                JobResult { id: 5, value: 0 }
            ]
        );
    });
}

#[test]
fn concurrent_producers_have_no_lost_or_duplicate_results() {
    support::bounded(|| {
        let queue = Arc::new(WorkQueue::new(2).unwrap());
        std::thread::scope(|scope| {
            for producer in 0..4 {
                let queue = Arc::clone(&queue);
                scope.spawn(move || {
                    for index in 0..25 {
                        queue
                            .submit(Job {
                                id: producer * 25 + index,
                                value: 3,
                            })
                            .unwrap();
                    }
                });
            }
        });
        let mut output = Arc::try_unwrap(queue).ok().unwrap().shutdown();
        output.sort_by_key(|result| result.id);
        assert_eq!(output.len(), 100);
        for (id, result) in output.iter().enumerate() {
            assert_eq!(
                *result,
                JobResult {
                    id: id as u64,
                    value: 9
                }
            );
        }
    });
}

#[test]
fn empty_and_drop_shutdown_terminate() {
    support::bounded(|| {
        assert!(WorkQueue::new(4).unwrap().shutdown().is_empty());
        let queue = WorkQueue::new(1).unwrap();
        for id in 0..100 {
            queue.submit(Job { id, value: 2 }).unwrap();
        }
        drop(queue);
    });
}
