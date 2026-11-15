use adapter::PaymentProcessor;
use composite::Node;
use decorator::Service;
use proxy::DocumentStore;
use std::{cell::RefCell, rc::Rc};
use structural::{adapter, composite, decorator, facade, newtype, proxy};

type AdapterExample = fn(u64, adapter::LegacyGateway) -> Result<(), adapter::PaymentError>;
type DecoratorExample = fn(&str) -> (Result<String, decorator::ServiceError>, Vec<String>);
type FacadeExample = fn(&str, u32, bool) -> Result<u64, facade::OrderError>;
type ProxyExample = fn(&str) -> (Result<String, proxy::StoreError>, usize);

#[path = "../01-adapter/examples/02_adapter.rs"]
mod after_adapter;
#[path = "../03-composite/examples/02_composite.rs"]
mod after_composite;
#[path = "../04-decorator/examples/02_decorator.rs"]
mod after_decorator;
#[path = "../05-facade/examples/02_facade.rs"]
mod after_facade;
#[path = "../08-newtype/examples/02_newtype.rs"]
mod after_newtype;
#[path = "../07-proxy/examples/02_proxy.rs"]
mod after_proxy;
#[path = "../01-adapter/examples/01_direct.rs"]
mod before_adapter;
#[path = "../03-composite/examples/01_direct.rs"]
mod before_composite;
#[path = "../04-decorator/examples/01_direct.rs"]
mod before_decorator;
#[path = "../05-facade/examples/01_direct.rs"]
mod before_facade;
#[path = "../08-newtype/examples/01_direct.rs"]
mod before_newtype;
#[path = "../07-proxy/examples/01_direct.rs"]
mod before_proxy;

#[test]
fn adapter_translates_success_and_errors() {
    let gateway = adapter::LegacyGateway::new(false);
    let adapted = adapter::LegacyAdapter::new(gateway);
    assert_eq!(adapted.charge(250), Ok(()));
    assert_eq!(adapted.charge(0), Err(adapter::PaymentError::InvalidAmount));
    assert_eq!(
        adapted.charge(u64::MAX),
        Err(adapter::PaymentError::InvalidAmount)
    );
    assert_eq!(
        adapter::LegacyAdapter::new(adapter::LegacyGateway::new(true)).charge(250),
        Err(adapter::PaymentError::Declined)
    );
    assert_eq!(
        adapter::LegacyAdapter::new(adapter::LegacyGateway::offline()).charge(250),
        Err(adapter::PaymentError::Unavailable)
    );
    let examples: [AdapterExample; 2] = [before_adapter::charge, after_adapter::charge];
    for charge in examples {
        assert_eq!(charge(250, adapter::LegacyGateway::new(false)), Ok(()));
        assert_eq!(
            charge(0, adapter::LegacyGateway::new(false)),
            Err(adapter::PaymentError::InvalidAmount)
        );
        assert_eq!(
            charge(u64::MAX, adapter::LegacyGateway::new(false)),
            Err(adapter::PaymentError::InvalidAmount)
        );
        assert_eq!(
            charge(250, adapter::LegacyGateway::new(true)),
            Err(adapter::PaymentError::Declined)
        );
        assert_eq!(
            charge(250, adapter::LegacyGateway::offline()),
            Err(adapter::PaymentError::Unavailable)
        );
    }
}

#[test]
fn adapter_generic_and_trait_object_callers_preserve_identical_errors() {
    fn generic<P: PaymentProcessor>(
        processor: &P,
        amount: u64,
    ) -> Result<(), adapter::PaymentError> {
        processor.charge(amount)
    }
    fn dynamic(processor: &dyn PaymentProcessor, amount: u64) -> Result<(), adapter::PaymentError> {
        processor.charge(amount)
    }
    for (gateway, amount, expected) in [
        (adapter::LegacyGateway::new(false), 250, Ok(())),
        (
            adapter::LegacyGateway::new(false),
            0,
            Err(adapter::PaymentError::InvalidAmount),
        ),
        (
            adapter::LegacyGateway::new(false),
            u64::MAX,
            Err(adapter::PaymentError::InvalidAmount),
        ),
        (
            adapter::LegacyGateway::new(true),
            250,
            Err(adapter::PaymentError::Declined),
        ),
        (
            adapter::LegacyGateway::offline(),
            250,
            Err(adapter::PaymentError::Unavailable),
        ),
    ] {
        let processor = adapter::LegacyAdapter::new(gateway);
        assert_eq!(generic(&processor, amount), expected);
        assert_eq!(dynamic(&processor, amount), expected);
    }
}

struct RecordingService(Rc<RefCell<Vec<String>>>);
impl Service for RecordingService {
    fn execute(&self, input: &str) -> Result<String, decorator::ServiceError> {
        self.0.borrow_mut().push(format!("service:{input}"));
        if input.is_empty() {
            Err(decorator::ServiceError::EmptyInput)
        } else {
            Ok(input.to_uppercase())
        }
    }
}

