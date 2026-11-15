use plugin_system::{
    DocumentPlugin, LegacyAdapter, LegacyReader, Plugin, PluginError, PluginRegistry,
};
use std::{cell::Cell, error::Error, io, rc::Rc};
use structural::proxy::{DocumentStore, StoreError};

struct FakePlugin(&'static str);
impl Plugin for FakePlugin {
    fn execute(&self, key: &str) -> Result<String, PluginError> {
        Ok(format!("{}:{key}", self.0))
    }
}
struct ChangingStore(Rc<Cell<usize>>);
impl DocumentStore for ChangingStore {
    fn fetch(&self, key: &str) -> Result<String, StoreError> {
        self.0.set(self.0.get() + 1);
        match key {
            "guide" => Ok(format!("guide:load-{}", self.0.get())),
            _ => Err(StoreError::NotFound),
        }
    }
}
struct ExternalReader;
impl LegacyReader for ExternalReader {
    fn read_text(&self, key: &str) -> io::Result<String> {
        if key == "private" {
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "restricted document",
            ))
        } else {
            Ok(format!("legacy:{key}"))
        }
    }
}

#[test]
fn registry_dispatches_heterogeneous_plugins_including_a_fake() {
    let mut registry = PluginRegistry::new();
    registry.register("fake", FakePlugin("fake")).unwrap();
    registry
        .register("legacy", LegacyAdapter::new(ExternalReader))
        .unwrap();
    assert_eq!(registry.run("fake", "guide").unwrap(), "fake:guide");
    assert_eq!(registry.run("legacy", "guide").unwrap(), "legacy:guide");
}

#[test]
fn document_adapter_uses_chapter_proxy_and_does_not_cache_failures() {
    let loads = Rc::new(Cell::new(0));
    let mut registry = PluginRegistry::new();
    registry
        .register("docs", DocumentPlugin::new(ChangingStore(loads.clone())))
        .unwrap();
    assert_eq!(registry.run("docs", "guide").unwrap(), "guide:load-1");
    assert_eq!(registry.run("docs", "guide").unwrap(), "guide:load-1");
    assert_eq!(loads.get(), 1);
    for _ in 0..2 {
        assert!(matches!(
            registry.run("docs", "missing"),
            Err(PluginError::Store(StoreError::NotFound))
        ));
    }
    assert_eq!(loads.get(), 3);
}

#[test]
fn legacy_adapter_preserves_error_sources_through_type_erasure() {
    let mut registry = PluginRegistry::new();
    registry
        .register("legacy", LegacyAdapter::new(ExternalReader))
        .unwrap();
    let error = registry.run("legacy", "private").unwrap_err();
    assert!(matches!(error, PluginError::App(_)));
    let app = error.source().expect("AppError source");
    let io = app.source().unwrap().downcast_ref::<io::Error>().unwrap();
    assert_eq!(io.kind(), io::ErrorKind::PermissionDenied);
    assert_eq!(io.to_string(), "restricted document");
}

#[test]
fn invalid_registration_does_not_replace_existing_plugins() {
    let mut registry = PluginRegistry::new();
    assert!(matches!(
        registry.register(" ", FakePlugin("bad")),
        Err(PluginError::InvalidName)
    ));
    registry.register("fake", FakePlugin("original")).unwrap();
    assert!(
        matches!(registry.register("fake", FakePlugin("replacement")), Err(PluginError::DuplicatePlugin(name)) if name == "fake")
    );
    assert_eq!(registry.run("fake", "guide").unwrap(), "original:guide");
    assert!(
        matches!(registry.run("absent", "guide"), Err(PluginError::UnknownPlugin(name)) if name == "absent")
    );
}

#[test]
fn blank_requests_are_rejected_before_touching_the_store() {
    let loads = Rc::new(Cell::new(0));
    let mut registry = PluginRegistry::new();
    registry
        .register("docs", DocumentPlugin::new(ChangingStore(loads.clone())))
        .unwrap();
    assert!(matches!(
        registry.run("docs", "\t "),
        Err(PluginError::InvalidRequest)
    ));
    assert_eq!(loads.get(), 0);
}
