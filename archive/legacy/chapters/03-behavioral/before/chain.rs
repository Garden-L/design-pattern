use behavioral::chain::ChainError;

// Authorization and lookup are fixed in one branching function.
pub fn run(key: &str) -> Result<String, ChainError> {
    if key == "private" {
        return Err(ChainError::Rejected);
    }
    if key == "public" {
        return Ok("public data".into());
    }
    Err(ChainError::Unhandled)
}
