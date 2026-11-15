#[path = "../02-actor/examples/02_actor.rs"]
mod after_actor;
#[path = "../04-async-pipeline/examples/02_async_pipeline.rs"]
mod after_async;
#[path = "../01-channel-pipeline/examples/02_channel_pipeline.rs"]
mod after_channel;
#[path = "../03-work-queue/examples/02_work_queue.rs"]
mod after_queue;
#[path = "../02-actor/examples/01_direct.rs"]
mod before_actor;
#[path = "../04-async-pipeline/examples/01_direct.rs"]
mod before_async;
#[path = "../01-channel-pipeline/examples/01_direct.rs"]
mod before_channel;
#[path = "../03-work-queue/examples/01_direct.rs"]
mod before_queue;
mod support;

use concurrency_async::{
    actor::ActorError, async_pipeline::AsyncPipelineError, channel_pipeline::PipelineError,
    work_queue::QueueError, Job, JobResult,
};
use std::{
    future::{poll_fn, Future},
    pin::Pin,
    task::Poll,
    time::Duration,
};
use tokio::sync::{mpsc, oneshot};

fn jobs() -> Vec<Job> {
    vec![
        Job { id: 8, value: 2 },
        Job { id: 1, value: 3 },
        Job {
            id: 8,
            value: u32::MAX,
        },
    ]
}
fn expected() -> Vec<JobResult> {
    vec![
        JobResult { id: 8, value: 4 },
        JobResult { id: 1, value: 9 },
        JobResult {
            id: 8,
            value: 18_446_744_065_119_617_025,
        },
    ]
}

#[test]
fn channel_callers_share_order_empty_and_validation_contracts() {
    support::bounded(|| {
        for run in [before_channel::run, after_channel::run] {
            for workers in [1, 4] {
                assert_eq!(run(jobs(), workers), Ok(expected()));
            }
            assert_eq!(run(vec![], 2), Ok(vec![]));
            assert_eq!(run(vec![], 0), Err(PipelineError::ZeroWorkers));
            assert_eq!(run(jobs(), 0), Err(PipelineError::ZeroWorkers));
        }
    });
}
#[test]
fn queue_callers_share_order_empty_and_validation_contracts() {
    support::bounded(|| {
        for run in [before_queue::run, after_queue::run] {
            for workers in [1, 4] {
                assert_eq!(run(jobs(), workers), Ok(expected()));
            }
            assert_eq!(run(vec![], 2), Ok(vec![]));
            assert_eq!(run(vec![], 0), Err(QueueError::ZeroWorkers));
            assert_eq!(run(jobs(), 0), Err(QueueError::ZeroWorkers));
        }
    });
}
#[test]
fn actor_callers_share_initial_state_updates_and_overflow_contracts() {
    support::bounded(|| {
        for run in [before_actor::run, after_actor::run] {
            assert_eq!(run(10, &[2, 3]), Ok(15));
            assert_eq!(run(4, &[]), Ok(4));
            assert_eq!(run(u64::MAX, &[1]), Err(ActorError::Overflow));
        }
    });
}

type AsyncRun =
    fn(
        mpsc::Receiver<Job>,
        oneshot::Receiver<()>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<JobResult>, AsyncPipelineError>> + Send>>;

#[tokio::test]
async fn async_callers_share_order_eof_and_cancellation_contracts() {
    let runs: [AsyncRun; 2] = [
        |input, cancel| Box::pin(before_async::run(input, cancel)),
        |input, cancel| Box::pin(after_async::run(input, cancel)),
    ];
    tokio::time::timeout(Duration::from_secs(5), async {
        for run in runs {
            let (sender, input) = mpsc::channel(1);
            let (_cancel, cancelled) = oneshot::channel();
            let producer = async move {
                for job in jobs() {
                    sender.send(job).await.unwrap();
                }
            };
            let (_, result) = tokio::join!(producer, run(input, cancelled));
            assert_eq!(result, Ok(expected()));

            let (sender, input) = mpsc::channel(1);
            let (_cancel, cancelled) = oneshot::channel();
            drop(sender);
            assert_eq!(run(input, cancelled).await, Ok(vec![]));

            let (sender, input) = mpsc::channel(1);
            let (cancel, cancelled) = oneshot::channel();
            let mut pipeline = run(input, cancelled);
            poll_fn(|cx| {
                assert!(pipeline.as_mut().poll(cx).is_pending());
                Poll::Ready(())
            })
            .await;
            cancel.send(()).unwrap();
            assert_eq!(pipeline.await, Err(AsyncPipelineError::Cancelled));
            sender.closed().await;

            let (sender, input) = mpsc::channel(1);
            let (cancel, cancelled) = oneshot::channel();
            sender.send(Job { id: 1, value: 2 }).await.unwrap();
            drop(sender);
            cancel.send(()).unwrap();
            assert_eq!(
                run(input, cancelled).await,
                Err(AsyncPipelineError::Cancelled)
            );

            let (sender, input) = mpsc::channel(1);
            let (cancel, cancelled) = oneshot::channel();
            drop(cancel);
            assert_eq!(
                run(input, cancelled).await,
                Err(AsyncPipelineError::Cancelled)
            );
            sender.closed().await;

            let (sender, input) = mpsc::channel(1);
            let (_cancel, cancelled) = oneshot::channel();
            let mut pipeline = run(input, cancelled);
            poll_fn(|cx| {
                assert!(pipeline.as_mut().poll(cx).is_pending());
                Poll::Ready(())
            })
            .await;
            drop(pipeline);
            sender.closed().await;
        }
    })
    .await
    .unwrap();
}
