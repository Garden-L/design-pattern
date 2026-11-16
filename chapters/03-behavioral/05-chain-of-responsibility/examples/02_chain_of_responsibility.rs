use chain_of_responsibility::{
    Event, EventDecision, EventHandler, EventManager, EventRequest, EventTransition,
};

struct EmergencyHandler;

impl EventHandler for EmergencyHandler {
    fn handle(&self, request: &EventRequest) -> EventDecision {
        request
            .candidates
            .iter()
            .find(|event| event.urgent)
            .cloned()
            .map(EventDecision::Activate)
            .unwrap_or(EventDecision::Continue)
    }
}

struct PriorityHandler;

impl EventHandler for PriorityHandler {
    fn handle(&self, request: &EventRequest) -> EventDecision {
        request
            .candidates
            .iter()
            .max_by_key(|event| event.priority)
            .cloned()
            .map(EventDecision::Activate)
            .unwrap_or(EventDecision::Continue)
    }
}

pub fn run(candidates: Vec<Event>) -> EventTransition {
    let mut manager = EventManager::new();
    manager.push(EmergencyHandler);
    manager.push(PriorityHandler);
    manager.process(EventRequest::new(candidates))
}

#[allow(dead_code)]
fn main() {
    let transition = run(vec![
        Event {
            id: "normal",
            priority: 10,
            urgent: false,
        },
        Event {
            id: "alarm",
            priority: 100,
            urgent: true,
        },
    ]);
    println!("active = {:?}", transition.active.map(|event| event.id));
}
