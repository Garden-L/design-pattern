use rust_native::raii::Tracker;

pub fn run(early: bool) -> (usize, usize) {
    let tracker = Tracker::new();
    fn work(tracker: &Tracker, early: bool) -> usize {
        let _guard = tracker.enter();
        let during = tracker.active();
        if early {
            return during;
        }
        during
    }
    let during = work(&tracker, early);
    (during, tracker.active())
}
