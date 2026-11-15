use std::{collections::HashMap, rc::Rc};
#[derive(Debug)]
pub struct Style {
    color: String,
}
pub struct Glyph {
    pub style: Rc<Style>,
    pub position: usize,
}
impl Glyph {
    pub fn render(&self) -> String {
        format!("{}@{}", self.style.color, self.position)
    }
}
#[derive(Default)]
pub struct StylePool {
    styles: HashMap<String, Rc<Style>>,
}
impl StylePool {
    pub fn glyph(&mut self, color: &str, position: usize) -> Glyph {
        let style = self.styles.entry(color.into()).or_insert_with(|| {
            Rc::new(Style {
                color: color.into(),
            })
        });
        Glyph {
            style: Rc::clone(style),
            position,
        }
    }
}
