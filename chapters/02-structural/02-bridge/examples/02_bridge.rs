use bridge::{Brackets, Notice, Plain};
pub fn render(text: &str, urgent: bool, brackets: bool) -> String {
    if brackets {
        Notice::new(Brackets).render(text, urgent)
    } else {
        Notice::new(Plain).render(text, urgent)
    }
}
#[allow(dead_code)]
pub fn main() {}
