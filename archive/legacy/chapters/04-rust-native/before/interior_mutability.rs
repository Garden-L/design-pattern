pub fn run(entries: &[&str]) -> Vec<String> {
    let mut log = Vec::new();
    for entry in entries {
        log.push((*entry).to_owned());
    }
    log
}
