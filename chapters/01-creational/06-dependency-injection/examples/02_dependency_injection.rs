use dependency_injection::{NotificationSender, Notifier};

pub fn notify<S: NotificationSender>(sender: S, message: &str) {
    Notifier::new(sender).notify(message);
}
#[allow(dead_code)]
pub fn main() {}
