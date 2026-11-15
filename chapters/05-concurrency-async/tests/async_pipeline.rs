use concurrency_async::{
    async_pipeline::{run_async_pipeline, AsyncPipelineError},
    Job, JobResult,
};
use std::{
    future::{poll_fn, Future},
    pin::Pin,
    task::Poll,
    time::Duration,
};
use tokio::sync::{mpsc, oneshot};
use tokio::time::timeout;

const DEADLINE: Duration = Duration::from_secs(5);

// Prove the real future reached Pending before sending cancellation/dropping it.
async fn assert_pending<F: Future>(mut future: Pin<&mut F>) {
    poll_fn(|cx| {
        assert!(future.as_mut().poll(cx).is_pending());
        Poll::Ready(())
    })
    .await;
}

#[tokio::test]
async fn bounded_producer_and_pipeline_preserve_receive_order() {
    let (sender, input) = mpsc::channel(1);
    let (_cancel, cancelled) = oneshot::channel();
    let producer = async move {
        for job in [
            Job { id: 7, value: 3 },
            Job { id: 1, value: 0 },
            Job {
                id: 7,
                value: u32::MAX,
            },
        ] {
            sender.send(job).await.unwrap();
        }
    };
    let (_, output) = timeout(DEADLINE, async {
        tokio::join!(producer, run_async_pipeline(input, cancelled))
    })
    .await
    .unwrap();
    assert_eq!(
        output,
        Ok(vec![
            JobResult { id: 7, value: 9 },
            JobResult { id: 1, value: 0 },
            JobResult {
                id: 7,
                value: 18_446_744_065_119_617_025
            }
        ])
    );
}

#[tokio::test]
async fn last_input_sender_drop_finishes_empty_pipeline() {
    let (sender, input) = mpsc::channel(1);
    let (_cancel, cancelled) = oneshot::channel();
    drop(sender);
    assert_eq!(
        timeout(DEADLINE, run_async_pipeline(input, cancelled))
            .await
            .unwrap(),
        Ok(vec![])
    );
}

#[tokio::test]
async fn cancellation_wakes_pending_receive_and_closes_input() {
    let (sender, input) = mpsc::channel(1);
    let (cancel, cancelled) = oneshot::channel();
    let mut pipeline = Box::pin(run_async_pipeline(input, cancelled));
    assert_pending(pipeline.as_mut()).await;
    cancel.send(()).unwrap();
    assert_eq!(
        timeout(DEADLINE, pipeline).await.unwrap(),
        Err(AsyncPipelineError::Cancelled)
    );
    timeout(DEADLINE, sender.closed()).await.unwrap();
    assert!(sender.send(Job { id: 1, value: 1 }).await.is_err());
}

#[tokio::test]
async fn ready_cancellation_wins_over_buffered_jobs_and_input_closure() {
    let (sender, input) = mpsc::channel(1);
    let (cancel, cancelled) = oneshot::channel();
    sender.send(Job { id: 1, value: 2 }).await.unwrap();
    drop(sender);
    cancel.send(()).unwrap();
    assert_eq!(
        timeout(DEADLINE, run_async_pipeline(input, cancelled))
            .await
            .unwrap(),
        Err(AsyncPipelineError::Cancelled)
    );
}

#[tokio::test]
async fn dropping_cancel_sender_requests_cancellation() {
    let (_sender, input) = mpsc::channel(1);
    let (cancel, cancelled) = oneshot::channel();
    drop(cancel);
    assert_eq!(
        timeout(DEADLINE, run_async_pipeline(input, cancelled))
            .await
            .unwrap(),
        Err(AsyncPipelineError::Cancelled)
    );
}

#[tokio::test]
async fn dropping_pending_pipeline_releases_input_and_has_no_detached_tasks() {
    let (sender, input) = mpsc::channel(1);
    let (_cancel, cancelled) = oneshot::channel();
    let mut pipeline = Box::pin(run_async_pipeline(input, cancelled));
    assert_pending(pipeline.as_mut()).await;
    drop(pipeline);
    timeout(DEADLINE, sender.closed()).await.unwrap();
}
