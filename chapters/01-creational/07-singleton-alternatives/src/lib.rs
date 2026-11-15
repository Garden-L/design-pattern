use std::sync::Arc;

/// Immutable settings can be shared explicitly with every consumer.
pub struct Config {
    name: String,
}
impl Config {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

pub struct Service {
    config: Arc<Config>,
}
impl Service {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
    pub fn describe(&self, item: &str) -> String {
        format!("{}: {item}", self.config.name)
    }
}
