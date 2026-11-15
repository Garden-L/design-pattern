use behavioral::{chain, command, iterator, observer, state, strategy, visitor};
use std::{cell::RefCell, rc::Rc};

#[path = "../05-chain-of-responsibility/examples/02_chain_of_responsibility.rs"]
mod after_chain;
#[path = "../03-command/examples/02_command.rs"]
mod after_command;
#[path = "../06-iterator/examples/02_iterator.rs"]
mod after_iterator;
#[path = "../02-observer/examples/02_observer.rs"]
mod after_observer;
#[path = "../04-state/examples/02_state.rs"]
mod after_state;
#[path = "../01-strategy/examples/02_strategy.rs"]
mod after_strategy;
#[path = "../07-visitor/examples/02_visitor.rs"]
mod after_visitor;
#[path = "../05-chain-of-responsibility/examples/01_direct.rs"]
mod before_chain;
#[path = "../03-command/examples/01_direct.rs"]
mod before_command;
#[path = "../06-iterator/examples/01_direct.rs"]
mod before_iterator;
#[path = "../02-observer/examples/01_direct.rs"]
mod before_observer;
#[path = "../04-state/examples/01_direct.rs"]
mod before_state;
#[path = "../01-strategy/examples/01_direct.rs"]
mod before_strategy;
#[path = "../07-visitor/examples/01_direct.rs"]
mod before_visitor;

#[test]
fn strategy_substitution_preserves_prices() {
    use strategy::{Order, PricingStrategy};
    let order = Order { subtotal: 105 };
    assert_eq!(strategy::Regular.price(&order), 105);
    assert_eq!(strategy::TenPercentOff.price(&order), 95);
    assert_eq!(strategy::price_with(&strategy::TenPercentOff, &order), 95);
    let selected: &dyn PricingStrategy = &strategy::TenPercentOff;
    assert_eq!(selected.price(&order), 95);
    assert_eq!(strategy::price_with(selected, &order), 95);
    // A caller-defined strategy requires no change to Order or price_with.
    struct HalfPrice;
    impl PricingStrategy for HalfPrice {
        fn price(&self, order: &Order) -> strategy::Money {
            order.subtotal / 2
        }
    }
    assert_eq!(strategy::price_with(&HalfPrice, &order), 52);
    for quote in [before_strategy::quote, after_strategy::quote] {
        assert_eq!(quote(105, false), 105);
        assert_eq!(quote(105, true), 95);
        assert_eq!(quote(0, true), 0);
        assert_eq!(quote(i64::MAX, true), 8_301_034_833_169_298_227);
        assert_eq!(quote(-105, true), -95);
    }
}

#[test]
fn observer_delivers_in_registration_order_and_unsubscribes() {
    let bus = observer::EventBus::new();
    let seen = Rc::new(RefCell::new(Vec::new()));
    let first = seen.clone();
    let id = bus.subscribe(move |event| first.borrow_mut().push(format!("first:{}", event.0)));
    let second = seen.clone();
    bus.subscribe(move |event| second.borrow_mut().push(format!("second:{}", event.0)));
    bus.publish(observer::Event("paid".into()));
    assert!(bus.unsubscribe(id));
    assert!(!bus.unsubscribe(id));
    bus.publish(observer::Event("shipped".into()));
    assert_eq!(
        *seen.borrow(),
        ["first:paid", "second:paid", "second:shipped"]
    );
    for run in [before_observer::run, after_observer::run] {
        assert_eq!(
            run(&["paid", "shipped"]),
            ["email:paid", "audit:paid", "email:shipped", "audit:shipped"]
        );
        assert!(run(&[]).is_empty());
    }
}

#[test]
fn command_execute_undo_and_invalid_replay() {
    use command::Command;
    let mut context = command::Context { balance: 10 };
    let mut add = command::Add::new(4);
    assert_eq!(add.execute(&mut context), Ok(()));
    assert_eq!(context.balance, 14);
    assert_eq!(
        add.execute(&mut context),
        Err(command::CommandError::AlreadyExecuted)
    );
    assert_eq!(add.undo(&mut context), Ok(()));
    assert_eq!(context.balance, 10);
    assert_eq!(
        add.undo(&mut context),
        Err(command::CommandError::NotExecuted)
    );
    for run in [before_command::run, after_command::run] {
        assert_eq!(run(10, 4), Ok((14, 10)));
        assert_eq!(run(i32::MAX, 1), Err(command::CommandError::Overflow));
        assert_eq!(run(i32::MIN, -1), Err(command::CommandError::Overflow));
        assert_eq!(run(10, -4), Ok((6, 10)));
        assert_eq!(run(10, 0), Ok((10, 10)));
    }
}

