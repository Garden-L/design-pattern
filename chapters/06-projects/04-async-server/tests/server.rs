use async_server::{Job, JobResult, ServerBuilder, ServerError};
use concurrency_async::{async_pipeline::AsyncPipelineError, work_queue::QueueError};
use rust_native::enum_state_machine::{ConnectionError, ConnectionState};
use std::{
    error::Error,
    future::{poll_fn, Future},
    pin::Pin,
    task::Poll,
    time::Duration,
};
use tokio::{
    sync::{mpsc, oneshot},
    time::timeout,
};

const DEADLINE: Duration = Duration::from_secs(5);
async fn assert_pending<F: Future>(mut future: Pin<&mut F>) {
    poll_fn(|cx| {
        assert!(future.as_mut().poll(cx).is_pending());
        Poll::Ready(())
    })
    .await;
}

#[test]
fn builder_rejects_zero_workers_and_capacity_before_starting_resources() {
    let error = ServerBuilder::new()
        .workers(0)
        .build()
        .err()
        .expect("zero workers rejected");
    assert!(matches!(error, ServerError::Queue(QueueError::ZeroWorkers)));
    assert!(error
        .source()
        .unwrap()
        .downcast_ref::<QueueError>()
        .is_some());
    assert!(matches!(
        ServerBuilder::new().capacity(0).build(),
        Err(ServerError::InvalidCapacity)
    ));
}

#[tokio::test]
async fn state_rejects_invalid_operations_and_eof_allows_restart() {
    let mut server = ServerBuilder::new().build().unwrap();
    assert_eq!(server.state(), ConnectionState::Disconnected);
    let (sender, input) = mpsc::channel(1);
    let (_cancel, cancelled) = oneshot::channel();
    assert!(matches!(
        server.serve(input, cancelled).await,
        Err(ServerError::State(ConnectionError::NotConnected))
    ));
    assert!(sender.is_closed());
    server.start().unwrap();
    assert!(matches!(
        server.start(),
        Err(ServerError::State(ConnectionError::AlreadyConnected))
    ));
    for _ in 0..2 {
        let (sender, input) = mpsc::channel(1);
        let (_cancel, cancelled) = oneshot::channel();
        drop(sender);
        assert!(timeout(DEADLINE, server.serve(input, cancelled))
            .await
            .unwrap()
            .unwrap()
            .is_empty());
        assert_eq!(server.state(), ConnectionState::Disconnected);
        server.start().unwrap();
    }
}

#[tokio::test]
async fn bounded_requests_are_processed_by_work_queue_in_receive_order() {
    let mut server = ServerBuilder::new().workers(2).capacity(1).build().unwrap();
    server.start().unwrap();
    let (sender, input) = mpsc::channel(1);
    let (_cancel, cancelled) = oneshot::channel();
    let producer = async move {
        for job in [
            Job { id: 9, value: 3 },
            Job { id: 1, value: 0 },
            Job {
                id: 9,
                value: u32::MAX,
            },
            Job { id: 2, value: 4 },
        ] {
            sender.send(job).await.unwrap();
        }
    };
    let (_, output) = timeout(DEADLINE, async {
        tokio::join!(producer, server.serve(input, cancelled))
    })
    .await
    .unwrap();
    assert_eq!(
        output.unwrap(),
        [
            JobResult { id: 9, value: 9 },
            JobResult { id: 1, value: 0 },
            JobResult {
                id: 9,
                value: 18_446_744_065_119_617_025
            },
            JobResult { id: 2, value: 16 }
        ]
    );
    assert_eq!(server.state(), ConnectionState::Disconnected);
}

#[tokio::test]
async fn pending_receive_cancels_joins_and_closes_input() {
    let mut server = ServerBuilder::new().build().unwrap();
    server.start().unwrap();
    let (sender, input) = mpsc::channel(1);
    let (cancel, cancelled) = oneshot::channel();
    let mut service = Box::pin(server.serve(input, cancelled));
    assert_pending(service.as_mut()).await;
    cancel.send(()).unwrap();
    assert!(matches!(
        timeout(DEADLINE, service).await.unwrap(),
        Err(ServerError::Cancelled(AsyncPipelineError::Cancelled))
    ));
    assert!(sender.is_closed());
    assert_eq!(server.state(), ConnectionState::Disconnected);
}

