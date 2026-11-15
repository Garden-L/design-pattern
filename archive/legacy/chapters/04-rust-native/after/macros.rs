pub fn row(name: &str, topic: &str) -> Vec<(String, String)> {
    rust_native::string_pairs!("name" => name, "topic" => topic)
}
