use behavioral::visitor::Element;

// Another operation over elements must repeat this match.
pub fn render(elements: &[Element]) -> String {
    let mut output = String::new();
    for element in elements {
        match element {
            Element::Text(value) => output.push_str(value),
            Element::Number(value) => output.push_str(&value.to_string()),
        }
    }
    output
}
