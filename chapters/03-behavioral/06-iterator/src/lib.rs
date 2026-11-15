//! A borrowed cursor yields each item once, then remains exhausted.
pub struct Cursor<'a, T> {
    items: &'a [T],
    position: usize,
}
impl<'a, T> Cursor<'a, T> {
    pub fn new(items: &'a [T]) -> Self {
        Self { items, position: 0 }
    }
}
impl<'a, T> Iterator for Cursor<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.items.get(self.position)?;
        self.position += 1;
        Some(item)
    }
}
impl<T> std::iter::FusedIterator for Cursor<'_, T> {}