#[test]
fn decorator_logs_in_order_and_preserves_failure() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let logger = decorator::Logger::new(RecordingService(events.clone()), events.clone());
    assert_eq!(logger.execute("hi"), Ok("HI".into()));
    assert_eq!(logger.execute(""), Err(decorator::ServiceError::EmptyInput));
    assert_eq!(
        *events.borrow(),
        [
            "before:hi",
            "service:hi",
            "after:ok",
            "before:",
            "service:",
            "after:error"
        ]
    );
    let examples: [DecoratorExample; 2] = [before_decorator::execute, after_decorator::execute];
    for execute in examples {
        assert_eq!(
            execute("hi"),
            (
                Ok("HI".into()),
                vec!["before:hi".to_string(), "after:ok".to_string()]
            )
        );
        assert_eq!(
            execute(""),
            (
                Err(decorator::ServiceError::EmptyInput),
                vec!["before:".to_string(), "after:error".to_string()]
            )
        );
    }
}

#[test]
fn composite_renders_nested_nodes_in_child_order() {
    let mut inner = composite::Group::new();
    inner.push(composite::Text::new("B"));
    inner.push(composite::Text::new("C"));
    let mut outer = composite::Group::new();
    outer.push(composite::Text::new("A"));
    outer.push(inner);
    assert_eq!(outer.render(), "ABC");
    assert_eq!(composite::Group::new().render(), "");
    assert_eq!(before_composite::render(), "ABC");
    assert_eq!(after_composite::render(), "ABC");
}

#[test]
fn facade_places_order_and_preserves_stage_errors() {
    let orders =
        facade::OrderFacade::new(facade::Catalog::new(20), facade::CardGateway::new(false));
    let request = facade::OrderRequest::new("book", 2);
    assert_eq!(
        orders.place_order(request.clone()),
        Ok(facade::OrderReceipt::new(40))
    );
    assert_eq!(
        orders.place_order(facade::OrderRequest::new("book", 0)),
        Err(facade::OrderError::InvalidQuantity)
    );
    assert_eq!(
        orders.place_order(facade::OrderRequest::new("missing", 1)),
        Err(facade::OrderError::Inventory(
            facade::InventoryError::UnknownSku
        ))
    );
    let declined =
        facade::OrderFacade::new(facade::Catalog::new(20), facade::CardGateway::new(true));
    assert_eq!(
        declined.place_order(request),
        Err(facade::OrderError::Payment(facade::PaymentError::Declined))
    );
    let examples: [FacadeExample; 2] = [before_facade::place_order, after_facade::place_order];
    for place_order in examples {
        assert_eq!(place_order("book", 2, false), Ok(40));
        assert_eq!(
            place_order("book", 0, false),
            Err(facade::OrderError::InvalidQuantity)
        );
        assert_eq!(
            place_order("missing", 1, false),
            Err(facade::OrderError::Inventory(
                facade::InventoryError::UnknownSku
            ))
        );
        assert_eq!(
            place_order("book", 2, true),
            Err(facade::OrderError::Payment(facade::PaymentError::Declined))
        );
    }
}

#[test]
fn proxy_caches_success_but_not_failure() {
    let calls = Rc::new(RefCell::new(0));
    let store = proxy::MemoryStore::new(calls.clone());
    let cached = proxy::CachedProxy::new(store);
    assert_eq!(cached.fetch("guide"), Ok("Rust guide".into()));
    assert_eq!(cached.fetch("guide"), Ok("Rust guide".into()));
    assert_eq!(*calls.borrow(), 1);
    assert_eq!(cached.fetch("missing"), Err(proxy::StoreError::NotFound));
    assert_eq!(cached.fetch("missing"), Err(proxy::StoreError::NotFound));
    assert_eq!(*calls.borrow(), 3);
    let examples: [(ProxyExample, usize); 2] = [
        (before_proxy::fetch_twice, 2),
        (after_proxy::fetch_twice, 1),
    ];
    for (fetch_twice, success_calls) in examples {
        assert_eq!(
            fetch_twice("guide"),
            (Ok("Rust guide".into()), success_calls)
        );
        assert_eq!(
            fetch_twice("missing"),
            (Err(proxy::StoreError::NotFound), 2)
        );
    }
}

#[test]
fn newtype_requires_explicit_identity_conversion() {
    let user = newtype::UserId::new(7);
    let order = newtype::OrderId::new(7);
    assert_eq!(user.get(), 7);
    assert_eq!(order.get(), 7);
    assert_eq!(newtype::UserId::from_raw(8).into_raw(), 8);
    assert_eq!(before_newtype::describe(7, 8), "user 7 ordered 8");
    assert_eq!(
        after_newtype::describe(user, newtype::OrderId::new(8)),
        "user 7 ordered 8"
    );
}
