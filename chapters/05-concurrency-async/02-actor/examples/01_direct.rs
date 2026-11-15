use actor::ActorError;
// One owner can mutate directly; sharing this state changes the design.
pub fn run(initial: u64, increments: &[u64]) -> Result<u64, ActorError> {
    let mut value = initial;
    for amount in increments {
        value = value.checked_add(*amount).ok_or(ActorError::Overflow)?;
    }
    Ok(value)
}
#[allow(dead_code)]
pub fn main() {}
