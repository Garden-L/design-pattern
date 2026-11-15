pub fn render() -> String {
    let first = "A";
    let nested = ["B", "C"].join("");
    format!("{first}{nested}")
}
