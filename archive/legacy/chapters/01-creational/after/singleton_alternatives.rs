use creational::singleton_alternatives::{Config, Service};
use std::sync::Arc;

pub fn describe(name: &str, item: &str) -> String {
    let shared = Arc::new(Config::new(name));
    Service::new(shared).describe(item)
}
