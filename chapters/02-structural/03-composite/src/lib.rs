pub trait Node {
    fn render(&self) -> String;
}

pub struct Text(String);
impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }
}
impl Node for Text {
    fn render(&self) -> String {
        self.0.clone()
    }
}

#[derive(Default)]
pub struct Group {
    children: Vec<Box<dyn Node>>,
}
impl Group {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, child: impl Node + 'static) {
        self.children.push(Box::new(child));
    }
}
impl Node for Group {
    fn render(&self) -> String {
        self.children.iter().map(|child| child.render()).collect()
    }
}
