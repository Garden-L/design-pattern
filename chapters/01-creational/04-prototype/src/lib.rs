pub trait Prototype {
    fn clone_box(&self) -> Box<dyn Prototype>;
    fn render(&self) -> String;
}

#[derive(Clone)]
pub struct ReportTemplate {
    title: String,
    body: String,
}

impl ReportTemplate {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
        }
    }
}
impl Prototype for ReportTemplate {
    fn clone_box(&self) -> Box<dyn Prototype> {
        Box::new(self.clone())
    }
    fn render(&self) -> String {
        format!("{}\n{}", self.title, self.body)
    }
}
