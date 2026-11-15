// Each new recipient requires editing this publishing loop.
pub fn run(events: &[&str]) -> Vec<String> {
    let mut delivered = Vec::new();
    for event in events {
        delivered.push(format!("email:{event}"));
        delivered.push(format!("audit:{event}"));
    }
    delivered
}
#[allow(dead_code)]
pub fn main() {}
