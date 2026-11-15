/// A notification created with its message already owned.
pub trait Notification {
    fn deliver(&self) -> String;
}

pub trait NotificationFactory {
    fn create(&self, message: &str) -> Box<dyn Notification>;
}

pub struct EmailFactory;
pub struct SmsFactory;

struct EmailNotification(String);
struct SmsNotification(String);

impl Notification for EmailNotification {
    fn deliver(&self) -> String {
        format!("EMAIL: {}", self.0)
    }
}
impl Notification for SmsNotification {
    fn deliver(&self) -> String {
        format!("SMS: {}", self.0)
    }
}
impl NotificationFactory for EmailFactory {
    fn create(&self, message: &str) -> Box<dyn Notification> {
        Box::new(EmailNotification(message.to_owned()))
    }
}
impl NotificationFactory for SmsFactory {
    fn create(&self, message: &str) -> Box<dyn Notification> {
        Box::new(SmsNotification(message.to_owned()))
    }
}
