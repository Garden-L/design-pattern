use concurrency_async::actor::ActorError;
pub fn run(initial: u64, increments: &[u64]) -> Result<u64, ActorError> {
    let actor = concurrency_async::actor::CounterActor::start(initial);
    let handle = actor.handle();
    for amount in increments {
        handle.add(*amount)?;
    }
    Ok(actor.shutdown())
}
