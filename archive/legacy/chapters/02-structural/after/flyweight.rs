use structural::flyweight::StylePool;
pub fn render(items: &[(&str, usize)]) -> Vec<String> {
    let mut pool = StylePool::default();
    items
        .iter()
        .map(|(color, position)| pool.glyph(color, *position).render())
        .collect()
}