#[test]
fn state_accepts_only_valid_transitions() {
    use state::{OrderState, TransitionError};
    assert_eq!(
        OrderState::Draft.ship(),
        Err(TransitionError::InvalidTransition)
    );
    assert_eq!(OrderState::Draft.place(), Ok(OrderState::Placed));
    assert_eq!(
        OrderState::Placed.place(),
        Err(TransitionError::InvalidTransition)
    );
    assert_eq!(OrderState::Placed.ship(), Ok(OrderState::Shipped));
    assert_eq!(
        OrderState::Shipped.ship(),
        Err(TransitionError::InvalidTransition)
    );
    for run in [before_state::run, after_state::run] {
        assert_eq!(
            run(&[state::Action::Place, state::Action::Ship]),
            Ok(OrderState::Shipped)
        );
        assert_eq!(
            run(&[state::Action::Ship]),
            Err(TransitionError::InvalidTransition)
        );
        assert_eq!(run(&[]), Ok(OrderState::Draft));
        assert_eq!(run(&[state::Action::Place]), Ok(OrderState::Placed));
        assert_eq!(
            run(&[state::Action::Place, state::Action::Place]),
            Err(TransitionError::InvalidTransition)
        );
        assert_eq!(
            run(&[
                state::Action::Place,
                state::Action::Ship,
                state::Action::Place
            ]),
            Err(TransitionError::InvalidTransition)
        );
        assert_eq!(
            run(&[
                state::Action::Place,
                state::Action::Ship,
                state::Action::Ship
            ]),
            Err(TransitionError::InvalidTransition)
        );
    }
}

#[test]
fn chain_short_circuits_on_response_or_rejection() {
    use chain::{ChainError, Handler, Request, Response};
    let calls = Rc::new(RefCell::new(Vec::new()));
    struct Probe(
        Rc<RefCell<Vec<&'static str>>>,
        &'static str,
        Result<Response, ChainError>,
    );
    impl Handler for Probe {
        fn handle(&self, _: &Request) -> Result<Response, ChainError> {
            self.0.borrow_mut().push(self.1);
            self.2.clone()
        }
    }
    let mut chain = chain::EventManager::new();
    chain.push(Probe(calls.clone(), "skip", Err(ChainError::Continue)));
    chain.push(Probe(calls.clone(), "answer", Ok(Response("ok".into()))));
    chain.push(Probe(calls.clone(), "unreached", Err(ChainError::Rejected)));
    assert_eq!(
        chain.handle(&Request("item".into())),
        Ok(Response("ok".into()))
    );
    assert_eq!(*calls.borrow(), ["skip", "answer"]);
    for run in [before_chain::run, after_chain::run] {
        assert_eq!(run("public"), Ok("public data".into()));
        assert_eq!(run("private"), Err(ChainError::Rejected));
        assert_eq!(run("other"), Err(ChainError::Unhandled));
        assert_eq!(run(""), Err(ChainError::Unhandled));
    }
}

#[test]
fn iterator_exhausts_and_handles_empty_input() {
    let numbers = [2, 4, 6];
    let mut cursor = iterator::Cursor::new(&numbers);
    assert_eq!(cursor.next(), Some(&2));
    assert_eq!(cursor.next(), Some(&4));
    assert_eq!(cursor.next(), Some(&6));
    assert_eq!(cursor.next(), None);
    assert_eq!(cursor.next(), None);
    for collect in [before_iterator::collect, after_iterator::collect] {
        assert_eq!(collect(&numbers), vec![2, 4, 6]);
        assert!(collect(&[]).is_empty());
    }
}

#[test]
fn visitor_covers_every_element() {
    use visitor::{Element, Summary};
    let elements = [
        Element::Text("hi".into()),
        Element::Number(7),
        Element::Text("!".into()),
    ];
    let mut summary = Summary::default();
    for element in &elements {
        element.accept(&mut summary);
    }
    assert_eq!(summary.finish(), "hi7!");
    for render in [before_visitor::render, after_visitor::render] {
        assert_eq!(render(&elements), "hi7!");
        assert_eq!(render(&[]), "");
        assert_eq!(
            render(&[
                Element::Text(String::new()),
                Element::Number(-12),
                Element::Number(0)
            ]),
            "-120"
        );
    }
}

