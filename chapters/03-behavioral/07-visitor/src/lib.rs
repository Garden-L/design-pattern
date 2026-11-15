//! A visitor adds operations to a fixed set of element types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Element {
    Text(String),
    Number(i32),
}

pub trait Visitor {
    fn visit_text(&mut self, value: &str);
    fn visit_number(&mut self, value: i32);
}

impl Element {
    pub fn accept(&self, visitor: &mut impl Visitor) {
        match self {
            Self::Text(value) => visitor.visit_text(value),
            Self::Number(value) => visitor.visit_number(*value),
        }
    }
}

#[derive(Default)]
pub struct Summary(String);
impl Summary {
    pub fn finish(self) -> String {
        self.0
    }
}
impl Visitor for Summary {
    fn visit_text(&mut self, value: &str) {
        self.0.push_str(value);
    }
    fn visit_number(&mut self, value: i32) {
        self.0.push_str(&value.to_string());
    }
}
