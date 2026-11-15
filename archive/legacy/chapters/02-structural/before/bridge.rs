pub fn render(text: &str, urgent: bool, brackets: bool) -> String {
    match (urgent, brackets) {
        (false, false) => text.into(),
        (false, true) => format!("[{text}]"),
        (true, false) => format!("! {text}"),
        (true, true) => format!("[! {text}]"),
    }
}
