pub fn lengths(input: &[&str]) -> Vec<usize> {
    input
        .iter()
        .map(|s| {
            let buffer = s.as_bytes().to_vec();
            buffer.len()
        })
        .collect()
}
#[allow(dead_code)]
pub fn main() {}
