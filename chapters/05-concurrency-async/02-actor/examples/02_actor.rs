use actor::ActorError;
pub fn run(initial: u64, increments: &[u64]) -> Result<u64, ActorError> {
    let actor = actor::CounterActor::start(initial);
    let handle = actor.handle();
    for amount in increments {
        handle.add(*amount)?;
    }
    Ok(actor.shutdown())
}
#[allow(dead_code)]
pub fn main() {}
