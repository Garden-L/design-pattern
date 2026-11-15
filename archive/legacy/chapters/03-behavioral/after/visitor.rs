use behavioral::visitor::{Element, Summary};

pub fn render(elements: &[Element]) -> String {
    let mut summary = Summary::default();
    for element in elements {
        element.accept(&mut summary);
    }
    summary.finish()
}
