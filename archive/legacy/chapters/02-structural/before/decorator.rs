use structural::decorator::ServiceError;

pub fn execute(input: &str) -> (Result<String, ServiceError>, Vec<String>) {
    let mut log = Vec::new();
    log.push(format!("before:{input}"));
    let result = if input.is_empty() {
        Err(ServiceError::EmptyInput)
    } else {
        Ok(input.to_uppercase())
    };
    log.push(if result.is_ok() {
        "after:ok".into()
    } else {
        "after:error".into()
    });
    (result, log)
}
