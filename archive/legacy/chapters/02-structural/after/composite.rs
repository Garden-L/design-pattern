use structural::composite::{Group, Node, Text};

pub fn render() -> String {
    let mut nested = Group::new();
    nested.push(Text::new("B"));
    nested.push(Text::new("C"));
    let mut root = Group::new();
    root.push(Text::new("A"));
    root.push(nested);
    root.render()
}
