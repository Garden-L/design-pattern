use prototype::{Prototype, ReportTemplate};

pub fn copy_template(title: &str, body: &str) -> String {
    let original: Box<dyn Prototype> = Box::new(ReportTemplate::new(title, body));
    original.clone_box().render()
}
#[allow(dead_code)]
pub fn main() {}
