pub fn describe(name: &str, item: &str) -> String {
    let local_name = name.to_owned();
    format!("{local_name}: {item}")
}
#[allow(dead_code)]
pub fn main() {}
