pub fn undo_last(edits: &[&str]) -> String {
    let mut text = String::new();
    let mut history = Vec::new();
    for edit in edits {
        history.push(text.clone());
        text = (*edit).into();
    }
    if let Some(previous) = history.pop() {
        text = previous;
    }
    text
}