#[tokio::test]
async fn ready_cancellation_wins_over_buffered_requests_and_eof() {
    let mut server = ServerBuilder::new().build().unwrap();
    server.start().unwrap();
    let (sender, input) = mpsc::channel(1);
    let (cancel, cancelled) = oneshot::channel();
    sender.send(Job { id: 1, value: 2 }).await.unwrap();
    drop(sender);
    cancel.send(()).unwrap();
    assert!(matches!(
        timeout(DEADLINE, server.serve(input, cancelled))
            .await
            .unwrap(),
        Err(ServerError::Cancelled(_))
    ));
    assert_eq!(server.state(), ConnectionState::Disconnected);
}

#[tokio::test]
async fn cancellation_sender_drop_requests_shutdown_with_input_still_open() {
    let mut server = ServerBuilder::new().build().unwrap();
    server.start().unwrap();
    let (sender, input) = mpsc::channel(1);
    let (cancel, cancelled) = oneshot::channel();
    let mut service = Box::pin(server.serve(input, cancelled));
    assert_pending(service.as_mut()).await;
    drop(cancel);
    assert!(matches!(
        timeout(DEADLINE, service).await.unwrap(),
        Err(ServerError::Cancelled(_))
    ));
    assert!(sender.is_closed());
    assert_eq!(server.state(), ConnectionState::Disconnected);
}

#[test]
fn cancellation_while_bridge_is_full_waits_for_drain_and_join() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .max_blocking_threads(1)
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let (started_tx, started_rx) = std::sync::mpsc::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel::<()>();
        let blocker = tokio::task::spawn_blocking(move || {
            started_tx.send(()).unwrap();
            release_rx.recv().unwrap();
        });
        started_rx.recv_timeout(DEADLINE).unwrap();

        let mut server = ServerBuilder::new().capacity(1).build().unwrap();
        server.start().unwrap();
        let (sender, input) = mpsc::channel(1);
        let (cancel, cancelled) = oneshot::channel();
        sender.send(Job { id: 1, value: 2 }).await.unwrap();
        let mut service = Box::pin(server.serve(input, cancelled));
        assert_pending(service.as_mut()).await;
        sender.send(Job { id: 2, value: 3 }).await.unwrap();
        assert_pending(service.as_mut()).await;
        assert_eq!(sender.capacity(), 1); // Both requests left input; one fills the bridge.

        sender.send(Job { id: 3, value: 4 }).await.unwrap();
        let mut blocked_send = Box::pin(sender.send(Job { id: 4, value: 5 }));
        assert_pending(blocked_send.as_mut()).await;
        cancel.send(()).unwrap();
        assert_pending(service.as_mut()).await; // Cleanup awaits the queued blocking task.
        assert!(sender.is_closed());
        assert!(blocked_send.await.is_err());
        assert_pending(service.as_mut()).await;

        release_tx.send(()).unwrap();
        timeout(DEADLINE, blocker).await.unwrap().unwrap();
        assert!(matches!(
            timeout(DEADLINE, service).await.unwrap(),
            Err(ServerError::Cancelled(AsyncPipelineError::Cancelled))
        ));
        assert_eq!(server.state(), ConnectionState::Disconnected);
    });
}

#[tokio::test]
async fn dropping_pending_service_closes_input_and_resets_state() {
    let mut server = ServerBuilder::new().build().unwrap();
    server.start().unwrap();
    let (sender, input) = mpsc::channel(1);
    let (_cancel, cancelled) = oneshot::channel();
    let mut service = Box::pin(server.serve(input, cancelled));
    assert_pending(service.as_mut()).await;
    drop(service);
    assert!(sender.is_closed());
    assert_eq!(server.state(), ConnectionState::Disconnected);
}
