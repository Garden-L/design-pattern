// A second pricing rule adds another branch at every quote call site.
pub fn quote(subtotal: i64, discounted: bool) -> i64 {
    if discounted {
        subtotal - subtotal / 10
    } else {
        subtotal
    }
}
#[allow(dead_code)]
pub fn main() {}
