// Every return path must remember the matching decrement.
pub fn run(early: bool) -> (usize, usize) {
    let mut active = 0;
    active += 1;
    let during = active;
    if early {
        active -= 1;
        return (during, active);
    }
    active -= 1;
    (during, active)
}
#[allow(dead_code)]
pub fn main() {}
