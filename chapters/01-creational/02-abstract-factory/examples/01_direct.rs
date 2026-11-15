pub fn screen(dark: bool) -> (String, String) {
    let button = if dark { "dark button" } else { "light button" };
    let checkbox = if dark {
        "dark checkbox"
    } else {
        "light checkbox"
    };
    (button.into(), checkbox.into())
}
#[allow(dead_code)]
pub fn main() {}
