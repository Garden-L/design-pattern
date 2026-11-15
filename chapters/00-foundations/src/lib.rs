//! Compatibility facade for the foundation lesson crate.

pub use design_foundations as design;

pub fn summarize_items(items: &[&str]) -> String {
    if items.is_empty() {
        return String::from("No items.");
    }

    let mut summary = String::from("Items: ");
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            summary.push_str(", ");
        }
        summary.push_str(item);
    }
    summary
}
