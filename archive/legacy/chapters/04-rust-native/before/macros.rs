pub fn row(name: &str, topic: &str) -> Vec<(String, String)> {
    vec![
        ("name".to_string(), name.to_string()),
        ("topic".to_string(), topic.to_string()),
    ]
}
