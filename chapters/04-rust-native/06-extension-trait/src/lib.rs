//! Add a local convenience method to `str` without wrapping it.
pub trait SlugExt {
    fn slug(&self) -> String;
}
impl SlugExt for str {
    fn slug(&self) -> String {
        self.split_whitespace()
            .collect::<Vec<_>>()
            .join("-")
            .to_lowercase()
    }
}
