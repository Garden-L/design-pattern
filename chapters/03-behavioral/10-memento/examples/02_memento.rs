use memento::Editor;
pub fn undo_last(edits: &[&str]) -> String {
    let mut editor = Editor::new("");
    let mut history = Vec::new();
    for edit in edits {
        history.push(editor.snapshot());
        editor.replace(edit);
    }
    if let Some(snapshot) = history.pop() {
        editor.restore(snapshot);
    }
    editor.text().into()
}
#[allow(dead_code)]
pub fn main() {}
