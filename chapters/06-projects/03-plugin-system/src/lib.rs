//! Runtime composition of statically linked plugins; no dynamic library ABI.
use rust_native::error_design::AppError;
use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
    fmt, io,
};
use structural::proxy::{CachedProxy, DocumentStore, StoreError};

#[derive(Debug)]
pub enum PluginError {
    InvalidName,
    InvalidRequest,
    DuplicatePlugin(String),
    UnknownPlugin(String),
    Store(StoreError),
    App(AppError),
}
impl fmt::Display for PluginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName => f.write_str("plugin name must not be blank"),
            Self::InvalidRequest => f.write_str("document key must not be blank"),
            Self::DuplicatePlugin(name) => write!(f, "plugin already registered: {name}"),
            Self::UnknownPlugin(name) => write!(f, "unknown plugin: {name}"),
            Self::Store(error) => write!(f, "document store failed: {error:?}"),
            Self::App(error) => write!(f, "legacy plugin failed: {error}"),
        }
    }
}
impl Error for PluginError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::App(error) => Some(error),
            _ => None,
        }
    }
}
/// Object-safe boundary; concrete plugin types are erased only in the registry.
pub trait Plugin {
    fn execute(&self, key: &str) -> Result<String, PluginError>;
}
pub trait LegacyReader {
    fn read_text(&self, key: &str) -> io::Result<String>;
}
/// Adapter from the chapter's store contract, composed with its caching proxy.
pub struct DocumentPlugin<S: DocumentStore>(CachedProxy<S>);
impl<S: DocumentStore> DocumentPlugin<S> {
    pub fn new(store: S) -> Self {
        Self(CachedProxy::new(store))
    }
}
impl<S: DocumentStore> Plugin for DocumentPlugin<S> {
    fn execute(&self, key: &str) -> Result<String, PluginError> {
        self.0.fetch(key).map_err(PluginError::Store)
    }
}
pub struct LegacyAdapter<L>(L);
impl<L: LegacyReader> LegacyAdapter<L> {
    pub fn new(reader: L) -> Self {
        Self(reader)
    }
}
impl<L: LegacyReader> Plugin for LegacyAdapter<L> {
    fn execute(&self, key: &str) -> Result<String, PluginError> {
        self.0
            .read_text(key)
            .map_err(|error| PluginError::App(AppError::from(error)))
    }
}
#[derive(Default)]
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
}
impl PluginRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(
        &mut self,
        name: &str,
        plugin: impl Plugin + 'static,
    ) -> Result<(), PluginError> {
        if name.trim().is_empty() {
            return Err(PluginError::InvalidName);
        }
        match self.plugins.entry(name.into()) {
            Entry::Vacant(entry) => {
                entry.insert(Box::new(plugin));
                Ok(())
            }
            Entry::Occupied(_) => Err(PluginError::DuplicatePlugin(name.into())),
        }
    }
    pub fn run(&self, name: &str, key: &str) -> Result<String, PluginError> {
        if key.trim().is_empty() {
            return Err(PluginError::InvalidRequest);
        }
        self.plugins
            .get(name)
            .ok_or_else(|| PluginError::UnknownPlugin(name.into()))?
            .execute(key)
    }
}
