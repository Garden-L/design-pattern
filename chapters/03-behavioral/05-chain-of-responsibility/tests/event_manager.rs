use std::cell::Cell;
use std::rc::Rc;

use chain_of_responsibility::{
    Event, EventDecision, EventHandler, EventManager, EventRequest, EventTransition,
};

fn event(id: &'static str, priority: u32, urgent: bool) -> Event {
    Event {
        id,
        priority,
        urgent,
    }
}

struct ImmediateActivation {
    calls: Rc<Cell<u32>>,
}

impl EventHandler for ImmediateActivation {
    fn handle(&self, request: &EventRequest) -> EventDecision {
        self.calls.set(self.calls.get() + 1);
        request
            .candidates
            .iter()
            .find(|candidate| candidate.urgent)
            .cloned()
            .map(EventDecision::Activate)
            .unwrap_or(EventDecision::Continue)
    }
}

struct MustNotRun {
    calls: Rc<Cell<u32>>,
}

impl EventHandler for MustNotRun {
    fn handle(&self, _request: &EventRequest) -> EventDecision {
        self.calls.set(self.calls.get() + 1);
        EventDecision::Continue
    }
}

#[test]
fn immediate_activation_stops_the_chain() {
    let first_calls = Rc::new(Cell::new(0));
    let later_calls = Rc::new(Cell::new(0));
    let mut manager = EventManager::new();
    manager.push(ImmediateActivation {
        calls: first_calls.clone(),
    });
    manager.push(MustNotRun {
        calls: later_calls.clone(),
    });

    let transition = manager.process(EventRequest::new(vec![event("alarm", 100, true)]));

    assert_eq!(first_calls.get(), 1);
    assert_eq!(later_calls.get(), 0);
    assert_eq!(manager.active_event(), Some(&event("alarm", 100, true)));
    assert_eq!(transition.activated, Some(event("alarm", 100, true)));
}

#[test]
fn replacement_reports_deactivation_and_keeps_one_active_event() {
    let mut manager = EventManager::with_handler(HighestPriority);
    let first = event("normal", 10, false);
    let second = event("urgent", 20, false);

    manager.process(EventRequest::new(vec![first.clone()]));
    let transition = manager.process(EventRequest::new(vec![second.clone()]));

    assert_eq!(transition.deactivated, Some(first));
    assert_eq!(transition.activated, Some(second.clone()));
    assert_eq!(manager.active_event(), Some(&second));
}

#[test]
fn no_selected_event_clears_the_active_event() {
    let mut manager = EventManager::with_handler(HighestPriority);
    let current = event("normal", 10, false);
    manager.process(EventRequest::new(vec![current.clone()]));

    let transition = manager.process(EventRequest::empty());

    assert_eq!(transition, EventTransition::deactivated(current));
    assert_eq!(manager.active_event(), None);
}

struct HighestPriority;

impl EventHandler for HighestPriority {
    fn handle(&self, request: &EventRequest) -> EventDecision {
        request
            .candidates
            .iter()
            .max_by_key(|candidate| candidate.priority)
            .cloned()
            .map(EventDecision::Activate)
            .unwrap_or(EventDecision::Continue)
    }
}
