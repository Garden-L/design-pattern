use rust_native::{
    enum_state_machine as connection, error_design, extension_trait, interior_mutability, raii,
    typestate,
};
use std::error::Error;

#[path = "../01-enum-state-machine/examples/02_enum_state_machine.rs"]
mod after_enum;
#[path = "../05-error-design/examples/02_error_design.rs"]
mod after_error;
#[path = "../06-extension-trait/examples/02_extension_trait.rs"]
mod after_extension;
#[path = "../04-interior-mutability/examples/02_interior_mutability.rs"]
mod after_interior;
#[path = "../03-raii/examples/02_raii.rs"]
mod after_raii;
#[path = "../02-typestate/examples/02_typestate.rs"]
mod after_typestate;
#[path = "../01-enum-state-machine/examples/01_direct.rs"]
mod before_enum;
#[path = "../05-error-design/examples/01_direct.rs"]
mod before_error;
#[path = "../06-extension-trait/examples/01_direct.rs"]
mod before_extension;
#[path = "../04-interior-mutability/examples/01_direct.rs"]
mod before_interior;
#[path = "../03-raii/examples/01_direct.rs"]
mod before_raii;
#[path = "../02-typestate/examples/01_direct.rs"]
mod before_typestate;

#[test]
fn connection_transitions_and_invalid_operations() {
    use connection::{ConnectionError, ConnectionState};
    assert_eq!(
        ConnectionState::Disconnected.send(),
        Err(ConnectionError::NotConnected)
    );
    assert_eq!(
        ConnectionState::Disconnected.connect(),
        Ok(ConnectionState::Connected)
    );
    assert_eq!(
        ConnectionState::Connected.connect(),
        Err(ConnectionError::AlreadyConnected)
    );
    assert_eq!(ConnectionState::Connected.send(), Ok(()));
    assert_eq!(
        ConnectionState::Connected.disconnect(),
        Ok(ConnectionState::Disconnected)
    );
    assert_eq!(
        ConnectionState::Disconnected.disconnect(),
        Err(ConnectionError::NotConnected)
    );
    for run in [before_enum::run, after_enum::run] {
        assert_eq!(
            run(&[
                connection::Action::Connect,
                connection::Action::Send,
                connection::Action::Disconnect
            ]),
            Ok(ConnectionState::Disconnected)
        );
        assert_eq!(
            run(&[connection::Action::Send]),
            Err(ConnectionError::NotConnected)
        );
        assert_eq!(
            run(&[connection::Action::Connect, connection::Action::Connect]),
            Err(ConnectionError::AlreadyConnected)
        );
        assert_eq!(run(&[]), Ok(ConnectionState::Disconnected));
    }
}

#[test]
fn typestate_requires_both_fields_and_preserves_values() {
    let request: typestate::Request = typestate::RequestBuilder::new()
        .method("GET")
        .url("/health")
        .build();
    assert_eq!(
        (request.method.as_str(), request.url.as_str()),
        ("GET", "/health")
    );
    let request = typestate::RequestBuilder::new()
        .url("/health")
        .method("GET")
        .build();
    assert_eq!(request.method, "GET");
    for run in [before_typestate::run, after_typestate::run] {
        assert_eq!(
            run(Some("POST"), Some("/orders")),
            Ok(typestate::Request {
                method: "POST".into(),
                url: "/orders".into()
            })
        );
        assert_eq!(
            run(None, Some("/orders")),
            Err(typestate::BuildError::MissingMethod)
        );
        assert_eq!(
            run(Some("GET"), None),
            Err(typestate::BuildError::MissingUrl)
        );
        assert_eq!(run(None, None), Err(typestate::BuildError::MissingMethod));
    }
}

#[test]
fn raii_releases_on_return_and_unwind() {
    let tracker = raii::Tracker::new();
    assert_eq!(tracker.active(), 0);
    {
        let _guard = tracker.enter();
        assert_eq!(tracker.active(), 1);
    }
    assert_eq!(tracker.active(), 0);
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = tracker.enter();
        panic!("unwind");
    }));
    assert_eq!(tracker.active(), 0);
    for run in [before_raii::run, after_raii::run] {
        assert_eq!(run(false), (1, 0));
        assert_eq!(run(true), (1, 0));
    }
}

#[test]
fn interior_mutability_records_through_shared_reference() {
    let log = interior_mutability::AuditLog::new();
    let shared = &log;
    shared.record("created");
    shared.record("paid");
    assert_eq!(shared.entries(), vec!["created", "paid"]);
    for run in [before_interior::run, after_interior::run] {
        assert_eq!(run(&["created", "paid"]), vec!["created", "paid"]);
        assert!(run(&[]).is_empty());
    }
}

#[test]
fn app_error_preserves_parse_source_and_inputs() {
    let error = error_design::parse_count("oops").unwrap_err();
    assert!(matches!(error, error_design::AppError::Parse(_)));
    assert!(error.source().is_some());
    assert!(error
        .source()
        .unwrap()
        .to_string()
        .contains("invalid digit"));
    let io_error = error_design::AppError::from(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "missing file",
    ));
    assert_eq!(io_error.source().unwrap().to_string(), "missing file");
    assert!(error_design::AppError::MissingCount.source().is_none());
    for run in [before_error::run, after_error::run] {
        assert_eq!(run("42").unwrap(), 42);
        assert!(matches!(run("oops"), Err(error_design::AppError::Parse(_))));
        assert!(matches!(run(""), Err(error_design::AppError::MissingCount)));
    }
}

#[test]
fn extension_trait_matches_all_callers() {
    use extension_trait::SlugExt;
    assert_eq!("  Rust Native  ".slug(), "rust-native");
    assert_eq!("A   B".slug(), "a-b");
    assert_eq!("".slug(), "");
    for run in [before_extension::run, after_extension::run] {
        assert_eq!(run("  Rust Native  "), "rust-native");
        assert_eq!(run("A   B"), "a-b");
        assert_eq!(run(""), "");
    }
}
