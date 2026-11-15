use creational::abstract_factory::{self, Dark, Light};
pub fn screen(dark: bool) -> (String, String) {
    abstract_factory::screen(if dark { &Dark } else { &Light })
}