#[test]
fn observer_changes_during_delivery_apply_to_the_next_event() {
    use observer::{Event, EventBus};
    use std::cell::Cell;
    let bus = Rc::new(EventBus::new());
    let seen = Rc::new(RefCell::new(Vec::new()));
    let changed = Rc::new(Cell::new(false));
    let removed = Rc::new(Cell::new(0));
    let weak_bus = Rc::downgrade(&bus);
    let output = seen.clone();
    let removed_id = removed.clone();
    bus.subscribe(move |event| {
        output.borrow_mut().push(format!("first:{}", event.0));
        if !changed.replace(true) {
            let bus = weak_bus.upgrade().unwrap();
            assert!(bus.unsubscribe(removed_id.get()));
            let late_output = output.clone();
            bus.subscribe(move |event| late_output.borrow_mut().push(format!("late:{}", event.0)));
        }
    });
    let output = seen.clone();
    removed
        .set(bus.subscribe(move |event| output.borrow_mut().push(format!("second:{}", event.0))));
    bus.publish(Event("one".into()));
    bus.publish(Event("two".into()));
    assert_eq!(
        *seen.borrow(),
        ["first:one", "second:one", "first:two", "late:two"]
    );
    EventBus::new().publish(Event("no subscribers".into()));
}

#[test]
fn command_overflow_preserves_context_and_replay_state() {
    use command::{Add, Command, CommandError, Context};
    let mut context = Context { balance: i32::MAX };
    let mut add = Add::new(1);
    assert_eq!(add.undo(&mut context), Err(CommandError::NotExecuted));
    assert_eq!(add.execute(&mut context), Err(CommandError::Overflow));
    assert_eq!(context.balance, i32::MAX);
    assert_eq!(add.undo(&mut context), Err(CommandError::NotExecuted));
    context.balance = 0;
    assert_eq!(add.execute(&mut context), Ok(()));
    context.balance = i32::MIN;
    assert_eq!(add.undo(&mut context), Err(CommandError::Overflow));
    assert_eq!(context.balance, i32::MIN);
    assert_eq!(
        add.execute(&mut context),
        Err(CommandError::AlreadyExecuted)
    );
    context.balance = 1;
    assert_eq!(add.undo(&mut context), Ok(()));
    assert_eq!(context.balance, 0);
    assert_eq!(add.execute(&mut context), Ok(()));
    assert_eq!(context.balance, 1);
}

#[test]
fn chain_rejection_skips_later_handlers_and_all_pass_is_unhandled() {
    use chain::{EventManager, ChainError, Handler, Request, Response};
    struct Reject;
    impl Handler for Reject {
        fn handle(&self, _: &Request) -> Result<Response, ChainError> {
            Err(ChainError::Rejected)
        }
    }
    struct Unreachable;
    impl Handler for Unreachable {
        fn handle(&self, _: &Request) -> Result<Response, ChainError> {
            panic!("ran after rejection")
        }
    }
    struct Pass;
    impl Handler for Pass {
        fn handle(&self, _: &Request) -> Result<Response, ChainError> {
            Err(ChainError::Continue)
        }
    }
    let request = Request("key".into());
    let mut chain = EventManager::new();
    assert_eq!(chain.handle(&request), Err(ChainError::Unhandled));
    chain.push(Pass);
    assert_eq!(chain.handle(&request), Err(ChainError::Unhandled));
    chain.push(Reject);
    chain.push(Unreachable);
    assert_eq!(chain.handle(&request), Err(ChainError::Rejected));
}

#[test]
fn empty_cursor_stays_exhausted_and_composes_with_adaptors() {
    let empty: [i32; 0] = [];
    let mut cursor = iterator::Cursor::new(&empty);
    assert_eq!(cursor.next(), None);
    assert_eq!(cursor.next(), None);
    let words = [String::from("one"), String::from("two")];
    assert_eq!(
        iterator::Cursor::new(&words)
            .map(String::len)
            .collect::<Vec<_>>(),
        [3, 3]
    );
}

#[test]
fn visitor_supports_a_second_operation_without_changing_elements() {
    use visitor::{Element, Visitor};
    #[derive(Default)]
    struct Counts {
        text: usize,
        number: usize,
    }
    impl Visitor for Counts {
        fn visit_text(&mut self, _: &str) {
            self.text += 1;
        }
        fn visit_number(&mut self, _: i32) {
            self.number += 1;
        }
    }
    let elements = [
        Element::Number(0),
        Element::Text("hi".into()),
        Element::Number(-1),
    ];
    let mut counts = Counts::default();
    for element in &elements {
        element.accept(&mut counts);
    }
    assert_eq!((counts.text, counts.number), (1, 2));
}
