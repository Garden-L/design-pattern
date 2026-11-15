use std::collections::hash_map::Entry;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatError {
    UnknownSender,
}
#[derive(Default)]
pub struct ChatRoom {
    inboxes: std::collections::HashMap<String, Vec<String>>,
}
impl ChatRoom {
    pub fn register(&mut self, name: &str) -> bool {
        if let Entry::Vacant(entry) = self.inboxes.entry(name.into()) {
            entry.insert(Vec::new());
            true
        } else {
            false
        }
    }
    pub fn send(&mut self, sender: &str, text: &str) -> Result<(), ChatError> {
        if !self.inboxes.contains_key(sender) {
            return Err(ChatError::UnknownSender);
        }
        for (name, inbox) in &mut self.inboxes {
            if name != sender {
                inbox.push(format!("{sender}: {text}"));
            }
        }
        Ok(())
    }
    pub fn inbox(&self, name: &str) -> Option<&[String]> {
        self.inboxes.get(name).map(Vec::as_slice)
    }
}
