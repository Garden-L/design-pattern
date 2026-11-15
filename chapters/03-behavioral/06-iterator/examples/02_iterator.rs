use iterator::Cursor;

pub fn collect(items: &[i32]) -> Vec<i32> {
    Cursor::new(items).copied().collect()
}
#[allow(dead_code)]
pub fn main() {}
