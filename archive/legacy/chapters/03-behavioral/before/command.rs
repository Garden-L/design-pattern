use behavioral::command::CommandError;

// Execute and reversal are coupled by convention, not represented as a value.
pub fn run(start: i32, amount: i32) -> Result<(i32, i32), CommandError> {
    let current = start.checked_add(amount).ok_or(CommandError::Overflow)?;
    let restored = current.checked_sub(amount).ok_or(CommandError::Overflow)?;
    Ok((current, restored))
}
