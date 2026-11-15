pub fn row(name: &str, topic: &str) -> Vec<(String, String)> {
    macros::string_pairs!("name" => name, "topic" => topic)
}
#[allow(dead_code)]
pub fn main() {}
