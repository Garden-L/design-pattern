use event_arbiter::{
    Context, Event, EventArbiter, MaxDistance, PriorityScorer, RecordingListener, RejectCooldown,
};

fn main() {
    let mut arbiter = EventArbiter::new(
        vec![Box::new(RejectCooldown), Box::new(MaxDistance::new(20))],
        Box::new(PriorityScorer),
    );
    let listener = RecordingListener::new();
    let view = listener.clone();
    arbiter.add_listener(listener);

    let result = arbiter.reconcile(
        &[
            Event {
                id: "bonus",
                priority: 10,
                distance: 12,
                cooldown: false,
            },
            Event {
                id: "warning",
                priority: 8,
                distance: 3,
                cooldown: false,
            },
        ],
        &Context { now: 1_700_000_000 },
    );

    println!("result={result:?}");
    println!("notifications={:?}", view.records());
}
