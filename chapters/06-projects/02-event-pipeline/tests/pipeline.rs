use behavioral::{chain::ChainError, command::CommandError};
use concurrency_async::actor::ActorError;
use event_pipeline::{EventPipeline, PipelineError, Summary};
use std::{cell::RefCell, rc::Rc, sync::mpsc, thread, time::Duration};

// A missing shutdown signal/join or an invalidated live handle fails by deadline.
fn bounded(test: impl FnOnce() + Send + 'static) {
    let (done, result) = mpsc::channel();
    let worker = thread::spawn(move || {
        test();
        done.send(()).unwrap();
    });
    result
        .recv_timeout(Duration::from_secs(5))
        .expect("pipeline finished");
    worker.join().unwrap();
}

#[test]
fn commands_notify_observers_in_order_and_actor_counts_accepted_events() {
    bounded(|| {
        let mut pipeline = EventPipeline::new();
        let observed = Rc::new(RefCell::new(Vec::new()));
        for label in ["audit", "display"] {
            let observed = observed.clone();
            pipeline
                .subscribe(move |event| observed.borrow_mut().push(format!("{label}:{}", event.0)));
        }
        assert_eq!(pipeline.process("add 5"), Ok(5));
        assert_eq!(pipeline.process("add -2"), Ok(3));
        assert_eq!(
            *observed.borrow(),
            [
                "audit:balance=5",
                "display:balance=5",
                "audit:balance=3",
                "display:balance=3"
            ]
        );
        let metrics = pipeline.metrics();
        assert_eq!(metrics.value(), Ok(2));
        assert_eq!(
            pipeline.shutdown(),
            Summary {
                balance: 3,
                processed: 2
            }
        );
        assert_eq!(metrics.value(), Err(ActorError::Closed));
    });
}

#[test]
fn validation_and_overflow_have_no_command_observer_or_metric_effect() {
    bounded(|| {
        let mut pipeline = EventPipeline::new();
        let events = Rc::new(RefCell::new(Vec::new()));
        let output = events.clone();
        pipeline.subscribe(move |event| output.borrow_mut().push(event.0.clone()));
        for request in [
            "",
            "   ",
            "add",
            "add nope",
            "add 1 extra",
            "add 2147483648",
        ] {
            assert_eq!(
                pipeline.process(request),
                Err(PipelineError::Chain(ChainError::Rejected)),
                "{request:?}"
            );
        }
        assert_eq!(
            pipeline.process("multiply 2"),
            Err(PipelineError::Chain(ChainError::Unhandled))
        );
        assert_eq!(pipeline.process("add 2147483647"), Ok(i32::MAX));
        assert_eq!(
            pipeline.process("add 1"),
            Err(PipelineError::Command(CommandError::Overflow))
        );
        assert_eq!(*events.borrow(), ["balance=2147483647"]);
        assert_eq!(
            pipeline.shutdown(),
            Summary {
                balance: i32::MAX,
                processed: 1
            }
        );
    });
}

#[test]
fn empty_shutdown_and_drop_close_actor_with_live_handles() {
    bounded(|| {
        let pipeline = EventPipeline::new();
        let metrics = pipeline.metrics();
        assert_eq!(
            pipeline.shutdown(),
            Summary {
                balance: 0,
                processed: 0
            }
        );
        assert_eq!(metrics.value(), Err(ActorError::Closed));
        let mut pipeline = EventPipeline::new();
        let metrics = pipeline.metrics();
        pipeline.process("add 8").unwrap();
        drop(pipeline);
        assert_eq!(metrics.value(), Err(ActorError::Closed));
    });
}

#[test]
fn shutdown_drains_many_acknowledged_commands() {
    bounded(|| {
        let mut pipeline = EventPipeline::new();
        for _ in 0..100 {
            pipeline.process("add 1").unwrap();
        }
        assert_eq!(
            pipeline.shutdown(),
            Summary {
                balance: 100,
                processed: 100
            }
        );
    });
}
