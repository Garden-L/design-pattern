pub fn render(items: &[(&str, usize)]) -> Vec<String> {
    let owned: Vec<_> = items
        .iter()
        .map(|(color, position)| (color.to_string(), *position))
        .collect();
    owned
        .iter()
        .map(|(color, position)| format!("{color}@{position}"))
        .collect()
}
