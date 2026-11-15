//! Synchronous validation and delivery, with actor-owned processing metrics.
use behavioral::{
    chain::{EventManager, ChainError, Handler, Request, Response},
    command::{Add, Command, CommandError, Context},
    observer::{Event, EventBus, SubscriptionId},
};
use concurrency_async::actor::{ActorError, CounterActor, CounterHandle};
use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Eq)]
pub enum PipelineError {
    Chain(ChainError),
    Command(CommandError),
    Actor(ActorError),
}
impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Chain(error) => write!(f, "request routing failed: {error:?}"),
            Self::Command(error) => write!(f, "command failed: {error:?}"),
            Self::Actor(error) => write!(f, "metrics failed: {error}"),
        }
    }
}
impl Error for PipelineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Actor(error) => Some(error),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Summary {
    pub balance: i32,
    pub processed: u64,
}

struct RejectEmpty;
impl Handler for RejectEmpty {
    fn handle(&self, request: &Request) -> Result<Response, ChainError> {
        if request.0.trim().is_empty() {
            Err(ChainError::Rejected)
        } else {
            Err(ChainError::Continue)
        }
    }
}
struct RouteAdd;
impl Handler for RouteAdd {
    fn handle(&self, request: &Request) -> Result<Response, ChainError> {
        let mut parts = request.0.split_whitespace();
        if parts.next() != Some("add") {
            return Err(ChainError::Continue);
        }
        let amount = parts
            .next()
            .ok_or(ChainError::Rejected)?
            .parse::<i32>()
            .map_err(|_| ChainError::Rejected)?;
        if parts.next().is_some() {
            return Err(ChainError::Rejected);
        }
        Ok(Response(amount.to_string()))
    }
}

/// Read-only view: clients cannot mutate the actor's accepted-event count.
pub struct Metrics(CounterHandle);
impl Metrics {
    pub fn value(&self) -> Result<u64, ActorError> {
        self.0.value()
    }
}

pub struct EventPipeline {
    chain: EventManager,
    context: Context,
    bus: EventBus,
    actor: CounterActor,
}
impl Default for EventPipeline {
    fn default() -> Self {
        Self::new()
    }
}
impl EventPipeline {
    pub fn new() -> Self {
        let mut chain = EventManager::new();
        chain.push(RejectEmpty);
        chain.push(RouteAdd);
        Self {
            chain,
            context: Context::default(),
            bus: EventBus::new(),
            actor: CounterActor::start(0),
        }
    }
    pub fn subscribe(&self, callback: impl Fn(&Event) + 'static) -> SubscriptionId {
        self.bus.subscribe(callback)
    }
    pub fn metrics(&self) -> Metrics {
        Metrics(self.actor.handle())
    }
    /// Rejections and arithmetic errors have no effects. Observer panics propagate.
    pub fn process(&mut self, request: &str) -> Result<i32, PipelineError> {
        let response = self
            .chain
            .handle(&Request(request.into()))
            .map_err(PipelineError::Chain)?;
        let amount = response
            .0
            .parse()
            .map_err(|_| PipelineError::Chain(ChainError::Rejected))?;
        let mut command = Add::new(amount);
        command
            .execute(&mut self.context)
            .map_err(PipelineError::Command)?;
        if let Err(error) = self.actor.handle().add(1) {
            command
                .undo(&mut self.context)
                .map_err(PipelineError::Command)?;
            return Err(PipelineError::Actor(error));
        }
        self.bus
            .publish(Event(format!("balance={}", self.context.balance)));
        Ok(self.context.balance)
    }
    /// Joins the actor even when clients retain a metrics view.
    pub fn shutdown(self) -> Summary {
        Summary {
            balance: self.context.balance,
            processed: self.actor.shutdown(),
        }
    }
}
