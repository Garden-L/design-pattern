// A second caller would need to duplicate this formatting rule.
pub fn run(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
        .to_lowercase()
}
