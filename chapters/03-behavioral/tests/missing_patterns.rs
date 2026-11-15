use behavioral::{
    interpreter::{self, EvalError},
    mediator::{ChatError, ChatRoom},
    memento::Editor,
    template_method::{Empty, Lower, Pipeline, Upper},
};
#[path = "../11-interpreter/examples/02_interpreter.rs"]
mod after_interpreter;
#[path = "../09-mediator/examples/02_mediator.rs"]
mod after_mediator;
#[path = "../10-memento/examples/02_memento.rs"]
mod after_memento;
#[path = "../08-template-method/examples/02_template_method.rs"]
mod after_template;
#[path = "../11-interpreter/examples/01_direct.rs"]
mod before_interpreter;
#[path = "../09-mediator/examples/01_direct.rs"]
mod before_mediator;
#[path = "../10-memento/examples/01_direct.rs"]
mod before_memento;
#[path = "../08-template-method/examples/01_direct.rs"]
mod before_template;
#[test]
fn template_preserves_validation_order_and_swaps_only_transform() {
    for s in ["", "   ", " Hi "] {
        for upper in [false, true] {
            assert_eq!(
                before_template::run(s, upper),
                after_template::run(s, upper)
            );
        }
    }
    assert_eq!(Upper.run(" Hi "), Ok("HI".into()));
    assert_eq!(Lower.run(" Hi "), Ok("hi".into()));
    assert_eq!(Upper.run(" "), Err(Empty));
}
#[test]
fn mediator_routes_without_self_delivery_and_rejects_unknown_sender() {
    let mut room = ChatRoom::default();
    assert!(room.register("a"));
    assert!(!room.register("a"));
    assert!(room.register("b"));
    room.send("a", "").unwrap();
    room.send("a", "hello").unwrap();
    assert_eq!(room.inbox("a"), Some(&[][..]));
    assert_eq!(
        room.inbox("b"),
        Some(&["a: ".to_string(), "a: hello".to_string()][..])
    );
    assert_eq!(room.send("missing", "bad"), Err(ChatError::UnknownSender));
    assert_eq!(room.inbox("missing"), None);
    assert_eq!(room.inbox("b").unwrap().len(), 2);
    for messages in [vec![], vec!["", "hi"]] {
        assert_eq!(
            before_mediator::conversation(&messages),
            after_mediator::conversation(&messages)
        );
    }
}
#[test]
fn memento_restores_opaque_snapshot_without_aliasing_current_text() {
    let mut editor = Editor::new("draft");
    let snapshot = editor.snapshot();
    editor.replace("published");
    editor.restore(snapshot);
    assert_eq!(editor.text(), "draft");
    for edits in [vec![], vec!["a"], vec!["a", "", "b"]] {
        assert_eq!(
            before_memento::undo_last(&edits),
            after_memento::undo_last(&edits)
        );
    }
    assert_eq!(after_memento::undo_last(&["a", "b"]), "a");
}
#[test]
fn interpreter_handles_nested_postfix_and_reports_all_invalid_boundaries() {
    for (input, expected) in [
        ("2 3 + 4 +", Ok(9)),
        ("-2 3 +", Ok(1)),
        ("0", Ok(0)),
        ("", Err(EvalError::Syntax)),
        ("1 +", Err(EvalError::Syntax)),
        ("1 2", Err(EvalError::Syntax)),
        ("abc", Err(EvalError::Syntax)),
        ("9223372036854775807 1 +", Err(EvalError::Overflow)),
    ] {
        assert_eq!(before_interpreter::evaluate(input), expected);
        assert_eq!(after_interpreter::evaluate(input), expected);
    }
    let expression = interpreter::parse("2 3 +").unwrap();
    assert_eq!(expression.evaluate(), Ok(5));
}
