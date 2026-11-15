mod support;
use concurrency_async::actor::{ActorError, CounterActor};

#[test]
fn request_reply_preserves_order_and_overflow_preserves_state() {
    support::bounded(|| {
        let actor = CounterActor::start(10);
        let handle = actor.handle();
        assert_eq!(handle.add(2), Ok(12));
        assert_eq!(handle.add(u64::MAX), Err(ActorError::Overflow));
        assert_eq!(handle.value(), Ok(12));
        assert_eq!(handle.add(3), Ok(15));
        assert_eq!(actor.shutdown(), 15);
        assert_eq!(handle.add(1), Err(ActorError::Closed));
        assert_eq!(handle.value(), Err(ActorError::Closed));
    });
}

#[test]
fn cloned_handles_serialize_state_without_lost_updates() {
    support::bounded(|| {
        let actor = CounterActor::start(0);
        std::thread::scope(|scope| {
            for _ in 0..4 {
                let handle = actor.handle();
                scope.spawn(move || {
                    for _ in 0..25 {
                        handle.add(1).unwrap();
                    }
                });
            }
        });
        assert_eq!(actor.shutdown(), 100);
    });
}

#[test]
fn owner_drop_stops_actor_even_with_live_handles() {
    support::bounded(|| {
        let actor = CounterActor::start(0);
        let handle = actor.handle();
        drop(actor);
        assert_eq!(handle.add(1), Err(ActorError::Closed));
    });
}
