use factory_method::{EmailFactory, NotificationFactory, SmsFactory};

pub fn email(message: &str) -> String {
    EmailFactory.create(message).deliver()
}
pub fn sms(message: &str) -> String {
    SmsFactory.create(message).deliver()
}
#[allow(dead_code)]
pub fn main() {}
