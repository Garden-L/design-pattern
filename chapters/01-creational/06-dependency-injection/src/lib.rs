pub trait NotificationSender {
    fn send(&self, message: &str);
}

impl<S: NotificationSender + ?Sized> NotificationSender for &S {
    fn send(&self, message: &str) {
        (*self).send(message);
    }
}

pub struct Notifier<S: NotificationSender> {
    sender: S,
}

impl<S: NotificationSender> Notifier<S> {
    pub fn new(sender: S) -> Self {
        Self { sender }
    }
    /// Forwards exactly once, including an empty string; transports define delivery.
    pub fn notify(&self, message: &str) {
        self.sender.send(message);
    }
    pub fn sender(&self) -> &S {
        &self.sender
    }
}
