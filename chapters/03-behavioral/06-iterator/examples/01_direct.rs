// The caller owns the index and the exhaustion check.
pub fn collect(items: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut index = 0;
    while index < items.len() {
        result.push(items[index]);
        index += 1;
    }
    result
}
#[allow(dead_code)]
pub fn main() {}
