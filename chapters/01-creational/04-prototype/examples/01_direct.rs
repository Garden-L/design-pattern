pub fn copy_template(title: &str, body: &str) -> String {
    let original = (title.to_owned(), body.to_owned());
    let copy = (original.0.clone(), original.1.clone());
    format!("{}\n{}", copy.0, copy.1)
}
#[allow(dead_code)]
pub fn main() {}
