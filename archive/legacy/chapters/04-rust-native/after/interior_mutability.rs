use rust_native::interior_mutability::AuditLog;

pub fn run(entries: &[&str]) -> Vec<String> {
    let log = AuditLog::new();
    for entry in entries {
        log.record(entry);
    }
    log.entries()
}
