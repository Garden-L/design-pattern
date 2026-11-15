//! Synchronous event delivery in subscription order.
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event(pub String);

pub type SubscriptionId = u64;
type Callback = Rc<dyn Fn(&Event)>;

/// Synchronous, single-threaded delivery. Callbacks are owned until removed.
/// Use weak references when a callback needs access to its own bus.
#[derive(Default)]
pub struct EventBus {
    next_id: Cell<SubscriptionId>,
    subscribers: RefCell<Vec<(SubscriptionId, Callback)>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe(&self, callback: impl Fn(&Event) + 'static) -> SubscriptionId {
        let id = self.next_id.get();
        self.next_id
            .set(id.checked_add(1).expect("subscription IDs exhausted"));
        self.subscribers.borrow_mut().push((id, Rc::new(callback)));
        id
    }

    pub fn unsubscribe(&self, id: SubscriptionId) -> bool {
        let mut subscribers = self.subscribers.borrow_mut();
        let old_len = subscribers.len();
        subscribers.retain(|(candidate, _)| *candidate != id);
        subscribers.len() != old_len
    }

    /// Takes a snapshot so callbacks can subscribe or unsubscribe during delivery.
    /// Changes affect subsequent publication snapshots (including nested publishes),
    /// not callbacks already captured for this event. Callback panics propagate.
    pub fn publish(&self, event: Event) {
        let callbacks: Vec<_> = self
            .subscribers
            .borrow()
            .iter()
            .map(|(_, callback)| Rc::clone(callback))
            .collect();
        for callback in callbacks {
            callback(&event);
        }
    }
}
