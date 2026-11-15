use foundations::summarize_items;

fn summarize_items_before(items: &[&str]) -> String {
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

fn assert_summary_examples(summarize: fn(&[&str]) -> String) {
    assert_eq!(summarize(&[]), "No items.");
    assert_eq!(summarize(&["ownership"]), "Items: ownership");
    assert_eq!(
        summarize(&["ownership", "borrowing", "traits"]),
        "Items: ownership, borrowing, traits"
    );
}

#[test]
fn before_and_after_implementations_follow_the_same_contract() {
    assert_summary_examples(summarize_items_before);
    assert_summary_examples(summarize_items);
}
