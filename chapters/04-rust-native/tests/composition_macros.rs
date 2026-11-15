use rust_native::function_composition::{compose, normalize, positive, InputError};
#[path = "../07-function-composition/examples/02_function_composition.rs"]
mod after_composition;
#[path = "../08-macros/examples/02_macros.rs"]
mod after_macros;
#[path = "../07-function-composition/examples/01_direct.rs"]
mod before_composition;
#[path = "../08-macros/examples/01_direct.rs"]
mod before_macros;
#[test]
fn composition_preserves_typed_errors_and_short_circuits() {
    for (text, expected) in [
        (" 12 ", Ok(12)),
        ("", Err(InputError::Invalid)),
        ("x", Err(InputError::Invalid)),
        ("0", Err(InputError::NonPositive)),
    ] {
        assert_eq!(before_composition::parse(text), expected);
        assert_eq!(after_composition::parse(text), expected);
    }
    let called = std::cell::Cell::new(false);
    let pipeline = compose(normalize, |n| {
        called.set(true);
        positive(n)
    });
    assert_eq!(pipeline("bad"), Err(InputError::Invalid));
    assert!(!called.get());
    assert_eq!(pipeline("1"), Ok(1));
    assert!(called.get());
}
#[test]
fn macro_preserves_order_duplicates_and_evaluates_each_expression_once() {
    assert_eq!(
        before_macros::row("Ada", "Rust"),
        after_macros::row("Ada", "Rust")
    );
    assert_eq!(
        after_macros::row("", ""),
        vec![
            ("name".to_string(), "".to_string()),
            ("topic".to_string(), "".to_string())
        ]
    );
    let mut calls = 0;
    let rows = rust_native::string_pairs!("x" => { calls += 1; calls }, "x" => 2,);
    assert_eq!(calls, 1);
    assert_eq!(
        rows,
        vec![
            ("x".to_string(), "1".to_string()),
            ("x".to_string(), "2".to_string())
        ]
    );
    let empty: Vec<(String, String)> = rust_native::string_pairs!();
    assert!(empty.is_empty());
}
