use std::{sync::mpsc, thread, time::Duration};

// The deadline only catches hangs. All assertions depend on messages or joins.
pub fn bounded<T: Send + 'static>(run: impl FnOnce() -> T + Send + 'static) -> T {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || sender.send(run()).unwrap());
    let result = receiver
        .recv_timeout(Duration::from_secs(5))
        .expect("operation failed or did not terminate");
    handle.join().unwrap();
    result
}
