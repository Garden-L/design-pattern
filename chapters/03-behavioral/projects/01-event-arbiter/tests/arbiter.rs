use event_arbiter::{
    Context, DistanceScorer, Event, EventArbiter, EventFilter, EventListener, EventScorer,
    MaxDistance, PriorityScorer, RecordingListener, RejectCooldown, SelectionResult,
};

fn event(id: &'static str, priority: i32, distance: u32, cooldown: bool) -> Event {
    Event {
        id,
        priority,
        distance,
        cooldown,
    }
}

fn arbiter_with_default_pipeline() -> EventArbiter {
    EventArbiter::new(
        vec![Box::new(RejectCooldown), Box::new(MaxDistance::new(10))],
        Box::new(PriorityScorer),
    )
}

#[test]
fn filters_candidates_before_scoring() {
    let mut arbiter = arbiter_with_default_pipeline();
    let result = arbiter.reconcile(
        &[
            event("cooldown", 100, 1, true),
            event("far-away", 90, 11, false),
        ],
        &Context { now: 42 },
    );

    assert_eq!(result.active, None);
    assert!(!result.changed);
    assert!(result.reason.contains("filter"));
}

#[test]
fn selects_highest_scored_candidate_and_returns_result() {
    let mut arbiter = arbiter_with_default_pipeline();
    let result = arbiter.reconcile(
        &[event("low", 1, 1, false), event("high", 5, 9, false)],
        &Context { now: 42 },
    );

    assert_eq!(
        result,
        SelectionResult {
            previous: None,
            active: Some("high"),
            deactivated: None,
            reason: "selected highest-scoring candidate: high".into(),
            changed: true,
        }
    );
    assert_eq!(arbiter.active(), Some("high"));
}

#[test]
fn combines_priority_and_distance_strategies() {
    let mut arbiter = EventArbiter::new(
        vec![],
        Box::new(event_arbiter::CombinedScorer::new(vec![
            Box::new(PriorityScorer),
            Box::new(DistanceScorer),
        ])),
    );
    let result = arbiter.reconcile(
        &[event("near", 5, 1, false), event("far", 10, 8, false)],
        &Context { now: 42 },
    );

    assert_eq!(result.active, Some("near"));
}

#[test]
fn same_active_event_is_notified_once() {
    let mut arbiter = arbiter_with_default_pipeline();
    let listener = RecordingListener::new();
    let view = listener.clone();
    arbiter.add_listener(listener);
    let candidate = [event("stable", 5, 1, false)];

    let first = arbiter.reconcile(&candidate, &Context { now: 1 });
    let second = arbiter.reconcile(&candidate, &Context { now: 2 });

    assert!(first.changed);
    assert!(!second.changed);
    assert_eq!(view.records(), vec![first]);
}

#[test]
fn switching_active_event_reports_both_sides() {
    let mut arbiter = arbiter_with_default_pipeline();
    let listener = RecordingListener::new();
    let view = listener.clone();
    arbiter.add_listener(listener);

    arbiter.reconcile(&[event("a", 10, 1, false)], &Context { now: 1 });
    let result = arbiter.reconcile(&[event("b", 20, 1, false)], &Context { now: 2 });

    assert_eq!(result.previous, Some("a"));
    assert_eq!(result.deactivated, Some("a"));
    assert_eq!(result.active, Some("b"));
    assert!(result.changed);
    assert_eq!(view.records().last(), Some(&result));
}

#[test]
fn clearing_active_event_reports_deactivation() {
    let mut arbiter = arbiter_with_default_pipeline();
    arbiter.reconcile(&[event("a", 10, 1, false)], &Context { now: 1 });

    let result = arbiter.reconcile(&[], &Context { now: 2 });

    assert_eq!(result.previous, Some("a"));
    assert_eq!(result.active, None);
    assert_eq!(result.deactivated, Some("a"));
    assert!(result.changed);
}

#[test]
fn tie_breaking_is_deterministic() {
    let mut arbiter = EventArbiter::new(vec![], Box::new(PriorityScorer));
    let context = Context { now: 1 };
    let first = arbiter.reconcile(
        &[event("zulu", 5, 2, false), event("alpha", 5, 2, false)],
        &context,
    );
    let second = arbiter.reconcile(
        &[event("alpha", 5, 2, false), event("zulu", 5, 2, false)],
        &context,
    );

    assert_eq!(first.active, Some("alpha"));
    assert!(!second.changed);
    assert_eq!(second.active, Some("alpha"));
}

#[test]
fn listener_trait_can_receive_selection_changes() {
    struct Counter(usize);
    impl EventListener for Counter {
        fn on_selection_changed(&mut self, _result: &SelectionResult) {
            self.0 += 1;
        }
    }

    let mut arbiter = EventArbiter::new(vec![], Box::new(PriorityScorer));
    arbiter.add_listener(Counter(0));
    let _ = arbiter.reconcile(&[event("a", 1, 1, false)], &Context { now: 0 });
}

#[test]
fn filter_and_scorer_traits_are_extensible() {
    struct EvenPriority;
    impl EventFilter for EvenPriority {
        fn accepts(&self, event: &Event, _context: &Context) -> bool {
            event.priority % 2 == 0
        }
    }

    struct ContextAwareScorer;
    impl EventScorer for ContextAwareScorer {
        fn score(&self, event: &Event, context: &Context) -> i32 {
            event.priority + context.now as i32
        }
    }

    let mut arbiter = EventArbiter::new(vec![Box::new(EvenPriority)], Box::new(ContextAwareScorer));
    let result = arbiter.reconcile(
        &[event("odd", 9, 1, false), event("even", 2, 1, false)],
        &Context { now: 3 },
    );

    assert_eq!(result.active, Some("even"));
}
