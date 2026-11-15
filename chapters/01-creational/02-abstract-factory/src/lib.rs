pub trait Widget {
    fn label(&self) -> String;
}
struct Button(&'static str);
struct Checkbox(&'static str);
impl Widget for Button {
    fn label(&self) -> String {
        format!("{} button", self.0)
    }
}
impl Widget for Checkbox {
    fn label(&self) -> String {
        format!("{} checkbox", self.0)
    }
}
pub trait UiFactory {
    fn button(&self) -> Box<dyn Widget>;
    fn checkbox(&self) -> Box<dyn Widget>;
}
pub struct Light;
pub struct Dark;
impl UiFactory for Light {
    fn button(&self) -> Box<dyn Widget> {
        Box::new(Button("light"))
    }
    fn checkbox(&self) -> Box<dyn Widget> {
        Box::new(Checkbox("light"))
    }
}
impl UiFactory for Dark {
    fn button(&self) -> Box<dyn Widget> {
        Box::new(Button("dark"))
    }
    fn checkbox(&self) -> Box<dyn Widget> {
        Box::new(Checkbox("dark"))
    }
}
pub fn screen(factory: &dyn UiFactory) -> (String, String) {
    (factory.button().label(), factory.checkbox().label())
}
