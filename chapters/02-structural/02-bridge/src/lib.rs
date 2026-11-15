pub trait Renderer {
    fn render(&self, text: &str) -> String;
}
pub struct Plain;
pub struct Brackets;
impl Renderer for Plain {
    fn render(&self, text: &str) -> String {
        text.into()
    }
}
impl Renderer for Brackets {
    fn render(&self, text: &str) -> String {
        format!("[{text}]")
    }
}
/// Message policy and rendering mechanism vary independently.
pub struct Notice<R> {
    renderer: R,
}
impl<R: Renderer> Notice<R> {
    pub fn new(renderer: R) -> Self {
        Self { renderer }
    }
    pub fn render(&self, text: &str, urgent: bool) -> String {
        let message = if urgent {
            format!("! {text}")
        } else {
            text.into()
        };
        self.renderer.render(&message)
    }
}
