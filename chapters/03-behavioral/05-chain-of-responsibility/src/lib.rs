//! Each handler may pass, answer, or reject a request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request(pub String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainError {
    /// Continue to the next handler; the chain never returns Pass on exhaustion.
    Continue,
    Rejected,
    Unhandled,
}

pub trait Handler {
    fn handle(&self, request: &Request) -> Result<Response, ChainError>;
}

#[derive(Default)]
pub struct EventManager {
    handlers: Vec<Box<dyn Handler>>,
}

impl EventManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, handler: impl Handler + 'static) {
        self.handlers.push(Box::new(handler));
    }
    /// Stops at the first response or non-Pass error; exhaustion is Unhandled.
    pub fn handle(&self, request: &Request) -> Result<Response, ChainError> {
        for handler in &self.handlers {
            match handler.handle(request) {
                Err(ChainError::Continue) => continue,
                answer => return answer,
            }
        }
        Err(ChainError::Unhandled)
    }
}
