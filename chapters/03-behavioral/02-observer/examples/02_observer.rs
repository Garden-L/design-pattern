use observer::{Event, EventBus};
use std::{cell::RefCell, rc::Rc};

pub fn run(events: &[&str]) -> Vec<String> {
    let bus = EventBus::new();
    let delivered = Rc::new(RefCell::new(Vec::new()));
    for recipient in ["email", "audit"] {
        let output = Rc::clone(&delivered);
        bus.subscribe(move |event| output.borrow_mut().push(format!("{recipient}:{}", event.0)));
    }
    for event in events {
        bus.publish(Event((*event).into()));
    }
    let result = delivered.borrow().clone();
    result
}
#[allow(dead_code)]
pub fn main() {}
