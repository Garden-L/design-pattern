pub fn notify(message: &str) -> Vec<String> {
    let mut sent = Vec::new();
    sent.push(format!("EMAIL: {message}"));
    sent
}
#[allow(dead_code)]
pub fn main() {}
