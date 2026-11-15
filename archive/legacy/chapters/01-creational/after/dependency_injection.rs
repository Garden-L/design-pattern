use creational::dependency_injection::{NotificationSender, Notifier};

pub fn notify<S: NotificationSender>(sender: S, message: &str) {
    Notifier::new(sender).notify(message);
}
