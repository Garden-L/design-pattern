pub struct Snapshot {
    text: String,
}
pub struct Editor {
    text: String,
}
impl Editor {
    pub fn new(text: &str) -> Self {
        Self { text: text.into() }
    }
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn replace(&mut self, text: &str) {
        self.text = text.into();
    }
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            text: self.text.clone(),
        }
    }
    pub fn restore(&mut self, snapshot: Snapshot) {
        self.text = snapshot.text;
    }
}
