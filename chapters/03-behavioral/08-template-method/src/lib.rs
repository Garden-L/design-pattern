#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Empty;
pub trait Pipeline {
    fn transform(&self, input: &str) -> String;
    fn run(&self, input: &str) -> Result<String, Empty> {
        let input = input.trim();
        if input.is_empty() {
            return Err(Empty);
        }
        Ok(self.transform(input))
    }
}
pub struct Upper;
pub struct Lower;
impl Pipeline for Upper {
    fn transform(&self, input: &str) -> String {
        input.to_uppercase()
    }
}
impl Pipeline for Lower {
    fn transform(&self, input: &str) -> String {
        input.to_lowercase()
    }
}
