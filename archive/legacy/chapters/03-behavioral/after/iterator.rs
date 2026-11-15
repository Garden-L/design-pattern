use behavioral::iterator::Cursor;

pub fn collect(items: &[i32]) -> Vec<i32> {
    Cursor::new(items).copied().collect()
}
