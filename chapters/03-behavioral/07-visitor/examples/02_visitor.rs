use visitor::{Element, Summary};

pub fn render(elements: &[Element]) -> String {
    let mut summary = Summary::default();
    for element in elements {
        element.accept(&mut summary);
    }
    summary.finish()
}
#[allow(dead_code)]
pub fn main() {}
