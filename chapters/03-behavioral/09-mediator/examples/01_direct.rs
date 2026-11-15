pub fn conversation(messages: &[&str]) -> Vec<String> {
    let mut bob = Vec::new();
    for message in messages {
        bob.push(format!("alice: {message}"));
    }
    bob
}
#[allow(dead_code)]
pub fn main() {}
