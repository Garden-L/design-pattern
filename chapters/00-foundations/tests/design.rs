use foundations::design::{self, AmountError, Decimal, Hex};
#[path = "../01-design-foundations/examples/02_design_foundations.rs"]
mod after;
#[path = "../01-design-foundations/examples/01_direct.rs"]
mod before;
#[test]
fn borrowed_input_generic_and_dynamic_dispatch_preserve_results() {
    for (input, expected) in [
        (" 42 ", Ok("42".to_string())),
        ("0", Ok("0".into())),
        ("", Err(AmountError::Empty)),
        ("x", Err(AmountError::InvalidNumber)),
        ("-1", Err(AmountError::InvalidNumber)),
    ] {
        let owned = input.to_string();
        assert_eq!(before::render(&owned, false), expected);
        assert_eq!(after::render(&owned, false), expected);
        assert_eq!(design::render(&owned, &Decimal), expected);
        assert_eq!(design::render_dyn(&owned, &Decimal), expected);
        assert_eq!(owned, input);
    }
    assert_eq!(design::render("42", &Hex), Ok("2a".into()));
    assert_eq!(design::render_dyn("42", &Hex), Ok("2a".into()));
}
