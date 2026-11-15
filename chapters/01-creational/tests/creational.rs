#[path = "../03-builder/examples/02_builder.rs"]
mod after_builder;
#[path = "../06-dependency-injection/examples/02_dependency_injection.rs"]
mod after_di;
#[path = "../01-factory-method/examples/02_factory_method.rs"]
mod after_factory;
#[path = "../04-prototype/examples/02_prototype.rs"]
mod after_prototype;
#[path = "../07-singleton-alternatives/examples/02_singleton_alternatives.rs"]
mod after_singleton;
#[path = "../03-builder/examples/01_direct.rs"]
mod before_builder;
#[path = "../06-dependency-injection/examples/01_direct.rs"]
mod before_di;
#[path = "../01-factory-method/examples/01_direct.rs"]
mod before_factory;
#[path = "../04-prototype/examples/01_direct.rs"]
mod before_prototype;
#[path = "../07-singleton-alternatives/examples/01_direct.rs"]
mod before_singleton;

use creational::{
    builder, dependency_injection as di, factory_method as factory, prototype,
    singleton_alternatives as singleton,
};
use factory::NotificationFactory;
use std::{cell::RefCell, sync::Arc};

#[test]
fn factory_before_and_after_format_both_products() {
    for (before, after) in [
        (
            before_factory::email("Hello"),
            factory::EmailFactory.create("Hello").deliver(),
        ),
        (
            before_factory::sms("Hello"),
            factory::SmsFactory.create("Hello").deliver(),
        ),
    ] {
        assert_eq!(before, after);
    }
    assert_eq!(before_factory::email("Hello"), "EMAIL: Hello");
    assert_eq!(before_factory::sms("Hello"), "SMS: Hello");
    assert_eq!(after_factory::email("Hello"), "EMAIL: Hello");
    assert_eq!(after_factory::sms("Hello"), "SMS: Hello");
}

#[test]
fn builder_before_and_after_render_same_report() {
    let before = before_builder::report("Quarterly", "Ready").unwrap();
    let after = builder::ReportBuilder::new()
        .title("Quarterly")
        .body("Ready")
        .build()
        .unwrap();
    assert_eq!(before, after.render());
    assert_eq!(after.render(), "Quarterly\nReady");
    assert_eq!(after_builder::report("Quarterly", "Ready"), Ok(before));
}

#[test]
fn builder_rejects_missing_and_blank_required_fields() {
    assert_eq!(
        before_builder::report("", "Ready"),
        Err(before_builder::BuildError::MissingTitle)
    );
    assert_eq!(
        builder::ReportBuilder::new().body("Ready").build(),
        Err(builder::BuildError::MissingTitle)
    );
    assert_eq!(
        before_builder::report("Quarterly", "  "),
        Err(before_builder::BuildError::MissingBody)
    );
    assert_eq!(
        builder::ReportBuilder::new()
            .title("Quarterly")
            .body("  ")
            .build(),
        Err(builder::BuildError::MissingBody)
    );
    assert_eq!(
        after_builder::report("", "Ready"),
        Err(builder::BuildError::MissingTitle)
    );
}

#[test]
fn prototype_clones_independent_trait_objects() {
    let before = before_prototype::copy_template("Weekly", "Draft");
    let original: Box<dyn prototype::Prototype> =
        Box::new(prototype::ReportTemplate::new("Weekly", "Draft"));
    let copy = original.clone_box();
    assert_eq!(before, copy.render());
    assert_eq!(copy.render(), "Weekly\nDraft");
    assert_eq!(after_prototype::copy_template("Weekly", "Draft"), before);
    drop(original);
    assert_eq!(copy.render(), "Weekly\nDraft");
}

struct RecordingSender(RefCell<Vec<String>>);
impl RecordingSender {
    fn drain(&self) -> Vec<String> {
        std::mem::take(&mut *self.0.borrow_mut())
    }
}
impl di::NotificationSender for RecordingSender {
    fn send(&self, message: &str) {
        self.0.borrow_mut().push(format!("EMAIL: {message}"));
    }
}

#[test]
fn injection_sends_same_message_without_hard_coded_transport() {
    let before = before_di::notify("Hello");
    assert_eq!(before, vec!["EMAIL: Hello"]);
    let sender = RecordingSender(RefCell::new(Vec::new()));
    let notifier = di::Notifier::new(sender);
    notifier.notify("Hello");
    assert_eq!(*notifier.sender().0.borrow(), before);
    let sender = RecordingSender(RefCell::new(Vec::new()));
    after_di::notify(&sender, "Hello");
    assert_eq!(*sender.0.borrow(), before);
}

#[test]
fn injection_forwards_empty_messages_once_in_order_and_can_be_reused() {
    let sender = RecordingSender(RefCell::new(Vec::new()));
    let notifier = di::Notifier::new(&sender);
    for message in ["first", "", "last"] {
        notifier.notify(message);
    }
    assert_eq!(sender.drain(), ["EMAIL: first", "EMAIL: ", "EMAIL: last"]);
    notifier.notify("again");
    assert_eq!(sender.drain(), ["EMAIL: again"]);
    assert!(sender.drain().is_empty());
}

#[test]
fn factory_generic_and_trait_object_callers_use_identical_fixtures() {
    fn generic<F: NotificationFactory>(factory: &F, message: &str) -> String {
        factory.create(message).deliver()
    }
    fn dynamic(factory: &dyn NotificationFactory, message: &str) -> String {
        factory.create(message).deliver()
    }
    for (message, email, sms) in [
        ("", "EMAIL: ", "SMS: "),
        ("Hello", "EMAIL: Hello", "SMS: Hello"),
        ("안녕", "EMAIL: 안녕", "SMS: 안녕"),
    ] {
        assert_eq!(generic(&factory::EmailFactory, message), email);
        assert_eq!(dynamic(&factory::EmailFactory, message), email);
        assert_eq!(generic(&factory::SmsFactory, message), sms);
        assert_eq!(dynamic(&factory::SmsFactory, message), sms);
    }
}

#[test]
fn explicit_shared_config_gives_services_same_values() {
    let before = before_singleton::describe("course", "chapter 1");
    let config = Arc::new(singleton::Config::new("course"));
    let first = singleton::Service::new(Arc::clone(&config));
    let second = singleton::Service::new(config);
    assert_eq!(first.describe("chapter 1"), before);
    assert_eq!(second.describe("chapter 2"), "course: chapter 2");
    assert_eq!(after_singleton::describe("course", "chapter 1"), before);
}
